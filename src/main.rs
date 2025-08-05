mod models;
mod weather_service;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use dotenvy::dotenv;
use serde::Deserialize;
use std::{env, sync::Arc};
use tera::{Context, Tera};
use tower_http::services::ServeDir;
use weather_service::WeatherService;

// AppState to hold our Tera instance and weather service
#[derive(Clone)]
pub struct AppState {
    pub tera: Arc<Tera>,
    pub weather_service: Arc<WeatherService>,
}

#[derive(Deserialize)]
pub struct WeatherQuery {
    location: Option<String>,
    days: Option<u8>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    // Initialize Tera with templates
    let mut tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };
    
    // Enable auto-reload for development
    tera.autoescape_on(vec![".html", ".htm"]);
    
    let api_key = env::var("WEATHER_API_KEY").expect("WEATHER_API_KEY not set");
    let weather_service = WeatherService::new(api_key);
    
    let app_state = AppState {
        tera: Arc::new(tera),
        weather_service: Arc::new(weather_service),
    };

    // Build our application with routes
    let app = Router::new()
        .route("/", get(index))
        .route("/weather", get(get_weather))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state);

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🌤️  Weather app running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let context = Context::new();
    
    match state.tera.render("index.html", &context) {
        Ok(html) => Html(html),
        Err(e) => {
            eprintln!("Template error: {}", e);
            Html(format!("<h1>Template Error</h1><p>{}</p>", e))
        }
    }
}

async fn get_weather(
    State(state): State<AppState>,
    Query(params): Query<WeatherQuery>,
) -> impl IntoResponse {
    let location = params.location.unwrap_or_else(|| "India".to_string());
    let days = params.days.unwrap_or(1).clamp(1, 14);
    
    match state.weather_service.get_weather(&location, days).await {
        Ok(weather_data) => {
            let mut context = Context::new();
            context.insert("weather", &weather_data);
            context.insert("location", &location);
            context.insert("days", &days);
            
            match state.tera.render("weather.html", &context) {
                Ok(html) => (StatusCode::OK, Html(html)).into_response(),
                Err(e) => {
                    eprintln!("Template error: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Html(format!("<h1>Template Error</h1><p>{}</p>", e)),
                    )
                        .into_response()
                }
            }
        }
        Err(e) => {
            let mut context = Context::new();
            context.insert("error", &format!("Failed to fetch weather data: {}", e));
            context.insert("location", &location);
            
            match state.tera.render("error.html", &context) {
                Ok(html) => (StatusCode::BAD_REQUEST, Html(html)).into_response(),
                Err(template_err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(format!("<h1>Error</h1><p>{}</p>", template_err)),
                )
                    .into_response(),
            }
        }
    }
}
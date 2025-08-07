mod models;

use axum::{
    extract::{Form, Query, State},
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Router,
};

use chrono::{Duration, Local, TimeZone};
use dotenvy::dotenv;
use models::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::env;
use tera::{Context, Tera};
use tower_http::services::ServeDir;
use tracing_subscriber;

#[derive(Deserialize)]
struct WeatherForm {
    location: String,
    days: Option<u8>,
}

#[derive(Deserialize)]
struct WeatherQuery {
    location: Option<String>,
    days: Option<u8>,
}

#[derive(Serialize)]
struct TemplateForcastItem {
    dt: i64,
    main: Main,
    weather: Vec<Condition>,
    pop: Option<f64>,
    pop_percentage: u8,
    formatted_time: String, // Add formatted time
    formatted_date: String, // Add formatted date
}

struct AppState {
    tera: Tera,
    client: Client,
    api_key: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let api_key = env::var("WEATHER_API_KEY").expect("WEATHER_API_KEY not set");
    
    let mut tera = Tera::new("templates/**/*").expect("Failed to parse templates");
    tera.autoescape_on(vec![".html", ".htm"]);
    
    let app_state = AppState {
        tera,
        client: Client::new(),
        api_key,
    };

    let app = Router::new()
        .route("/", get(home_page))
        .route("/weather", post(get_weather))
        .route("/weather", get(get_weather_query))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(std::sync::Arc::new(app_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("🌤️  Weather app running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn home_page(
    State(app_state): State<std::sync::Arc<AppState>>,
) -> Result<Html<String>, StatusCode> {
    let mut context = Context::new();
    context.insert("title", "Weather Forecast");

    match app_state.tera.render("index.html", &context) {
        Ok(html) => Ok(Html(html)),
        Err(e) => {
            eprintln!("Home page template error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_weather_query(
    State(app_state): State<std::sync::Arc<AppState>>,
    Query(params): Query<WeatherQuery>,
) -> Result<Html<String>, StatusCode> {
    let location = params.location.unwrap_or_else(|| "Delhi".to_string());
    let days = params.days.unwrap_or(1).clamp(1, 5);
    
    fetch_and_render_weather(&app_state, &location, days).await
}

async fn get_weather(
    State(app_state): State<std::sync::Arc<AppState>>,
    Form(form): Form<WeatherForm>,
) -> Result<Html<String>, StatusCode> {
    let days = form.days.unwrap_or(1).clamp(1, 5);
    fetch_and_render_weather(&app_state, &form.location, days).await
}

async fn fetch_and_render_weather(
    app_state: &AppState,
    location: &str,
    days: u8,
) -> Result<Html<String>, StatusCode> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?q={}&appid={}&units=metric",
        location, app_state.api_key
    );

    let response = match app_state.client.get(&url).send().await {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("API request error: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if !response.status().is_success() {
        eprintln!("API response error: {}", response.status());
        return render_error(&app_state.tera, "Location not found or API error");
    }

    let weather: Weather = match response.json().await {
        Ok(w) => w,
        Err(e) => {
            eprintln!("JSON parsing error: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let today = Local::now().date_naive();
    let cutoff = today + Duration::days(days.into());
    let mut grouped: BTreeMap<String, Vec<TemplateForcastItem>> = BTreeMap::new();

    for item in &weather.list {
        let dt = Local.timestamp_opt(item.dt, 0).unwrap();
        let date = dt.date_naive();
        if date >= today && date < cutoff {
            let date_key = date.format("%Y-%m-%d").to_string();
            let template_item = TemplateForcastItem {
                dt: item.dt,
                main: item.main.clone(),
                weather: item.weather.clone(),
                pop: item.pop,
                pop_percentage: (item.pop.unwrap_or(0.0) * 100.0).round() as u8,
                formatted_time: dt.format("%H:%M").to_string(),
                formatted_date: date.format("%Y-%m-%d").to_string(),
            };
            grouped.entry(date_key).or_insert_with(Vec::new).push(template_item);
        }
    }

    // Prepare simplified chart data
    let mut chart_data = Vec::new();
    let mut dates: Vec<String> = Vec::new();
    
    for (date_str, forecasts) in &grouped {
        if !dates.contains(date_str) {
            dates.push(date_str.clone());
        }
        
        for forecast in forecasts {
            let mut item = HashMap::new();
            item.insert("date".to_string(), forecast.formatted_date.clone());
            item.insert("time".to_string(), forecast.formatted_time.clone());
            item.insert("temp".to_string(), forecast.main.temp.round().to_string());
            item.insert("description".to_string(), forecast.weather[0].description.clone());
            item.insert("pop".to_string(), forecast.pop_percentage.to_string());
            chart_data.push(item);
        }
    }

    let mut context = Context::new();
    context.insert("title", "Weather Forecast");
    context.insert("location", &weather.city.name);
    context.insert("country", &weather.city.country);
    context.insert("days", &days);
    context.insert("grouped_forecasts", &grouped);
    context.insert("chart_data", &chart_data);
    context.insert("dates", &dates);
    context.insert("current_time", &Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

    match app_state.tera.render("weather.html", &context) {
        Ok(html) => Ok(Html(html)),
        Err(e) => {
            eprintln!("Template render error: {}", e);
            eprintln!("Context data available - location: {}, country: {}, days: {}", 
                weather.city.name, weather.city.country, days);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn render_error(tera: &Tera, message: &str) -> Result<Html<String>, StatusCode> {
    let mut context = Context::new();
    context.insert("title", "Error");
    context.insert("error_message", message);
    
    match tera.render("error.html", &context) {
        Ok(html) => Ok(Html(html)),
        Err(e) => {
            eprintln!("Error template render error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
mod models;

use chrono::{DateTime, Utc};
use colored::*;
use dotenvy::dotenv;
use models::*;
use reqwest::blocking::get;
use std::env;

fn main() {
    dotenv().ok(); // Load .env into environment

    let api_key = env::var("WEATHER_API_KEY").expect("WEATHER_API_KEY not set");
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    let location = if args.len() > 1 { &args[1] } else { "India" };
    println!("Location variable: '{}'", location);

    let days = if args.len() > 2 {
        args[2].parse::<u8>().unwrap_or(1).clamp(1, 14)
    } else {
        1
    };

    let url = format!(
        "http://api.weatherapi.com/v1/forecast.json?key={}&q={}&days={}",
        api_key, location, days
    );

    let response = get(&url).expect("Failed to fetch weather");
    if !response.status().is_success() {
        eprintln!("API returned an error: {}", response.status());
        std::process::exit(1);
    }

    let weather: Weather = response
        .json()
        .expect("Failed to parse weather data as JSON");

    println!(
        "{}: {:.0}°C, {}",
        format!("{}, {}", weather.location.name, weather.location.country).bold(),
        weather.current.temp_c,
        weather.current.condition.text
    );

    let now = Utc::now().timestamp();

    use chrono::Local;
    let today_date = Local::now().date_naive();

    let future_days: Vec<_> = weather
        .forecast
        .forecastday
        .iter()
        .filter(|day| {
            let day_date = DateTime::<Utc>::from_timestamp(day.date_epoch, 0)
                .unwrap()
                .with_timezone(&Local)
                .date_naive();

            day_date >= today_date
        })
        .into_iter()
        .take(days as usize)
        .collect();

    for (i, day) in future_days.iter().enumerate() {
    let date = &day.hour.first().unwrap().time_epoch;
    let day_date = DateTime::<Utc>::from_timestamp(*date, 0)
        .unwrap()
        .format("%Y-%m-%d");
        println!("\n📅 Forecast for {day_date}:\n");

        for hour in &day.hour {
            if i == 0 && hour.time_epoch < now {
                continue;
            }

            let dt = DateTime::<Utc>::from_timestamp(hour.time_epoch, 0).unwrap();
            let time = dt.format("%H:%M");

            let rain_chance = hour.chance_of_rain.unwrap_or(0.0);
            let output = format!(
                "{} - {:.0}°C, {:.0}%, {}",
                time,
                hour.temp_c,
                rain_chance,
                hour.condition.text
            );

            if rain_chance >= 40.0 {
                println!("{}", output.red());
            } else {
                println!("{}", output);
            }
        }
    }
    println!("\n{}: Weather data fetched successfully!", "Success".green());
    println!("Filtered days: {}", future_days.len());
    println!("Total days in forecast: {}", weather.forecast.forecastday.len());
    println!("Location: {}", weather.location.name.bold());
    println!("Current Temperature: {:.0}°C", weather.current.temp_c);
    println!("Current Condition: {}", weather.current.condition.text);
    println!("Forecast Days: {}", weather.forecast.forecastday.len());
    println!("Data fetched at: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    println!("Data fetched from: {}", weather.location.name);

}

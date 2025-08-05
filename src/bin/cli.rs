// This is the original CLI version of your weather app
use chrono::{DateTime, Local, Utc};
use colored::*;
use dotenvy::dotenv;
use reqwest::blocking::get;
use std::env;
use weather_cli::models::*;

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
    
    let today_date = Local::now().date_naive();
    
    // Filter out past days and only get future/current days
    let future_days: Vec<_> = weather
        .forecast
        .forecastday
        .iter()
        .filter(|day| {
            let day_date = DateTime::<Utc>::from_timestamp(day.date_epoch, 0)
                .unwrap()
                .with_timezone(&Local)
                .date_naive();
            day_date >= today_date  // Only include today and future days
        })
        .take(days as usize)  // Take only the requested number of days
        .collect();
    
    println!("Requested days: {}, Available future days: {}", days, future_days.len());
    
    if future_days.is_empty() {
        println!("No future forecast data available!");
        return;
    }
    
    for day in future_days.iter() {
        let day_date = DateTime::<Utc>::from_timestamp(day.date_epoch, 0)
            .unwrap()
            .with_timezone(&Local)
            .date_naive();
        
        println!("\n📅 Forecast for {}:", day_date.format("%Y-%m-%d"));
        
        // Check if this is today's date
        let is_today = day_date == today_date;
        let mut hours_shown = 0;
        
        for hour in &day.hour {
            // For today, only skip hours that are more than 1 hour in the past
            // This gives some flexibility and shows recent hours
            if is_today {
                let hour_time = DateTime::<Utc>::from_timestamp(hour.time_epoch, 0)
                    .unwrap()
                    .with_timezone(&Local);
                let current_time = Local::now();
                
                // Skip if hour is more than 1 hour in the past
                if hour_time < current_time - chrono::Duration::hours(1) {
                    continue;
                }
            }
            
            let dt = DateTime::<Utc>::from_timestamp(hour.time_epoch, 0)
                .unwrap()
                .with_timezone(&Local);
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
            hours_shown += 1;
        }
        
        // If no hours shown for today, show at least the next few hours
        if is_today && hours_shown == 0 {
            println!("No future hours available for today. Showing last few hours:");
            for hour in day.hour.iter().rev().take(3).rev() {
                let dt = DateTime::<Utc>::from_timestamp(hour.time_epoch, 0)
                    .unwrap()
                    .with_timezone(&Local);
                let time = dt.format("%H:%M");
                let rain_chance = hour.chance_of_rain.unwrap_or(0.0);
                
                let output = format!(
                    "{} - {:.0}°C, {:.0}%, {} (past)",
                    time,
                    hour.temp_c,
                    rain_chance,
                    hour.condition.text
                );
                
                if rain_chance >= 40.0 {
                    println!("{}", output.red().dimmed());
                } else {
                    println!("{}", output.dimmed());
                }
            }
        }
    }
    
    println!("\n{}: Weather data fetched successfully!", "Success".green());
    println!("Requested days: {}", days);
    println!("Available future days: {}", future_days.len());
    println!("Total days in API response: {}", weather.forecast.forecastday.len());
    println!("Location: {}", weather.location.name.bold());
    println!("Current Temperature: {:.0}°C", weather.current.temp_c);
    println!("Current Condition: {}", weather.current.condition.text);
    println!("Data fetched at: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
    
    // Warning if requested more days than available
    if days as usize > future_days.len() {
        println!("\n⚠️  Warning: You requested {} days, but only {} future days are available.", 
                 days, future_days.len());
        println!("Note: Free WeatherAPI.com accounts typically provide only 3 days of forecast data.");
    }
}
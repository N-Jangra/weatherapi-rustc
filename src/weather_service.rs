use crate::models::*;
use chrono::{DateTime, Local, Utc};
use reqwest;
use serde::Serialize;

#[derive(Clone)]
pub struct WeatherService {
    api_key: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
pub struct WeatherData {
    pub location: LocationInfo,
    pub current: CurrentWeather,
    pub forecast_days: Vec<ForecastDayData>,
    pub total_days: usize,
    pub requested_days: u8,
}

#[derive(Serialize)]
pub struct LocationInfo {
    pub name: String,
    pub country: String,
}

#[derive(Serialize)]
pub struct CurrentWeather {
    pub temp_c: f64,
    pub condition: String,
}

#[derive(Serialize)]
pub struct ForecastDayData {
    pub date: String,
    pub date_formatted: String,
    pub is_today: bool,
    pub hours: Vec<HourInfo>,
}

#[derive(Serialize)]
pub struct HourInfo {
    pub time: String,
    pub temp_c: f64,
    pub chance_of_rain: f64,
    pub condition: String,
    pub is_past: bool,
    pub is_high_rain: bool,
}

impl WeatherService {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_weather(&self, location: &str, days: u8) -> Result<WeatherData, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "http://api.weatherapi.com/v1/forecast.json?key={}&q={}&days={}",
            self.api_key, location, days
        );

        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(format!("API returned an error: {}", response.status()).into());
        }

        let weather: Weather = response.json().await?;
        
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
                day_date >= today_date
            })
            .take(days as usize)
            .collect();

        let mut forecast_days = Vec::new();

        for day in future_days {
            let day_date = DateTime::<Utc>::from_timestamp(day.date_epoch, 0)
                .unwrap()
                .with_timezone(&Local)
                .date_naive();

            let is_today = day_date == today_date;
            let mut hours = Vec::new();
            let current_time = Local::now();

            for hour in &day.hour {
                let hour_time = DateTime::<Utc>::from_timestamp(hour.time_epoch, 0)
                    .unwrap()
                    .with_timezone(&Local);

                // For today, skip hours that are more than 1 hour in the past
                let is_past = if is_today {
                    hour_time < current_time - chrono::Duration::hours(1)
                } else {
                    false
                };

                // Skip past hours for today, but include all hours for future days
                if is_today && is_past {
                    continue;
                }

                let rain_chance = hour.chance_of_rain.unwrap_or(0.0);
                
                hours.push(HourInfo {
                    time: hour_time.format("%H:%M").to_string(),
                    temp_c: hour.temp_c,
                    chance_of_rain: rain_chance,
                    condition: hour.condition.text.clone(),
                    is_past,
                    is_high_rain: rain_chance >= 40.0,
                });
            }

            // If no future hours for today, add the last few hours as past data
            if is_today && hours.is_empty() {
                for hour in day.hour.iter().rev().take(3).rev() {
                    let hour_time = DateTime::<Utc>::from_timestamp(hour.time_epoch, 0)
                        .unwrap()
                        .with_timezone(&Local);
                    let rain_chance = hour.chance_of_rain.unwrap_or(0.0);
                    
                    hours.push(HourInfo {
                        time: hour_time.format("%H:%M").to_string(),
                        temp_c: hour.temp_c,
                        chance_of_rain: rain_chance,
                        condition: format!("{} (past)", hour.condition.text),
                        is_past: true,
                        is_high_rain: rain_chance >= 40.0,
                    });
                }
            }

            forecast_days.push(ForecastDayData {
                date: day_date.format("%Y-%m-%d").to_string(),
                date_formatted: day_date.format("%B %d, %Y").to_string(),
                is_today,
                hours,
            });
        }

        Ok(WeatherData {
            location: LocationInfo {
                name: weather.location.name,
                country: weather.location.country,
            },
            current: CurrentWeather {
                temp_c: weather.current.temp_c,
                condition: weather.current.condition.text,
            },
            forecast_days,
            total_days: weather.forecast.forecastday.len(),
            requested_days: days,
        })
    }
}
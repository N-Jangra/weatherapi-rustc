use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Weather {
    pub list: Vec<ForecastItem>,
    pub city: City,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastItem {
    pub dt: i64,
    pub main: Main,
    pub weather: Vec<Condition>,
    pub pop: Option<f64>, // Probability of precipitation
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Main {
    pub temp: f64,
    pub feels_like: Option<f64>,
    pub humidity: Option<u8>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Condition {
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct City {
    pub name: String,
    pub country: String,
}
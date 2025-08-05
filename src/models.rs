use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub location: Location,
    pub current: Current,
    pub forecast: Forecast,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    pub name: String,
    pub country: String,
}

#[derive(Deserialize, Debug)]
pub struct Current {
    pub temp_c: f64,
    pub condition: Condition,
}

#[derive(Deserialize, Debug)]
pub struct Forecast {
    pub forecastday: Vec<ForecastDay>,
}

#[derive(Debug, Deserialize)]
pub struct ForecastDay {
    pub date_epoch: i64, 
    pub hour: Vec<HourData>,
}

#[derive(Deserialize, Debug)]
pub struct HourData {
    pub time_epoch: i64,
    pub temp_c: f64,
    pub chance_of_rain: Option<f64>,
    pub condition: Condition,
}

#[derive(Deserialize, Debug)]
pub struct Condition {
    pub text: String,
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Response {
    pub coord: Coord,
    pub main: Weather,
    pub name: String,
    pub timezone: i64,
    pub visibility: i64,
    pub wind: Wind,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Coord {
    lat: f64,
    lon: f64,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Weather {
    pub feels_like: f64,
    pub grnd_level: i64,
    pub humidity: i64,
    pub pressure: i64,
    pub sea_level: i64,
    pub temp: f64,
    pub temp_max: f64,
    pub temp_min: f64,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Wind {
    pub deg: i64,
    pub speed: f64,
}

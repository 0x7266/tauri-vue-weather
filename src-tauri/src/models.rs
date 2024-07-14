use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Geo {
    pub coord: Coord,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Coord {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Data {
    pub city: City,
    pub list: Vec<Day>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct City {
    pub coord: Coord,
    pub country: String,
    pub id: u64,
    pub name: String,
    pub population: u64,
    pub sunrise: u64,
    pub sunset: u64,
    pub timezone: i64,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Day {
    pub dt_txt: String,
    pub main: Main,
    pub weather: Vec<Weather>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Main {
    pub feels_like: f64,
    pub humidity: u64,
    pub temp: f64,
    pub temp_max: f64,
    pub temp_min: f64,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Weather {
    pub description: String,
    pub icon: String,
    pub id: u64,
    pub main: String,
}

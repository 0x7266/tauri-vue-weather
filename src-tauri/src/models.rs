use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Data {
    pub location: Location,
    pub current: Current,
    pub forecast: Forecast,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Autocomplete {
    pub country: String,
    pub id: u64,
    pub lat: f32,
    pub lon: f32,
    pub name: String,
    pub region: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f32,
    pub lon: f32,
    pub tz_id: String,
    pub localtime_epoch: u64,
    pub localtime: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Current {
    pub condition: Condition,
    pub feelslike_c: f32,
    pub feelslike_f: f32,
    pub humidity: u64,
    pub is_day: bool,
    pub last_updated: String,
    pub last_updated_epoch: u64,
    pub precip_in: u64,
    pub precip_mm: u64,
    pub pressure_in: f32,
    pub pressure_mb: u32,
    pub temp_c: f32,
    pub temp_f: f32,
    pub wind_degree: f32,
    pub wind_dir: String,
    pub wind_kph: f64,
    pub wind_mph: f64,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Condition {
    pub code: u32,
    pub icon: String,
    pub text: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Forecast {
    pub forecastday: Vec<Forecastday>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Forecastday {
    pub date: String,
    pub date_epoch: String,
    pub day: Day,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Day {
    pub avghumidity: u32,
    pub avgtemp_c: f32,
    pub avgtemp_f: f32,
    pub avgvis_km: u32,
    pub avgvis_miles: u32,
    pub condition: Condition,
    pub daily_chance_of_rain: u8,
    pub daily_chance_of_snow: u8,
    pub daily_will_it_rain: u8,
    pub daily_will_it_snow: u8,
    pub maxtemp_c: f32,
    pub maxtemp_f: f32,
    pub maxwind_kph: f32,
    pub maxwind_mph: f32,
    pub mintemp_c: f32,
    pub mintemp_f: f32,
    pub totalprecip_in: f32,
    pub totalprecip_mm: f32,
    pub totalsnow_cm: f32,
}

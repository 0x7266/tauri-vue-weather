// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use models::{Coord, Data, Geo};

mod models;

#[tauri::command]
fn get_data(city: &str) -> Data {
    let geo = reqwest::blocking::get(format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city,
        env::var("LAT_LON_KEY").expect("Error reading the API key")
    ));
    match geo {
        Ok(r) => {
            let Geo {
                coord: Coord { lat, lon },
            } = r
                .json::<Geo>()
                .expect("Error while parsing response to JSON");

            let data = reqwest::blocking::get::<String>(format!(
                "https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&cnt=7&appid={}",
                lat,
                lon,
                env::var("FORECAST_KEY").expect("Error while reading the forecast key")
            ))
            .unwrap()
            .json::<Data>()
            .unwrap();

            data
        }
        Err(_e) => todo!(),
    }
}

fn main() {
    dotenvy::dotenv().expect(".env file not found");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

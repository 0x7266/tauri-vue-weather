// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use models::Response;

mod models;

#[tauri::command]
fn get_data(city: &str) -> Response {
    reqwest::blocking::get(format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city,
        env::var("WEATHER_API_KEY").expect("Error reading the API key")
    ))
    .unwrap()
    .json::<Response>()
    .unwrap()
}

fn main() {
    dotenvy::dotenv().expect(".env file not found");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

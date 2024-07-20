// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

mod models;
#[tauri::command]
fn get_autocomplete(query: &str) -> serde_json::Value {
    let autocomplete = reqwest::blocking::get(format!(
        "https://api.weatherapi.com/v1/search.json?key={}&q={}",
        env::var("AUTOCOMPLETE_KEY").expect("Error reading the autocomplete key"),
        query
    ))
    .unwrap()
    .json::<serde_json::Value>()
    .expect("Error while parsing the AUTOCOMPLETE");
    println!("{autocomplete:#}");
    autocomplete
}

#[tauri::command]
fn get_data(query: &str, days: Option<u16>) -> serde_json::Value {
    let data = reqwest::blocking::get(format!(
        "https://api.weatherapi.com/v1/forecast.json?key={}&q={}&days={}",
        env::var("FORECAST_KEY").expect("Error reading the API key"),
        query,
        days.unwrap_or(7)
    ))
    .unwrap()
    .json::<serde_json::Value>()
    .expect("Error while parsing the FORECAST DATA");
    println!("{data:#}");

    data
}

fn main() {
    dotenvy::dotenv().expect(".env file not found");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_data, get_autocomplete])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

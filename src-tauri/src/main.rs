// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod handlers;
mod state;
mod db;

use state::State;
use handlers::example;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .manage(State::init().await.unwrap())
        .invoke_handler(tauri::generate_handler![
            example
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

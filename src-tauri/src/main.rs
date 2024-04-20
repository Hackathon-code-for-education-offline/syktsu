// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod handlers;
mod state;

use handlers::example;
use state::State;

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .manage(State::init().await.unwrap())
        .invoke_handler(tauri::generate_handler![example])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

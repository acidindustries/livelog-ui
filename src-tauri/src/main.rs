// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use payloads::Payload;
use std::sync::mpsc;

mod commands;
mod db;
mod endpoints;
mod handlers;
mod models;
mod payloads;
mod server;
mod templating;

fn main() {
    let (input_tx, input_rx) = mpsc::channel::<Payload>();
    tauri::Builder::default()
        .manage(db::Db(tauri::async_runtime::block_on(db::init())))
        .invoke_handler(tauri::generate_handler![
            commands::logs,
            commands::refresh_logs
        ])
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move { server::init(input_tx).await });
            tauri::async_runtime::spawn(async move {
                while let Ok(payload) = input_rx.recv() {
                    handlers::refresh(payload, &app_handle).await;
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

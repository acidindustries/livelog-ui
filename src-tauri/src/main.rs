// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use filters::Filter;
use payloads::Payload;
use std::sync::mpsc;
use tauri::async_runtime::Mutex;
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

mod commands;
mod db;
mod endpoints;
mod filters;
mod handlers;
mod models;
mod payloads;
mod server;
mod templating;

fn main() {
    let (input_tx, input_rx) = mpsc::channel::<Payload>();
    tauri::Builder::default()
        .system_tray(configure_tray())
        .on_system_tray_event(|app, event| {
            let window = app.get_window("main").unwrap();
            match event {
                SystemTrayEvent::DoubleClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    window.show().unwrap();
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
                        window.hide().unwrap();
                    }
                    "show" => {
                        window.show().unwrap();
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .manage(Mutex::new(Filter::new(None, None, None)))
        .manage(db::Db(tauri::async_runtime::block_on(db::init())))
        .invoke_handler(tauri::generate_handler![
            commands::logs,
            commands::refresh_logs,
            commands::delete_log,
            commands::clear_all,
            commands::apply_filters,
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

fn configure_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    SystemTray::new().with_menu(menu)
}

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
    // let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    // let show = CustomMenuItem::new("hide".to_string(), "Hide");
    // let hide = CustomMenuItem::new("show".to_string(), "Show");
    // let systray_menu = SystemTrayMenu::new()
    //     .add_item(show)
    //     .add_item(hide)
    //     .add_native_item(SystemTrayMenuItem::Separator)
    //     .add_item(quit);
    // let sys_tray = SystemTray::new().with_menu(systray_menu);
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        // .system_tray(sys_tray)
        // .on_system_tray_event(|app, event| match event {
        //     SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
        //         "quit" => {
        //             std::process::exit(0);
        //         }
        //         "hide" => {
        //             let window = app.get_window("main").unwrap();
        //             window.hide().unwrap();
        //         }
        //         "show" => {
        //             let window = app.get_window("main").unwrap();
        //             window.show().unwrap();
        //         }
        //         _ => {}
        //     },
        //     _ => {}
        // })
        // .plugin(tauri_plugin_dialog::init()
        .manage(db::Db(tauri::async_runtime::block_on(db::init())))
        .invoke_handler(tauri::generate_handler![
            commands::logs,
            commands::refresh_logs,
            commands::delete_log
        ])
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move { server::init(input_tx).await });
            tauri::async_runtime::spawn(async move {
                while let Ok(payload) = input_rx.recv() {
                    handlers::refresh(payload, &app).await;
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // .run(|_app_handle, event| match event {
    //     tauri::RunEvent::ExitRequested { api, .. } => {
    //         api.prevent_exit();
    //     }
    //     _ => {}
    // });
}

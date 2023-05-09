// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod tray;
mod utils;

#[tauri::command]
async fn move_window(window: tauri::Window) {
    utils::move_window(&window);
}

#[tauri::command]
async fn make_visible(window: tauri::Window) {
    window.show().unwrap();
    window.unminimize().unwrap();
    window.set_focus().unwrap();
}

fn main() {
    tauri::Builder::default()
        .system_tray(tray::get_tray())
        .on_system_tray_event(tray::tray_callback)
        .on_window_event(|event| {
            use tauri::WindowEvent::*;

            match event.event() {
                CloseRequested { api, .. } => {
                    event.window().hide().unwrap();
                    api.prevent_close();
                }
                Resized(size) => {
                    if size.width == 0 && size.height == 0 {
                        // minimized
                        event.window().emit("hide", 0).unwrap();
                    } else {
                        // unminimized
                        event.window().emit("show", 0).unwrap();
                        utils::move_window(&event.window());
                    }
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![move_window, make_visible])
        .setup(|app| {
            use tauri_plugin_positioner::{Position, WindowExt};

            let window = app.get_window("main").unwrap();
            window.move_window(Position::TopRight).unwrap();
            window.set_always_on_top(true).unwrap();

            window.emit("make_visible", 0).unwrap();

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building the context")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}

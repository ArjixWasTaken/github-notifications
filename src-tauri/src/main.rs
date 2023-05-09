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
            window.move_window(Position::BottomRight).unwrap();

            let monitor = window.current_monitor().unwrap().unwrap();

            let size = monitor.size();
            let win_size = window.outer_size().unwrap();

            let desktop_height = size.height as f64 * 0.958;

            let mut pos = window.outer_position().unwrap();
            pos.y = desktop_height as i32 - win_size.height as i32;

            window.set_position(pos).unwrap();

            window.show().unwrap();
            window.unminimize().unwrap();
            window.set_focus().unwrap();

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

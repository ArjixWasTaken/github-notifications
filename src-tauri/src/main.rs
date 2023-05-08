// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ::device_query::{DeviceQuery, DeviceState, MouseState};
use tauri::{Manager, PhysicalPosition, Position};

mod tray;

#[tauri::command]
async fn move_window(window: tauri::Window) {
    let state: DeviceState = DeviceState::new();

    let mouse = state.get_mouse();
    let (mouse_x, mouse_y) = mouse.coords;

    let monitor = window.current_monitor().unwrap().unwrap();
    let size = monitor.size();
    let win_size = window.outer_size().unwrap();

    window
        .set_position(Position::Physical(PhysicalPosition {
            x: (mouse_x) - (win_size.width / 2) as i32,
            y: mouse_y,
        }))
        .unwrap();

    window.show().unwrap();
}

fn main() {
    tauri::Builder::default()
        .system_tray(tray::get_tray())
        .on_system_tray_event(tray::tray_callback)
        .on_window_event(|event| {
            use tauri::WindowEvent::*;

            match event.event() {
                CloseRequested { api, .. } => {
                    let win_handle = event.window().hwnd().unwrap();

                    event.window().hide().unwrap();
                    api.prevent_close();
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![move_window])
        .build(tauri::generate_context!())
        .expect("error while building the context")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}

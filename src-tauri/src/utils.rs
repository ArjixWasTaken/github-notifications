use std::cmp::{max, min};

use device_query::{DeviceQuery, DeviceState};
use tauri::{PhysicalPosition, Position};

pub fn move_window(window: &tauri::Window) {
    let state: DeviceState = DeviceState::new();

    let mouse = state.get_mouse();
    let (mouse_x, mouse_y) = mouse.coords;

    let monitor = window.current_monitor().unwrap().unwrap();

    let size = monitor.size();
    let win_size = window.outer_size().unwrap();

    let desktop_height = size.height as f64 * 0.958;

    window
        .set_position(Position::Physical(PhysicalPosition {
            x: min(
                (size.width - win_size.width) as i32,
                (mouse_x) - (win_size.width / 2) as i32,
            ),
            y: max(
                0,
                min(
                    mouse_y - win_size.height as i32 - 10,
                    desktop_height as i32 - win_size.height as i32,
                ),
            ),
        }))
        .unwrap();
}

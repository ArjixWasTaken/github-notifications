use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

fn get_menu() -> SystemTrayMenu {
    SystemTrayMenu::new().add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
}

pub fn get_tray() -> SystemTray {
    SystemTray::new()
        .with_menu(get_menu())
        .with_tooltip("Github Notifications")
}

pub fn tray_callback(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            let window = app.get_window("main").unwrap();
            if window.is_visible().unwrap() && !window.is_minimized().unwrap() {
                window.minimize().unwrap();

                window.hide().unwrap();
                window.emit("hide", 0).unwrap();
            } else {
                crate::utils::move_window(&window);
                window.emit("make_visible", 0).unwrap();
                window.emit("show", 0).unwrap();
            }
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}

use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

pub fn create_system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

pub fn set_hide_title(app: &AppHandle, hide: bool) {
    let item_handle: tauri::SystemTrayMenuItemHandle = app.tray_handle().get_item("hide");
    if hide {
        item_handle.set_title("Hide").unwrap();
    } else {
        item_handle.set_title("Show").unwrap();
    }
}

pub fn handle_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a left click");
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a right request");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a double click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                let window_option = app.get_window("main");
                if window_option.is_none() {
                    let window = tauri::WindowBuilder::new(
                        app,
                        "main",
                        tauri::WindowUrl::App("index.html".into()),
                    )
                    .build()
                    .expect("failed to build window");
                    window.show().unwrap();
                    set_hide_title(app, true);
                    return;
                }

                let window = window_option.unwrap();
                if window.is_visible().unwrap() {
                    set_hide_title(app, false);
                    window.hide().unwrap();
                } else {
                    set_hide_title(app, true);
                    window.show().unwrap();
                }
            }
            _ => {}
        },
        _ => {}
    }
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tray;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("你好, {}!", name)
}

fn main() {
    tauri::Builder::default()
        .system_tray(tray::create_system_tray())
        .on_system_tray_event(|app, event| tray::handle_system_tray_event(app, event))
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
                tray::set_hide_title(app, false);
            }
            _ => {}
        });
}

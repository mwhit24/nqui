#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, SystemTray};
use tauri::{ SystemTrayMenu, SystemTrayMenuItem};


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let system_tray = create_system_tray_menu();
    let menu = create_menu();
    tauri::Builder::default()
        .system_tray(system_tray)
        .menu(menu)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // let menu = construct_menu();
    // tauri::Builder::default()
    //     .menu(menu)    
    //     .invoke_handler(tauri::generate_handler![greet])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}

fn create_menu() -> Menu {
    let quit = CustomMenuItem::new("quit".to_string(), "Exit");
    let submenu = Submenu::new("File", Menu::new().add_item(quit));
    let menu = Menu::new()
        .add_submenu(submenu)
        .add_item(CustomMenuItem::new("view", "View"));
    return menu
}

fn create_system_tray_menu() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
      .add_item(quit)
      .add_native_item(SystemTrayMenuItem::Separator)
      .add_item(hide);
    let tray = SystemTray::new().with_menu(tray_menu);
    return tray;
}

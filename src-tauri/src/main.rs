#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, Submenu, SystemTray};
use tauri::{SystemTrayMenu, SystemTrayMenuItem};

fn main() {
    let system_tray = create_system_tray_menu();
    let menu = create_menu();
    tauri::Builder::default()
        .system_tray(system_tray)
        .menu(menu)
        .invoke_handler(tauri::generate_handler![
            nqui::nqlite::get_tables,
            nqui::nqlite::fetch_related_table,
            nqui::nqlite::run_query
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_menu() -> Menu {
    let quit = CustomMenuItem::new("quit".to_string(), "Exit");
    let submenu = Submenu::new("File", Menu::new().add_item(quit));

    Menu::new()
        .add_submenu(submenu)
        .add_item(CustomMenuItem::new("view", "View"))
}

fn create_system_tray_menu() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    SystemTray::new().with_menu(tray_menu)
}

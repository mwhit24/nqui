#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use reqwest::Url;
use serde_json::Value;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, SystemTray};
use tauri::{SystemTrayMenu, SystemTrayMenuItem};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[tauri::command]
async fn get_tables() -> Result<String, Error> {
    let connection_string = "http://localhost:4001/db/query";
    let query = "SELECT name FROM sqlite_schema WHERE type = 'table' AND name NOT LIKE 'sqlite_%'";
    let client = reqwest::Client::new();
    let url = Url::parse_with_params(connection_string, &[("q", query)]).unwrap();
    let response = client.get(url).send().await.unwrap();
    Ok(response.text().await.unwrap())
}

#[tauri::command]
async fn fetch_related_table(table_name: &str) -> Result<String, Error> {
    let connection_string = "http://localhost:4001/db/query";
    let query = format!("SELECT * FROM {}", table_name);
    let client = reqwest::Client::new();
    let url = Url::parse_with_params(connection_string, &[("q", query)]).unwrap();
    let response = client.get(url).send().await.unwrap();
    Ok(response.text().await.unwrap())
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

fn main() {
    let system_tray = create_system_tray_menu();
    let menu = create_menu();
    tauri::Builder::default()
        .system_tray(system_tray)
        .menu(menu)
        .invoke_handler(tauri::generate_handler![get_tables, fetch_related_table])
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

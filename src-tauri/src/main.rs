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

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_tables() -> Result<String, Error> {
    let connection_string = "http://localhost:4001/db/query";
    let query = "SELECT name FROM sqlite_schema WHERE type = 'table' AND name NOT LIKE 'sqlite_%'";

    let client = reqwest::Client::new();

    let url = Url::parse_with_params(connection_string, &[("q", query)]).unwrap();

    let response = client.get(url).send().await.unwrap();

    Ok(response.text().await.unwrap())

    // // If we just parse from a string, we can dynamically cast any stringified json object into json, since the db will be dynamic
    // match response.status() {
    //     reqwest::StatusCode::OK => {
    //         let res: Value = serde_json::from_str(&response.text().await?)?;
    //         Ok(res)?;
    //     },
    //     reqwest::StatusCode::UNAUTHORIZED => {
    //         panic!("Error: Unauthorized!",);
    //     },
    //     e => {
    //         panic!("Uh oh! Something unexpected happened, {}", e);
    //     }
    // }
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
        .invoke_handler(tauri::generate_handler![get_tables])
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
    return menu;
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

use anyhow::Result;
use reqwest::Url;

use crate::config;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] Box<dyn std::error::Error>),
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

#[tauri::command]
pub async fn run_query(query: &str) -> Result<String, Error> {
    let config = config::Config::default();
    let mut connection_string = config.connection_string.unwrap().unwrap();
    {
        connection_string += "/db/query";
    }
    let client = reqwest::Client::new();
    let url = Url::parse_with_params(&connection_string, &[("q", query)]).unwrap();
    let response = client.get(url).send().await.unwrap();
    Ok(response.text().await.unwrap())
}

#[tauri::command]
pub async fn get_tables() -> Result<String, Error> {
    let config = config::Config::default();
    let mut connection_string = config.connection_string.unwrap().unwrap();
    {
        connection_string += "/db/query";
    }
    let query = "SELECT name FROM sqlite_schema WHERE type = 'table' AND name NOT LIKE 'sqlite_%' AND name NOT LIKE '_nqlite_%'";
    let client = reqwest::Client::new();
    let url = Url::parse_with_params(&connection_string, &[("q", query)]).unwrap();
    let response = client.get(url).send().await.unwrap();
    Ok(response.text().await.unwrap())
}

#[tauri::command]
pub async fn fetch_related_table(table_name: &str) -> Result<String, Error> {
    let config = config::Config::default();
    let mut connection_string = config.connection_string.unwrap().unwrap();
    {
        connection_string += "/db/query";
    }
    let query = format!("SELECT * FROM {}", table_name);
    let client = reqwest::Client::new();
    let url = Url::parse_with_params(&connection_string, &[("q", query)]).unwrap();
    let response = client.get(url).send().await.unwrap();
    Ok(response.text().await.unwrap())
}

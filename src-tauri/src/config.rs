use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct ConfigJson {
    connection_string: String,
}

#[derive(Debug)]
pub struct Config {
    pub connection_string: Result<Option<String>>,
    pub sequence_number: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            connection_string: Self::set_connection_string(),
            sequence_number: String::from("1"),
        }
    }

    fn read_config_file() -> Result<ConfigJson> {
        // Read the file with fs\        println!("{}", std::env::current_dir().unwrap().display());
        let config_data = fs::read_to_string("config.json")?;
        let config_json: ConfigJson = serde_json::from_str(&config_data)?;
        Ok(config_json)
    }

    fn set_connection_string() -> Result<Option<String>> {
        let config = Self::read_config_file()?;
        println!("{:?}", config.connection_string);
        Ok(Some(config.connection_string))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

use std::{fs, process::exit};

use log::error;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Ca {
    pub private_key_file: String,
    pub certificate_chain_file: String,
}

#[derive(Deserialize)]
pub struct Client {
    pub serve_from: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub ca: Ca,
    pub client: Client,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Config {
        let config_file = match fs::read_to_string(common::CONFIG_FILE) {
            Ok(t) => t,
            Err(e) => {
                error!(
                    ">>> Failed to read config file {}. Err: {e}",
                    common::CONFIG_FILE
                );
                exit(1);
            }
        };

        let config: Config = match toml::from_str(&config_file) {
            Ok(t) => t,
            Err(e) => {
                error!(
                    ">>> Failed to parse config file {}. Err: {e}",
                    common::CONFIG_FILE
                );
                exit(1);
            }
        };

        config
    }
}

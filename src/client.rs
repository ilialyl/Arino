use std::{fs::File, io::Write};

use serde::{Deserialize, Serialize};

use crate::database::cloud::get_credentials;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub has_access: bool,
}

pub fn save_config(config: &Config) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(config).expect("Error serializing");
    let mut file = File::create("user_config.json")?;
    file.write_all(json.as_bytes())?;

    Ok(())
}

pub fn load_config() -> Config {
    match std::fs::read_to_string("user_config.json") {
        Ok(s) => return serde_json::from_str(&s).expect("Error parsing user_config.json"),
        Err(_) => {
            if get_credentials().is_err() {
                return Config { has_access: false };
            } else {
                return Config { has_access: true };
            }
        }
    };
}

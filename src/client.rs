use std::{fs, fs::File, io::Write};

use serde::{Deserialize, Serialize};

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
    fs::read_to_string("user_config.json")
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or(Config { has_access: false })
}

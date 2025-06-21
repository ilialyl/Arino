pub mod cli;
pub mod client;
pub mod database;
pub mod miscellaneous;

use clap::Parser;
use once_cell::sync::Lazy;
use tokio;

use cli::commands::Cli;
use database::{
    cloud::{Database, get_credentials, has_internet_access},
    database_exists,
};

use crate::{
    client::{Config, load_config, save_config},
    database::cloud::download_database,
};

pub static CONFIG: Lazy<Config> = Lazy::new(load_config);

#[tokio::main]
async fn main() {
    if !database_exists() & has_internet_access().await {
        let config: Config;

        if get_credentials().is_err() {
            config = Config { has_access: false };
            download_database().await;
        } else {
            config = Config { has_access: true };
            match database::cloud::fetch(Database::Main).await {
                Ok(_) => {}
                Err(e) => println!("{e}"),
            }
        }

        save_config(&config).expect("Failed to save config");
    } else {
        eprintln!("Internet access is required to fetch database for first use!");
    }

    let cli = Cli::parse();
    match cli.command.execute().await {
        Ok(_) => {}
        Err(e) => eprintln!("{e}"),
    }
}

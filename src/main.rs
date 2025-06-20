pub mod cli;
pub mod database;
pub mod miscellaneous;

use clap::Parser;
use cli::commands::Cli;
use database::{
    cloud::{Database, get_credentials, has_internet_access},
    database_exists,
};

use tokio;

use crate::database::cloud::download_database;

#[tokio::main]
async fn main() {
    if !database_exists() & has_internet_access().await {
        if get_credentials().is_err() {
            download_database();
        } else {
            match database::cloud::fetch(Database::Main).await {
                Ok(_) => {}
                Err(e) => println!("{e}"),
            }
        }
    } else {
        eprintln!("Internet access is required to fetch database for first use!");
    }

    let cli = Cli::parse();
    match cli.command.execute().await {
        Ok(_) => {}
        Err(e) => eprintln!("{e}"),
    }
}

pub mod cli;
pub mod client;
pub mod database;
pub mod miscellaneous;

use clap::Parser;
use tokio;

use cli::commands::Cli;
use database::{
    cloud::{Database, has_internet_access},
    database_exists,
};

use crate::{client::has_access, database::cloud::download_database};

#[tokio::main]
async fn main() {
    let (db_exists, has_internet) = tokio::join!(
        async {
            let result = database_exists();
            result
        },
        has_internet_access(),
    );

    if !db_exists && has_internet {
        if has_access() {
            match database::cloud::fetch(Database::Main).await {
                Ok(_) => {}
                Err(e) => println!("{e}"),
            }
        } else {
            println!("You are using an offline version as you do not have access");
            download_database().await;
        }
    } else if !db_exists && !has_internet {
        eprintln!("Internet access is required to fetch database for first use!");
    }

    let cli = Cli::parse();
    match cli.command.execute().await {
        Ok(_) => {}
        Err(e) => eprintln!("{e}"),
    }
}

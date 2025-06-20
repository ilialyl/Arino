pub mod cli_operations;
pub mod database;
pub mod miscellaneous;

use clap::Parser;
use cli_operations::commands::Cli;
use database::{
    cloud::{has_internet_access, Database},
    database_exists,
};
use tokio;

#[tokio::main]
async fn main() {
    if !database_exists() {
        if has_internet_access().await {
            match database::cloud::fetch(Database::Main).await {
                Ok(_) => {}
                Err(e) => println!("{e}"),
            }
        } else {
            eprintln!("Internet access is required to fetch database for first use!");
        }
    }

    let cli = Cli::parse();
    match cli.command.execute().await {
        Ok(_) => {}
        Err(e) => eprintln!("{e}"),
    }
}

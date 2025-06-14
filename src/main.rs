pub mod database;
pub mod cli_operations;
pub mod helper;

use database::{cloud::{has_internet_access, Database}, database_exists};
use cli_operations::commands::Cli;
use tokio;
use clap::Parser;


#[tokio::main]
async fn main() {
    if !database_exists() {
        if has_internet_access().await {
            match database::cloud::fetch(Database::Main).await {
                Ok(_) => {},
                Err(e) => println!("{e}"),
            }    
        } else {
            eprintln!("Internet access is required to fetch database for first use!");
        }
    }

    let cli = Cli::parse();
    cli.command.execute().await.unwrap();    
}

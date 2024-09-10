pub mod database;
pub mod cli_operations;
pub mod helper;

use cli_operations::user_input::{self, prompt};
use database::cloud::Database;
use tokio;


#[tokio::main]
async fn main() {
    match database::cloud::fetch(Database::Main).await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{e}");
            return;
        },
    }    
    println!("----Arino----");
    loop {
        let user_input = prompt("Command");
        match user_input::match_commands(user_input) {
            Ok(_) => {},
            Err(e) => eprintln!("{e}"),
        }
    }
}

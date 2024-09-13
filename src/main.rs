pub mod database;
pub mod cli_operations;
pub mod helper;

use cli_operations::user_input::{self, match_enums, prompt};
use database::{cloud::{has_internet_access, Database}, first_start};
use tokio;


#[tokio::main]
async fn main() {
    if first_start() {
        

        if has_internet_access().await {
            match database::cloud::fetch(Database::Main).await {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("{e}");
                    return;
                },
            }    
        } else {
            eprintln!("Internet access is required to fetch database for first use!");
            return;
        }
    }
    
    println!("----Arino----");
    loop {
        let user_input = prompt("Command");
        let command_enum = match_enums(user_input);
        match user_input::match_commands(command_enum).await {
            Ok(_) => {},
            Err(e) => eprintln!("{e}"),
        }
    }
}

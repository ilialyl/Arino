pub mod database;
pub mod cli_operations;
pub mod helper;

use cli_operations::{commands::get_command_bimap, user_input::{self, to_command_enum, prompt}};
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
    
    println!("-----------------Arino-----------------");
    println!("Type \"help\" for the list of commands");
    let command_bimap = get_command_bimap();

    loop {
        let user_input = prompt("Command");
        let command_enum = to_command_enum(user_input, &command_bimap);
        match user_input::match_commands(command_enum, &command_bimap).await {
            Ok(_) => {},
            Err(e) => eprintln!("{e}"),
        }
    }
}

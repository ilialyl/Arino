pub mod database;
pub mod cli_operations;
pub mod helper;

use cli_operations::{commands::get_command_bimap, user_input::{self, to_command_enum}, commands::Command};
use database::{cloud::{has_internet_access, Database}, database_exists};
use tokio;
use std::env;


#[tokio::main]
async fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let arg_count = env::args().count();

    let command_bimap = get_command_bimap();

    if arg_count < 2 {
        match user_input::match_commands(Command::Help, &command_bimap).await {
            Ok(_) => {},
            Err(e) => println!("{e}"),
        }
        return
    }
    

    let command = args.remove(0);
    // let command_args = &env_args[1..];

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

    let command_enum = to_command_enum(command, &command_bimap);
    match user_input::match_commands(command_enum, &command_bimap).await {
        Ok(_) => {},
        Err(e) => {
            println!("{e}");
        }
    }
}

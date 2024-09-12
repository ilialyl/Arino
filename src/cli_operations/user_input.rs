use rusqlite::Result;
use database::query;
use crate::database::cloud::{fetch, has_internet_access, sync, Database};
use crate::database::{self, insert};
use crate::helper::flush;
use std::io::stdin;

pub async fn match_commands(user_input: String) -> Result<()>{
    let mut user_input = user_input.split("\"");
    let command = user_input.next().expect("No command input");

    match command.trim() {
        "new ingredient" => insert::ingredient().await,
        "add price" => insert::price().await,
        "new dish" => insert::dish().await,
        "add recipe" => insert::recipe(None).await,
        "list all dishes" => query::all_dish_names(),
        "list all ingredients" => query::all_ingredients(),
        "i have" => query::dish_by_ingredients::get_dishes(),
        "recipe of" => query::recipe_by_dish_name(),
        "fetch database" => {
            if has_internet_access().await {
                fetch(Database::Main).await.expect("Error fetching database");
            } else {
                eprintln!("Internet access is required to fetch database from cloud");
            }
            Ok(())
        },
        "sync database" => {
            if has_internet_access().await {
                sync().await.expect("Error syncing database");
            } else {
                eprintln!("Internet access is required to sync database to cloud");
            }
            Ok(())
        },
        "quit" => std::process::exit(0),
        _ => {
            eprintln!("Unknown command");
            Ok(())
        }
    }
}

pub fn separate_by(separator: &str, user_input: String) -> Vec<String>{
    let split_iter = user_input.split(separator);
    let separated_inputs_vec: Vec<String> = split_iter.map(|s| s.trim().to_string()).collect();

    separated_inputs_vec
}

pub fn prompt(prompt: &str) -> String {
    let mut user_input = String::new();
    print!("\n{}> ", prompt);
    flush();
    match stdin().read_line(&mut user_input) {
        Ok(_) => return user_input.trim().to_lowercase().to_string(),
        Err(e) => {
            eprint!("{e}");
            return user_input;
        },
    }
}
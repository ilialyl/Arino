use bimap::BiMap;
use rusqlite::Result;
use database::show;
use crate::database::cloud::{backup, fetch, has_internet_access, sync, Database};
use crate::database::{self, delete, insert, modify};
use crate::helper::flush;
use std::io::stdin;
use super::commands::Command;


pub fn to_command_enum(user_input: String, command_bimap: &BiMap<Command, String>) -> Command {
    if let Some(command_enum) = command_bimap.get_by_right(&user_input) {
        command_enum.clone()
    } else {
        Command::Unknown
    }
}

pub async fn match_commands(command_enum: Command, command_bimap: &BiMap<Command, String>) -> Result<()> {
    match command_enum {
        Command::NewIngredient => insert::ingredient().await,
        Command::AddPrice => insert::price().await,
        Command::NewDish => insert::dish().await,
        Command::AddRecipe => insert::recipe(None).await,
        Command::ListAllDishes => show::all_dish_names(),
        Command::ListAllIngredients => show::all_ingredients(),
        Command::IHave => show::dish_by_ingredients::get_dishes(),
        Command::RecipeOf => show::recipe_by_dish_name(),
        Command::DeleteIngredientFromRecipe => delete::ingredient_from_recipe().await,
        Command::DeleteDish => delete::dish().await,
        Command::DeleteIngredient => delete::ingredient().await,
        Command::FetchDatabase => {
            if has_internet_access().await {
                fetch(Database::Main).await.expect("Error fetching database");
            } else {
                eprintln!("Internet access is required to fetch database from cloud");
            }
            Ok(())
        },
        Command::SyncDatabase => {
            if has_internet_access().await {
                sync().await.expect("Error syncing database");
            } else {
                eprintln!("Internet access is required to sync database to cloud");
            }
            Ok(())
        },
        Command::BackupDatabase => {
            if has_internet_access().await {
                backup().await.expect("Error backing up database");
            } else {
                eprintln!("Internet access is required to backup database to cloud");
            }
            Ok(())
        },
        Command::Help => {
            list_all_commands(command_bimap);
            Ok(())
        },
        Command::Quit => std::process::exit(0),
        Command::Unknown => {
            eprintln!("Unknown command");
            Ok(())
        }
        Command::UpdateIngredient => {
            modify::ingredient().await
        }
    }
}

fn list_all_commands(command_bimap: &BiMap<Command, String>) {
    command_bimap.right_values().for_each(|s| println!("-- {s}"));
}

pub fn separate_by(separator: &str, user_input: String) -> Vec<String>{
    let split_iter = user_input.split(separator);
    let separated_inputs_vec: Vec<String> = split_iter.map(|s| s.trim().to_string()).collect();

    separated_inputs_vec
}

pub fn prompt(prompt: &str) -> String {
    let mut user_input = String::new();
    print!("{}> ", prompt);
    flush();
    match stdin().read_line(&mut user_input) {
        Ok(_) => return user_input.trim().to_lowercase().to_string(),
        Err(e) => {
            eprint!("{e}");
            return user_input;
        },
    }
}
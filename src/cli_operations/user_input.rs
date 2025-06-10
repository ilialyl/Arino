use bimap::BiMap;
use rusqlite::Result;
use database::show;
use crate::database::cloud::{backup, fetch, has_internet_access, push, Database};
use crate::database::{self, delete, insert, modify};
use crate::helper::flush;
use std::io::stdin;
use super::commands::Command;

// Return command enum from string
pub fn to_command_enum(user_input: String, command_bimap: &BiMap<Command, String>) -> Command {
    if let Some(command_enum) = command_bimap.get_by_right(&user_input) {
        command_enum.clone()
    } else {
        Command::Unknown
    }
}

// Match command enums to functions (defines what each command does)
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
                push().await.expect("Error syncing database");
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
        Command::UpdateDishName => {
            modify::dish_name().await
        }
    }
}

// Prints a list of commands by building from bidirectional command map
fn list_all_commands(command_bimap: &BiMap<Command, String>) {
    let mut commands: Vec<_> = command_bimap.right_values().cloned().collect();
    commands.sort();
    commands.iter().for_each(|s| println!("-- {s}"));
}

// Splits a string to a vector depending on what the separator is.
pub fn split_to_vec(separator: &str, user_input: String) -> Vec<String>{
    let split_iter = user_input.split(separator);
    let separated_inputs_vec: Vec<String> = split_iter.map(|s| s.trim().to_string()).collect();

    separated_inputs_vec
}

// Prompts user to input something based on the function argument and return the user input.
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
use rusqlite::Result;
use database::query;
use crate::database::cloud::{fetch, has_internet_access, sync, Database};
use crate::database::{self, delete, insert};
use crate::helper::flush;
use std::io::stdin;
use super::commands::Command;


pub fn match_enums(user_input: String) -> Command {
    match user_input.trim() {
        "new ingredient" => Command::NewIngredient,
        "add price" => Command::AddPrice,
        "new dish" => Command::NewDish,
        "add recipe" => Command::AddRecipe,
        "list all dishes" => Command::ListAllDishes,
        "list all ingredients" => Command::ListAllIngredients,
        "i have" => Command::IHave,
        "recipe of" => Command::RecipeOf,
        "delete ingredient from recipe" => Command::DeleteIngredientFromRecipe,
        "delete dish" => Command::DeleteDish,
        "fetch database" => Command::FetchDatabase,
        "sync database" => Command::SyncDatabase,
        "help" => Command::Help,
        "quit" => Command::Quit,
        _ => Command::Unknown
    }
}

pub async fn match_commands(command_enum: Command) -> Result<()> {
    match command_enum {
        Command::NewIngredient => insert::ingredient().await,
        Command::AddPrice => insert::price().await,
        Command::NewDish => insert::dish().await,
        Command::AddRecipe => insert::recipe(None).await,
        Command::ListAllDishes => query::all_dish_names(),
        Command::ListAllIngredients => query::all_ingredients(),
        Command::IHave => query::dish_by_ingredients::get_dishes(),
        Command::RecipeOf => query::recipe_by_dish_name(),
        Command::DeleteIngredientFromRecipe => delete::ingredient_from_recipe().await,
        Command::DeleteDish => Ok(()),
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
        Command::Help => {
            list_all_commands();
            Ok(())
        },
        Command::Quit => std::process::exit(0),
        Command::Unknown => {
            eprintln!("Unknown command");
            Ok(())
        }
    }
}

fn list_all_commands() {
    let all_commands = vec![
        Command::NewIngredient.to_str(),
        Command::AddPrice.to_str(),
        Command::NewDish.to_str(),
        Command::AddRecipe.to_str(),
        Command::ListAllDishes.to_str(),
        Command::ListAllIngredients.to_str(),
        Command::IHave.to_str(),
        Command::RecipeOf.to_str(),
        Command::DeleteIngredientFromRecipe.to_str(),
        Command::DeleteDish.to_str(),
        Command::FetchDatabase.to_str(),
        Command::SyncDatabase.to_str(),
        Command::Help.to_str(),
        Command::Quit.to_str(),
        Command::Unknown.to_str(),
    ];
    for command in all_commands {
        println!("-- {command}");
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
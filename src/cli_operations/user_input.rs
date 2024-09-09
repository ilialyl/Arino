use rusqlite::Connection;
use database::query;
use crate::database;
use crate::helper::flush;
use std::io::stdin;

pub fn match_commands(user_input: String, conn: &Connection) {
    let mut user_input = user_input.split("\"");
    let command = user_input.next().expect("No command input");

    match command.trim() {
        "list all dishes" => query::all_dish_names(conn).expect("database error"),
        "list all ingredients" => query::all_ingredients(conn).expect("database error"),
        "i have" => query::dish_by_ingredients::get_dishes(conn).expect("database error"),
        "recipe of" => query::recipe_by_dish_name(conn).expect("database error"),
        "quit" => std::process::exit(0),
        _ => eprintln!("Unknown command"),
    }
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
use rusqlite::Connection;
use database::query_commands;
use crate::database;

pub fn match_commands(user_input: String, conn: &Connection) {
    let mut user_input = user_input.split("\"");
    let command = user_input.next().expect("No command input");
    let argument = user_input.next();

    match command.trim() {
        "dish all" => query_commands::query_all_dishes(conn).expect("database error"),
        "recipe" => query_commands::query_recipe(argument, conn).expect("database error"),
        "ingredient all" => query_commands::query_all_ingredients(conn).expect("database error"),
        "quit" => std::process::exit(0),
        _ => eprintln!("Unknown command"),
    }
}
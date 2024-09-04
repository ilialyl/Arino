use rusqlite::Connection;
use crate::database;

pub fn match_commands(user_input: String, conn: &Connection) {
    let mut user_input = user_input.split("\"");
    let command = user_input.next().expect("No command input");
    let argument = user_input.next();

    match command.trim() {
        "recipe" => database::query_commands::query_recipe(argument, conn).expect("This dish does not exist in database"),
        "quit" => std::process::exit(0),
        _ => eprintln!("Unknown command"),
    }
}
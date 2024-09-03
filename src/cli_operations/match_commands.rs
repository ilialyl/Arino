use rusqlite::{Connection, Result};
use crate::database;

pub fn match_commands(user_input: String, conn: &Connection) -> Result<()> {
    let mut user_input = user_input.split("\"");
    let command = user_input.next().expect("No command input");
    let argument = user_input.next().expect("Argument not specified");

    match command.trim() {
        "recipe" => database::query_commands::query_recipe(argument, conn).expect("This dish does not exist in database"),
        _ => eprintln!("Unknown command"),
    }

    Ok(())
}
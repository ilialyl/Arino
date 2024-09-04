use rusqlite::{Connection, Result};
use crate::database;

pub fn match_commands(user_input: String, conn: &Connection) -> Result<()> {
    if !user_input.contains(" ") {
        println!("Unknown command or argument");
        return Ok(());
    }
    let mut user_input = user_input.split("\"");
    let command = user_input.next().expect("No command input");
    let argument = Some(user_input.next());

    match command.trim() {
        "recipe" => database::query_commands::query_recipe(argument, conn).expect("This dish does not exist in database"),
        _ => eprintln!("Unknown command"),
    }

    Ok(())
}
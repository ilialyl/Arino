use rusqlite::Connection;
use database::query;
use crate::database;

pub fn match_commands(user_input: String, conn: &Connection) {
    let mut user_input = user_input.split("\"");
    let command = user_input.next().expect("No command input");
    let arg_str = user_input.next();
    let arg_list = get_argument_list(arg_str);

    match command.trim() {
        "all dishes" => query::all_dish_names(conn).expect("database error"),
        "I have" => query::dish_by_ingredients::get_dishes(arg_list, conn).expect("database error"),
        "recipe of" => query::recipe_by_dish_name(arg_list, conn).expect("database error"),
        "all ingredients" => query::all_ingredients(conn).expect("database error"),
        "quit" => std::process::exit(0),
        _ => eprintln!("Unknown command"),
    }
}

pub fn get_argument_list(arg_str: Option<&str>) -> Vec<String> {
    let arg_str = match arg_str {
        Some(s) => s,
        None => return Vec::new(),
    };

    let separated_args = arg_str.split(",");
    let arg_list: Vec<String> = separated_args.map(|s| s.trim().to_string().to_lowercase()).collect();

    arg_list
}
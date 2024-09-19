use rusqlite::Connection;

use crate::cli_operations::user_input::prompt;

pub fn dish_id(conn: &Connection) -> Option<u32> {
    let dish_id = loop {
        let dish_name = prompt("Dish name");
        if dish_name.is_empty() {
            return None;
        }

        let retrieved_dish_id: u32 = match conn.query_row("SELECT id FROM dishes WHERE name = ?1;", [&dish_name], |row| row.get(0)) {
            Ok(id) => id,
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                eprintln!("Invalid dish name");
                continue;
            },
            Err(e) => {
                eprintln!("Error: {e}");
                continue;
            }
        };
        break retrieved_dish_id;
    };

    Some(dish_id)
}

pub fn ingredient_id(conn: &Connection) -> Option<u32> {
    let ingredient_id = loop {
        let ingredient_name = prompt("Ingredient name");
        if ingredient_name.is_empty() {
            return None;
        }

        let retrieved_ingredient_id: u32 = match conn.query_row("SELECT id FROM ingredients WHERE name = ?1;", [&ingredient_name], |row| row.get(0)) {
            Ok(id) => id,
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                eprintln!("Invalid ingredient name");
                continue;
            },
            Err(e) => {
                eprintln!("Error: {e}");
                continue;
            }
        };
        break retrieved_ingredient_id;
    };

    Some(ingredient_id)
}


use prettytable::{Cell, Row, Table};
use rusqlite::{Connection, Result};
use std::collections::{HashMap, HashSet};

use crate::{cli_operations::commands, database::get_connection};

pub fn get_dishes(args: &commands::IHaveArgs) -> Result<()> {
    let conn = get_connection();

    let ingredient_vec = &args.ingredients;

    let mut ingredient_id_vec: Vec<u32> = Vec::new();

    // get all ingredient id
    let mut select_ingredient_ids_stmt = conn.prepare("SELECT id FROM ingredients;")?;
    let all_ingredient_ids_set: HashSet<u32> = select_ingredient_ids_stmt
        .query_map([], |row| Ok(row.get::<_, u32>(0)?))?
        .map(|result| result.unwrap())
        .collect();

    // get input ingredient id
    for ingredient in ingredient_vec {
        let mut select_ingredient_ids_by_name_stmt =
            conn.prepare("SELECT id FROM ingredients WHERE name = ?1;")?;
        let ingredient_ids_result: Result<u32> =
            select_ingredient_ids_by_name_stmt.query_row([&ingredient], |row| row.get(0));
        match ingredient_ids_result {
            Ok(id) => {
                ingredient_id_vec.push(id);
            }
            Err(e) => {
                eprintln!("Ingredient \"{}\" does not exist in database.", ingredient);
                eprintln!("{}", e);
                return Ok(());
            }
        };
    }

    let input_ingredient_ids_set: HashSet<u32> = ingredient_id_vec.into_iter().collect();

    let all_dish_recipes_map = get_all_recipes_map(&conn)?;

    let filtered_dish_ids_vec = filter_dishes_with_input_ingredients(
        &input_ingredient_ids_set,
        &all_ingredient_ids_set,
        &all_dish_recipes_map,
    );

    let mut available_dishes: Vec<String> = Vec::new();

    for id in filtered_dish_ids_vec {
        let mut select_dish_names_by_id_stmt =
            conn.prepare("SELECT name FROM dishes WHERE id = ?1;")?;
        let dish_names: String = select_dish_names_by_id_stmt.query_row([id], |row| row.get(0))?;
        available_dishes.push(dish_names);
    }

    if !available_dishes.is_empty() {
        let mut table: Table = Table::new();
        table.add_row(Row::new(vec![Cell::new("Available Dish")]));

        for dish in available_dishes {
            table.add_row(Row::new(vec![Cell::new(&dish)]));
        }

        table.printstd();
    } else {
        println!("No available dishes");
    }

    Ok(())
}

fn get_all_recipes_map(conn: &Connection) -> Result<HashMap<u32, Vec<u32>>> {
    let mut all_recipes_map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut select_dish_ids_stmt = conn.prepare("SELECT id FROM dishes;")?;
    let dish_ids_vec: Vec<u32> = select_dish_ids_stmt
        .query_map([], |row| Ok(row.get::<_, u32>(0)?))?
        .map(|result| result.unwrap())
        .collect();

    for dish_id in dish_ids_vec {
        let mut select_recipe_ingredient_ids_stmt =
            conn.prepare("SELECT ingredient_id FROM recipes WHERE dish_id = ?1;")?;
        let ingredient_ids_vec: Vec<u32> = select_recipe_ingredient_ids_stmt
            .query_map([dish_id], |row| Ok(row.get::<_, u32>(0)?))?
            .map(|result| result.unwrap())
            .collect();
        all_recipes_map.insert(dish_id, ingredient_ids_vec);
    }

    Ok(all_recipes_map)
}

fn filter_dishes_with_input_ingredients(
    input_ingredients: &HashSet<u32>,
    all_ingredients: &HashSet<u32>,
    dish_recipes: &HashMap<u32, Vec<u32>>,
) -> Vec<u32> {
    let mut filtered: Vec<u32> = Vec::new();
    for (dish_name, ingredients_vec) in dish_recipes {
        for keyword in input_ingredients {
            if ingredients_vec.contains(keyword) {
                if filter_dishes_with_other_ingredients(
                    input_ingredients,
                    all_ingredients,
                    ingredients_vec,
                ) {
                    filtered.push(*dish_name);
                    break;
                }
            }
        }
    }

    filtered
}

fn filter_dishes_with_other_ingredients(
    input_ingredients: &HashSet<u32>,
    all_ingredients: &HashSet<u32>,
    ingredients_vec: &Vec<u32>,
) -> bool {
    let other_ingredients: HashSet<_> = all_ingredients
        .difference(&input_ingredients)
        .cloned()
        .collect();

    for ingredient in other_ingredients {
        if ingredients_vec.contains(&ingredient) {
            return false;
        }
    }

    return true;
}

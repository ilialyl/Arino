use std::collections::{HashMap, HashSet};

use rusqlite::{Connection, Result};
use crate::helper::calculate_mean;
use prettytable::{Table, Row, Cell};

use super::logic::filter_recipes_with_ingredient;
#[allow(dead_code)]

pub fn query_all_dishes(conn: &Connection) -> Result<()> {
    let mut dishes_query = conn.prepare("Select id, name FROM dishes")?;
    let dishes_iter = dishes_query.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    })?;

    for dish in dishes_iter {
        let (id, name) = dish?;
        println!("ID {id}: {name}");
    }

    Ok(())
}

pub fn query_dish_with_ingredient(arg_list: Vec<String>, conn: &Connection) -> Result<()> {
    let ingredient_list = arg_list;
    if ingredient_list.is_empty() {
        eprintln!("No ingredient input for dish query");
        return Ok(());
    }

    // get all ingredient id
    let mut all_ingredient_id_query = conn.prepare("SELECT id FROM ingredients;")?;
    let all_ingredient_id_hashset: HashSet<u32> = all_ingredient_id_query
        .query_map([], |row| Ok(row.get::<_, u32>(0)?))?
        .map(|result| result.unwrap())
        .collect();

    // get input ingredient id
    let mut input_ingredient_id_list: Vec<u32> = Vec::new(); 
    for ingredient in ingredient_list {
        let mut input_ingredient_id_query = conn.prepare("SELECT id FROM ingredients WHERE name = ?1;")?;
        let ingredient_id: Result<u32> = input_ingredient_id_query.query_row([&ingredient], |row| row.get(0));
        let ingredient_id = match ingredient_id {
            Ok(id) => {
                id  
            },
            Err(e) => {
                eprintln!("Ingredient \"{}\" does not exist in database.", ingredient);
                eprintln!("{}", e);
                return Ok(());
            }
        };
        input_ingredient_id_list.push(ingredient_id);
    }
    let input_ingredient_id_hashset: HashSet<u32> = input_ingredient_id_list.into_iter().collect();

    let all_recipes_hashmap = get_all_recipes_hashmap(&conn)?;

    let dish_id_list = filter_recipes_with_ingredient(&input_ingredient_id_hashset, &all_ingredient_id_hashset, &all_recipes_hashmap);
    
    let mut available_dishes: Vec<String> = Vec::new();
    for id in dish_id_list {
        let mut stmt = conn.prepare("SELECT name FROM dishes WHERE id = ?1;")?;
        let dishes_result: String = stmt.query_row([id], |row| row.get(0))?;
        available_dishes.push(dishes_result);
    }

    if !available_dishes.is_empty() {
        println!("You can make: ");
        available_dishes.iter().for_each(|s| println!("- {s}") );
    }
    else {
        println!("No available dish");
    }
    
    Ok(())
}

pub fn get_all_recipes_hashmap(conn: &Connection) -> Result<HashMap<u32, Vec<u32>>> {
    let mut all_recipes: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut all_dish_id_query = conn.prepare("SELECT id FROM dishes;")?;
    let all_dish_id_list: Vec<u32> = all_dish_id_query
        .query_map([], |row| Ok(row.get::<_, u32>(0)?))?
        .map(|result| result.unwrap())
        .collect();
    for dish_id in all_dish_id_list {
        let mut all_dish_id_query = conn.prepare("SELECT ingredient_id FROM recipes WHERE dish_id = ?1;")?;
        let ingredient_id: Vec<u32> = all_dish_id_query
        .query_map([dish_id], |row| Ok(row.get::<_, u32>(0)?))?
        .map(|result| result.unwrap())
        .collect();
        all_recipes.insert(dish_id, ingredient_id);
    }

    Ok(all_recipes)
}

pub fn query_recipe(arg_list: Vec<String>, conn: &Connection) -> Result<()> {
    if arg_list.is_empty() {
        eprintln!("No dish name input for recipe query");
        return Ok(());
    }

    let dish_name = &arg_list[0];

    let mut dish_id_query = conn.prepare("SELECT id FROM dishes WHERE name = ?1;")?;
    let dish_id: Result<u32> = dish_id_query.query_row([dish_name], |row| row.get(0));

    let dish_id = match dish_id {
        Ok(id) => id,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            eprintln!("Dish \"{}\" not found in the database.", dish_name);
            return Ok(());
        }
        Err(e) => return Err(e),
    };
    
    let mut ingredient_id_query = conn.prepare("SELECT ingredient_id FROM recipes WHERE dish_id = ?1;")?;
    let ingredient_id_iter = ingredient_id_query.query_map([dish_id], |row| {
        Ok(row.get::<_, u32>(0)?)
    })?;

    let mut ingredient_names: Vec<String> = Vec::new();
    let mut ingredient_quantity: Vec<u32> = Vec::new();

    for ingredient_id in ingredient_id_iter {
        let ingredient_id = ingredient_id?;
        let ingredient_name_query: String = conn.query_row(
            "SELECT name FROM ingredients WHERE id = ?1;",
            [ingredient_id],
            |row| row.get(0),
        )?;

        ingredient_names.push(ingredient_name_query);

        let ingredient_quantity_query: u32 = conn.query_row(
            "SELECT quantity FROM recipes WHERE dish_id = ?1 AND ingredient_id = ?2;",
            [dish_id, ingredient_id],
            |row| row.get(0),
        )?;
            
        ingredient_quantity.push(ingredient_quantity_query);
    }

    println!("{}", "-".repeat(50));
    println!("Recipe for {dish_name}:");
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Name"),
        Cell::new("Quantity"),
    ]));
    for (name, quantity) in ingredient_names.iter().zip(ingredient_quantity.iter()) {
        table.add_row(Row::new(vec![
            Cell::new(&name),
            Cell::new(quantity.to_string().trim()),
        ]));
    }
    table.printstd();

    Ok(())
}

pub fn query_all_ingredients_details (conn: &Connection) -> Result<()> {
    let mut ingredient_details_query = conn.prepare("SELECT * FROM ingredients;")?;
    let ingredient_details_iter = ingredient_details_query.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(2)?, row.get::<_, String>(3)?))
    })?;

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("ID"),
        Cell::new("Name"),
        Cell::new("Lifespan"),
        Cell::new("Price"),
    ]));

    for ingredient_detail in ingredient_details_iter {
        let (ingredient_id, name, lifespan) = ingredient_detail?;
        let mut price_query = conn.prepare("SELECT price from prices where ingredient_id = ?1;")?;
        let prices_iter = price_query.query_map([ingredient_id], |row| {
            Ok(row.get::<_, f32>(0)?)
        })?;

        /* let prices_iter = match prices_iter {
            Ok(prices_iter) => prices_iter,
            Err(_) => {
                println!("ID {ingredient_id}: {name} ({lifespan}) (no data)");
                return Ok(());
            }
        }; */

        let mut prices: Vec<f32> = Vec::new();

        for price in prices_iter {
            prices.push(price?);
        }

        let mean_price = calculate_mean(prices);

        table.add_row(Row::new(vec![
            Cell::new(ingredient_id.to_string().trim()),
            Cell::new(&name),
            Cell::new(&lifespan),
            Cell::new(format!("${mean_price:.2}").trim()),
        ]));
    }

    table.printstd();

    Ok(())
}
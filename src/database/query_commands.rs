use rusqlite::{Connection, Result};
use crate::helper::calculate_mean;
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

pub fn query_recipe(dish_name: Option<&str>, conn: &Connection) -> Result<()> {
    let dish_name = match dish_name {
        Some(s) => s,
        None => {
            eprintln!("Dish name not input for recipe query");
            return Ok(());
        }
    };

    let mut ingredient_list: Vec<String> = Vec::new();

    let mut dish_id_query = conn.prepare("SELECT id FROM dishes WHERE name = ?1;")?;
    let dish_id: Result<u32> = dish_id_query.query_row([dish_name], |row| {
        row.get(0)
    });

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
        Ok(row.get::<_, i32>(0)?)
    })?;

    for id in ingredient_id_iter {
        let mut ingredient_names_query = conn.prepare("SELECT name FROM ingredients WHERE id = ?1;")?;
        let ingredient_names_iter = ingredient_names_query.query_map([id?], |row| {
            Ok(row.get::<_, String>(0)?)
        })?;

        for ingredient_name in ingredient_names_iter {
            ingredient_list.push(ingredient_name?);
        }
    }

    println!("Recipe for {dish_name}:");
    for ingredient_name in ingredient_list {
        println!("- {ingredient_name}");
    }

    Ok(())
}

pub fn query_all_ingredients (conn: &Connection) -> Result<()> {
    let mut ingredient_details_query = conn.prepare("SELECT * FROM ingredients;")?;
    let ingredient_details_iter = ingredient_details_query.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(2)?, row.get::<_, String>(3)?))
    })?;

    for ingredient_detail in ingredient_details_iter {
        let (ingredient_id, name, lifespan) = ingredient_detail?;
        let mut price_query = conn.prepare("SELECT price from prices where ingredient_id = ?1;")?;
        let prices_iter = price_query.query_map([ingredient_id], |row| {
            Ok(row.get::<_, f32>(0)?)
        });

        let prices_iter = match prices_iter {
            Ok(prices_iter) => prices_iter,
            Err(_) => {
                println!("ID {ingredient_id}: {name} ({lifespan}) (no data)");
                return Ok(());
            }
        };

        let mut prices: Vec<f32> = Vec::new();

        for price in prices_iter {
            prices.push(price?);
        }

        let mean_price = calculate_mean(prices);

        println!("ID {ingredient_id}: {name} ({lifespan}) (${mean_price})");
    }

    Ok(())
}
use rusqlite::{Connection, Result};
#[allow(dead_code)]

pub fn query_all_dishes(conn: Connection) -> Result<()> {
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

pub fn query_recipe(dish_name: &str, conn: &Connection) -> Result<()> {
    let mut ingredient_list: Vec<String> = Vec::new();

    let mut dish_id_query = conn.prepare("SELECT id FROM dishes WHERE name = ?1;")?;
    let dish_id: u32 = dish_id_query.query_row([dish_name], |row| {
        row.get(0)
    })?;
    
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
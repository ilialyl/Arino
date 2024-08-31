use rusqlite::{Connection, Result};

pub fn query_dishes(conn: Connection) -> Result<()> {
    let mut statement = conn.prepare("Select id, name FROM dishes")?;
    let dish_iter = statement.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
    })?;

    for dish in dish_iter {
        let (id, name) = dish?;
        println!("ID {id}: {name}");
    }

    Ok(())
}

pub fn query_recipes(conn: Connection, dish_name: &str) -> Result<()> {
    let mut ingredients: Vec<String> = Vec::new();

    let mut get_dish_id = conn.prepare("SELECT id FROM dishes WHERE name = ?1;")?;
    let dish_id: u32 = get_dish_id.query_row([dish_name], |row| {
        row.get(0)
    })?;
    
    let mut get_ingredient_id = conn.prepare("SELECT ingredient_id FROM recipes WHERE dish_id = ?1;")?;
    let ingredient_id_iter = get_ingredient_id.query_map([dish_id], |row| {
        Ok(row.get::<_, i32>(0)?)
    })?;

    for id in ingredient_id_iter {
        let mut get_ingredient_name = conn.prepare("SELECT name FROM ingredients WHERE id = ?1;")?;
        let ingredient_name_iter = get_ingredient_name.query_map([id?], |row| {
            Ok(row.get::<_, String>(0)?)
        })?;

        for name in ingredient_name_iter {
            ingredients.push(name?);
        }
    }

    println!("{dish_name}");
    for name in ingredients {
        println!("- {name}");
    }

    Ok(())
}
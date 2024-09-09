use rusqlite::{Connection, Result};

use crate::cli_operations::user_input::prompt;

pub fn ingredient(conn: Connection) -> Result<()> {
    let name = prompt("Name");

    if name.is_empty() {
        return Ok(());
    }

    let category = prompt("Category (vegetable, fruit, dairy, meat, condiment)");

    if category.is_empty() {
        return Ok(());
    }

    let category_exists: bool = conn.query_row("SELECT name FROM categories WHERE name = ?1", [&category], |row| row.get(0))?;
    if !category_exists {
        eprintln!("Invalid category");
        return Ok(());
    }
    let category_id: u32 = conn.query_row("SELECT id FROM categories WHERE name = ?1", [category], |row| row.get(0))?;

    let lifespan = prompt("Lifespan (in _y_mo_d_h_m_s)");

    let mut stmt = conn.prepare("INSERT INTO ingredients (category_id, name, lifespan) VALUES (?1, ?2, ?3)")?;
    stmt.execute((category_id, name, lifespan))?;

    Ok(())
}

pub fn price(ingredient_name: String, price: String, conn: Connection) -> Result<()> {

    Ok(())
}

pub fn dish(name: String, conn: Connection) -> Result<()> {

    Ok(())
}

pub fn recipe(dish_name: String, conn: Connection) -> Result<()> {

    Ok(())
}
use rusqlite::Result;
use crate::{cli_operations::user_input::prompt, database::{cloud::{fetch, has_internet_access, sync, Database}, get_connection}};

pub async fn ingredient() -> Result<()> {
    if !has_internet_access().await {
        return Ok(());
    }

    match fetch(Database::Main).await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        },
    }

    let conn = get_connection();
    let name = prompt("Name");

    if name.is_empty() {
        return Ok(());
    }

    let category = prompt("Category (vegetable, fruit, dairy, meat, condiment, grain)");

    if category.is_empty() {
        return Ok(());
    }

    let retrieved_category_name: String = conn.query_row("SELECT name FROM categories WHERE name = ?1;", [&category], |row| row.get(0))?;
    if retrieved_category_name.is_empty() {
        eprintln!("Invalid category");
        return Ok(());
    }

    let category_id: u32 = conn.query_row("SELECT id FROM categories WHERE name = ?1;", [&category], |row| row.get(0))?;

    let lifespan = prompt("Lifespan (in _y_mo_d_h_m_s)");

    let mut stmt = conn.prepare("INSERT INTO ingredients (category_id, name, lifespan) VALUES (?1, ?2, ?3);")?;
    stmt.execute((category_id, &name, &lifespan))?;
    println!("Inserted: {} {} {} successfully", name, category, lifespan);

    match sync().await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        },
    }

    Ok(())
}

pub fn price(ingredient_name: String, price: String) -> Result<()> {

    Ok(())
}

pub fn dish(name: String) -> Result<()> {

    Ok(())
}

pub fn recipe(dish_name: String) -> Result<()> {

    Ok(())
}
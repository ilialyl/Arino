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

pub async fn price() -> Result<()> {
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

    let ingredient_name = prompt("Ingredient name");

    if ingredient_name.is_empty() {
        return Ok(());
    }

    let retrieved_ingredient_name: String = conn.query_row("SELECT name FROM ingredients WHERE name = ?1;", [&ingredient_name], |row| row.get(0))?;
    if retrieved_ingredient_name.is_empty() {
        eprintln!("Invalid ingredient name");
        return Ok(());
    }

    let ingredient_id: u32 = conn.query_row("SELECT id FROM ingredients WHERE name = ?1;", [&ingredient_name], |row| row.get(0))?;

    let input_price = prompt("Price per kg in AUD");
    let input_price_float = match input_price.trim().parse::<f32>() {
        Ok(f) => f,
        Err(e) => {
            eprint!("{e}");
            return Ok(());
        },
    };

    let mut stmt = conn.prepare("INSERT INTO prices (ingredient_id, price) VALUES (?1, ?2);")?;
    stmt.execute((&ingredient_id, input_price))?;
    println!("Inserted: ${:.2} to {} successfully", input_price_float, ingredient_name);

    match sync().await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        },
    }

    Ok(())
}

pub async fn dish() -> Result<()> {
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

    let dish_name = prompt("Dish name");

    if dish_name.is_empty() {
        return Ok(());
    }

    let mut stmt = conn.prepare("INSERT INTO dishes (name) VALUES (?1);")?;
    stmt.execute([&dish_name])?;
    println!("Inserted {dish_name} successfully. Do you want to add recipe now?");
    if prompt("Y/N") == "y" {
        match recipe(Some(dish_name)).await {
            Ok(_) => {},
            Err(e) => eprintln!("{e}"),
        }
    }

    match sync().await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        },
    }

    Ok(())
}

pub async fn recipe(dish_name: Option<String>) -> Result<()> {
    let dish_name = match dish_name {
        Some(s) => s,
        None => {
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
        
            let s = prompt("Dish name");
            if s.is_empty() {
                return Ok(());
            }
            s
        },
    };

    println!("Your dish is {dish_name}, but recipe insertion function is not yet implemented.");    

    Ok(())
}
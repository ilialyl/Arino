use rusqlite::Result;

use crate::{cli_operations::{cancel_prompt, user_input::prompt}, database::cloud::sync};

use super::{cloud::{fetch, has_internet_access, Database}, get, get_connection, show};

// Fetches the database from Cloud, modify information about an ingredient of choice, and sync the database to Cloud.
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

    let ingredient_id = match get::ingredient_id(&conn) {
        Some(id) => id,
        None => return Ok(()),
    };

    let new_name = prompt("New name");

    if !new_name.is_empty() {
        let mut update_name_stmt = conn.prepare("UPDATE ingredients SET name = ?1 WHERE id = ?2")?;
        update_name_stmt.execute((&new_name, &ingredient_id))?;
    }

    let new_lifespan = prompt("New lifespan");

    if !new_lifespan.is_empty() {
        let mut update_lifespan_stmt = conn.prepare("UPDATE ingredients SET name = ?1 WHERE id = ?2")?;
        update_lifespan_stmt.execute((&new_lifespan, &ingredient_id))?;
    }

    match get::category_name_and_id(&conn) {
        Some((_, category_id)) => {
            let mut update_category_stmt = conn.prepare("UPDATE ingredients SET category_id = ?1 WHERE id = ?2")?;
            update_category_stmt.execute((category_id, &ingredient_id))?;
        }
        None => {},
    }

    println!("Ingredient Updated");
    match show::specific_ingredient(ingredient_id) {
        Ok(_) => {},
        Err(e) => eprintln!("Error: {e}"),
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

// Fetches the database from Cloud, modify the name of a dish of choice, and sync the database to Cloud.
pub async fn dish_name() -> Result<()> {
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

    let dish_id = match get::dish_id(&conn) {
        Some(id) => id,
        None => return Ok(()),
    };

    let old_name = match get::dish_name(dish_id, &conn) {
        Some(name) => name,
        None => return Ok(()),
    };

    let new_name = prompt("New dish name");
    if new_name.is_empty() {
        cancel_prompt();
        return Ok(())
    }

    let mut update_name_stmt = conn.prepare("UPDATE dishes SET name = ?1 WHERE id = ?2")?;
    update_name_stmt.execute((&new_name, dish_id))?;

    let retrieved_new_name = match get::dish_name(dish_id, &conn) {
        Some(name) => name,
        None => {
            return Ok(())
        }
    };

    println!("\"{old_name}\" has been updated to \"{retrieved_new_name}\"");

    match sync().await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        },
    }

    Ok(())
}
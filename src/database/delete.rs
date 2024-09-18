use rusqlite::Result;

use crate::{cli_operations::user_input::prompt, database::cloud::sync};

use super::{cloud::{fetch, has_internet_access, Database}, get_connection};



pub async fn ingredient_from_recipe() -> Result<()> {
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

    let dish_id = loop {
        let dish_name = prompt("Dish name");
        if dish_name.is_empty() {
            return Ok(());
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

    let ingredient_id = loop {
        let ingredient_name = prompt("Ingredient name");
        if ingredient_name.is_empty() {
            return Ok(());
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

    println!("Are you sure you want to delete this ingredient from this recipe?");
    if prompt("[Y/N]") != "y" {
        println!("Deletion aborted");
        return Ok(());
    }
    
    let mut stmt = conn.prepare("DELETE FROM recipes WHERE dish_id = ?1 AND ingredient_id = ?2;")?;
    stmt.execute((&dish_id, &ingredient_id))?;

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

    let dish_id = loop {
        let dish_name = prompt("Dish name");
        if dish_name.is_empty() {
            return Ok(());
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

    println!("Are you sure you want to delete this dish along with its recipe?");
    if prompt("[Y/N]") != "y" {
        println!("Deletion aborted");
        return Ok(());
    }

    let mut delete_recipe_stmt = conn.prepare("DELETE FROM recipes WHERE dish_id = ?1")?;
    delete_recipe_stmt.execute([dish_id])?;

    let mut delete_dish_stmt = conn.prepare("DELETE FROM dishes WHERE id = ?1")?;
    delete_dish_stmt.execute([dish_id])?;

    match sync().await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        },
    }

    Ok(())
}

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

    let ingredient_id = loop {
        let ingredient_name = prompt("Ingredient name");
        if ingredient_name.is_empty() {
            return Ok(());
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

    println!("Are you sure you want to delete this ingredient from the database?");
    if prompt("[Y/N]") != "y" {
        println!("Deletion aborted");
        return Ok(());
    }
    
    let mut stmt = conn.prepare("DELETE FROM ingredients WHERE id = ?1;")?;
    stmt.execute([&ingredient_id])?;

    match sync().await {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        },
    }



    Ok(())
}
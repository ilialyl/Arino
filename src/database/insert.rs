use rusqlite::Result;
use crate::{cli_operations::{cancel_prompt, user_input::prompt}, database::{cloud::{fetch, has_internet_access, sync, Database}, get_connection}};

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

    let ingredient_name = prompt("Name");
    if ingredient_name.is_empty() {
        cancel_prompt();
        return Ok(());
    }

    let (category_name, category_id) = loop {
        let input_category_name = prompt("Category (vegetable, fruit, dairy, meat, condiment, grain)");
        if input_category_name.is_empty() {
            cancel_prompt();
            return Ok(());
        }

        let retrieved_category_id: u32 = match conn.query_row("SELECT id FROM categories WHERE name = ?1;", [&input_category_name], |row| row.get(0)) {
            Ok(id) => id,
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                eprintln!("Invalid category");
                continue;
            },
            Err(e) => {
                eprintln!("Error: {e}");
                continue;
            }
        };
        break (input_category_name, retrieved_category_id);
    };

    let lifespan = prompt("Lifespan (in _y_mo_d_h_m_s)");

    let mut stmt = conn.prepare("INSERT INTO ingredients (category_id, name, lifespan) VALUES (?1, ?2, ?3);")?;
    stmt.execute((category_id, &ingredient_name, &lifespan))?;
    println!("Inserted: {} {} {} successfully", ingredient_name, category_name, lifespan);

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

    let (ingredient_name, ingredient_id) = loop {
        let input_ingredient_name = prompt("Ingredient name");
        if input_ingredient_name.is_empty() {
            cancel_prompt();
            return Ok(());
        }

        let retrieved_ingredient_id: u32 = match conn.query_row("SELECT id FROM ingredients WHERE name = ?1;", [&input_ingredient_name], |row| row.get(0)) {
            Ok(id) => id,
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                eprintln!("Invalid category");
                continue;
            },
            Err(e) => {
                eprintln!("Error: {e}");
                continue;
            }
        };
        break (input_ingredient_name, retrieved_ingredient_id);
    };

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
        cancel_prompt();
        return Ok(());
    }

    let mut stmt = conn.prepare("INSERT INTO dishes (name) VALUES (?1);")?;
    
    stmt.execute([&dish_name])?;

    println!("Inserted {dish_name} successfully. Do you want to add recipe now?");

    if prompt("[Y/N]") == "y" {
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
    let chained_operation: bool;

    let conn = get_connection();

    let (dish_name, dish_id) = match dish_name {
        Some(dish_name) => {
            chained_operation = true;
            let retrieved_dish_id: u32 = conn.query_row("SELECT id FROM dishes WHERE name = ?1;", [&dish_name], |row| row.get(0))?;

            (dish_name, retrieved_dish_id)
        },
        None => {
            chained_operation = false;
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

            let (dish_name, dish_id) = loop {
                let input_dish_name = prompt("Dish name");
                if input_dish_name.is_empty() {
                    cancel_prompt();
                    return Ok(());
                }
        
                let retrieved_dish_id: u32 = match conn.query_row("SELECT id FROM dishes WHERE name = ?1;", [&input_dish_name], |row| row.get(0)) {
                    Ok(id) => id,
                    Err(rusqlite::Error::QueryReturnedNoRows) => {
                        eprintln!("Invalid dish");
                        continue;
                    },
                    Err(e) => {
                        eprintln!("Error: {e}");
                        continue;
                    }
                };
                break (input_dish_name, retrieved_dish_id);
            };

            (dish_name, dish_id)
        },
    };

    let mut ingredients_added_vec: Vec<String> = Vec::new();

    'outer: loop {
        let (ingredient_name, ingredient_id) = 'name_and_id: loop {
            let input_ingredient_name = prompt("Ingredient name");
            if input_ingredient_name.is_empty() {
                break 'outer;
            }
    
            let retrieved_ingredient_id: u32 = match conn.query_row("SELECT id FROM ingredients WHERE name = ?1;", [&input_ingredient_name], |row| row.get(0)) {
                Ok(id) => id,
                Err(rusqlite::Error::QueryReturnedNoRows) => {
                    eprintln!("Invalid dish");
                    continue;
                },
                Err(e) => {
                    eprintln!("Error: {e}");
                    continue;
                }
            };
            break 'name_and_id (input_ingredient_name, retrieved_ingredient_id);
        };

        let quantity = 'quantity: loop {
            let user_input = prompt("Quantity (g)");

            if user_input.is_empty() {
                cancel_prompt();
                return Ok(());
            }

            match user_input.parse::<u32>() {
                Ok(num) => break num,
                Err(_) => {
                    eprintln!("Invalid quantity");
                    continue 'quantity;
                }
            }
        };

        let mut stmt = conn.prepare("INSERT INTO recipes (dish_id, ingredient_id, quantity) VALUES (?1, ?2, ?3);")?;
        stmt.execute((&dish_id, &ingredient_id, &quantity))?;

        ingredients_added_vec.push(ingredient_name);
    }

    if !ingredients_added_vec.is_empty() {
        let ingredient_added_string = ingredients_added_vec.join(", ");

        println!("Inserted: {ingredient_added_string} into {dish_name}'s recipe");
    }
    
    if !chained_operation {
        match sync().await {
            Ok(_) => {},
            Err(e) => {
                eprintln!("{e}");
                return Ok(());
            },
        }
    }

    Ok(())
}
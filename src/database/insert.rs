use crate::cli::commands;
use crate::database::{
    cloud::{fetch, has_internet_access, push, Database},
    get_connection,
};
use rusqlite::Result;

// Fetches the database from Cloud, insert an ingredient of choice, and sync the database to Cloud.
pub async fn ingredient(args: &commands::NewIngredientArgs) -> Result<()> {
    if !has_internet_access().await {
        return Ok(());
    }

    match fetch(Database::Main).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    }

    let conn = get_connection();
    let name = &args.name;
    let category_id = args.category as u8;
    let lifespan = &args.lifespan;

    let mut stmt =
        conn.prepare("INSERT INTO ingredients (category_id, name, lifespan) VALUES (?1, ?2, ?3);")?;
    stmt.execute((category_id, &name, &lifespan))?;
    println!(
        "Inserted: {} {} {} successfully",
        &name, category_id, &lifespan
    );

    match push().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    }

    Ok(())
}

// Fetches the database from Cloud, insert price to an ingredient of choice, and sync the database to Cloud.
pub async fn price(args: &commands::AddPriceArgs) -> Result<()> {
    if !has_internet_access().await {
        return Ok(());
    }

    match fetch(Database::Main).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    }

    let conn = get_connection();
    let ingredient = &args.ingredient;
    let price = args.price;

    let ingredient_id: u32 = match conn.query_row(
        "SELECT id FROM ingredients WHERE name = ?1;",
        [&ingredient],
        |row| row.get(0),
    ) {
        Ok(id) => id,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            eprintln!("Invalid category");
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        Err(e) => {
            eprintln!("Error: {e}");
            return Err(e);
        }
    };

    let mut stmt = conn.prepare("INSERT INTO prices (ingredient_id, price) VALUES (?1, ?2);")?;
    stmt.execute((&ingredient_id, price))?;
    println!("Inserted: ${:.2} to {} successfully", price, ingredient);

    match push().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    }

    Ok(())
}

// Fetches the database from Cloud, insert a dish of choice, and sync the database to Cloud.
pub async fn dish(args: &commands::NewDishArgs) -> Result<()> {
    if !has_internet_access().await {
        return Ok(());
    }

    match fetch(Database::Main).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    }

    let conn = get_connection();

    let dish = &args.name;

    let mut stmt = conn.prepare("INSERT INTO dishes (name) VALUES (?1);")?;

    stmt.execute([&dish])?;

    println!("Inserted {dish} successfully. Do you want to add recipe now?");

    match push().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    }

    Ok(())
}

// Fetches the database from Cloud, insert a recipe of choice, and sync the database to Cloud.
pub async fn recipe(args: &commands::AddRecipeArgs) -> Result<()> {
    let conn = get_connection();
    let dish_name = &args.dish;

    let dish_id: u32 = conn.query_row(
        "SELECT id FROM dishes WHERE name = ?1;",
        [&dish_name],
        |row| row.get(0),
    )?;

    let ingredient_vec = &args.ingredient;
    let quantity_vec = &args.quantity;

    if ingredient_vec.len() != quantity_vec.len() {
        eprintln!("The number of ingredients and amounts are not the same.");
        return Ok(());
    }

    let mut ingredient_id_vec: Vec<u32> = Vec::new();

    for ingredient in ingredient_vec {
        let id: u32 = match conn.query_row(
            "SELECT id FROM ingredients WHERE name = ?1;",
            [&ingredient],
            |row| row.get(0),
        ) {
            Ok(id) => id,
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                eprintln!("{ingredient} does not exist, skipping.");
                0u32
            }
            Err(e) => {
                eprintln!("Error: {e}");
                0u32
            }
        };

        ingredient_id_vec.push(id);
    }

    for (ingredient_id, quantity) in ingredient_id_vec.iter().zip(quantity_vec.iter()) {
        if *ingredient_id == 0 {
            continue;
        }

        let mut stmt = conn.prepare(
            "INSERT INTO recipes (dish_id, ingredient_id, quantity) VALUES (?1, ?2, ?3);",
        )?;
        stmt.execute((&dish_id, &ingredient_id, &quantity))?;
    }

    match push().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    }

    Ok(())
}

use rusqlite::Result;

use crate::cli::{commands, prompt};

use super::{
    cloud::{Database, fetch, has_internet_access, push},
    get, get_connection,
};

// Fetches the database from Cloud and deletes an ingredient of choice from a recipe.
pub async fn ingredient_from_recipe(args: &commands::DeleteIngredientFromRecipeArgs) -> Result<()> {
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

    let dish = &args.dish;
    let ingredient = &args.ingredient;

    let dish_id = match get::dish_id(&dish, &conn) {
        Some(id) => id,
        None => return Ok(()),
    };

    let ingredient_id = match get::ingredient_id(&ingredient, &conn) {
        Some(id) => id,
        None => return Ok(()),
    };

    println!("Are you sure you want to delete {ingredient} from {dish}?");
    if prompt("[Y/N]") != "y" {
        println!("Deletion aborted");
        return Ok(());
    }

    let mut stmt =
        conn.prepare("DELETE FROM recipes WHERE dish_id = ?1 AND ingredient_id = ?2;")?;
    stmt.execute((&dish_id, &ingredient_id))?;

    match push().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    }

    Ok(())
}

// Fetches the database from Cloud and deletes a dish of choice
pub async fn dish(args: &commands::DeleteDishArgs) -> Result<()> {
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
    let dish = &args.dish;

    let dish_id = match get::dish_id(&dish, &conn) {
        Some(id) => id,
        None => return Ok(()),
    };

    println!("Are you sure you want to delete {dish} along with its recipe?");
    if prompt("[y/N]") != "y" {
        println!("Deletion aborted");
        return Ok(());
    }

    let mut delete_stmt = conn.prepare("DELETE FROM recipes WHERE dish_id = ?1")?;
    delete_stmt.execute([dish_id])?;

    let mut delete_dish_stmt = conn.prepare("DELETE FROM dishes WHERE id = ?1")?;
    delete_dish_stmt.execute([dish_id])?;

    match push().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    }

    Ok(())
}

// Fetches the database from Cloud and deletes an ingredient of choice.
pub async fn ingredient(args: &commands::DeleteIngredientArgs) -> Result<()> {
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

    let ingredient_id = match get::ingredient_id(&ingredient, &conn) {
        Some(id) => id,
        None => return Ok(()),
    };

    println!("Are you sure you want to delete {ingredient} from the database?");
    if prompt("[y/N]") != "y" {
        println!("Deletion aborted");
        return Ok(());
    }

    let mut stmt = conn.prepare("DELETE FROM ingredients WHERE id = ?1;")?;
    stmt.execute([&ingredient_id])?;

    match push().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    }

    Ok(())
}

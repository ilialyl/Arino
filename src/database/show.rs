pub mod dish_by_ingredients;
use crate::cli_operations::commands;

use crate::database::Category;
use crate::{cli_operations::user_input::prompt, helper::calculate_mean};
use prettytable::{Cell, Row, Table};
use rusqlite::Result;

use super::{get, get_connection};

// Prints all dishes by their names
pub fn all_dish_names(_args: &commands::ListAllDishesArgs) -> Result<()> {
    let conn = get_connection();
    let mut select_dish_names_stmt = conn.prepare("Select id, name FROM dishes")?;
    let names_iter = select_dish_names_stmt.query_map([], |row| {
        Ok((row.get::<_, u32>(0)?, row.get::<_, String>(1)?))
    })?;

    let mut table: Table = Table::new();
    table.add_row(Row::new(vec![Cell::new("ID"), Cell::new("Name")]));

    for dish in names_iter {
        let (id, name) = dish?;

        table.add_row(Row::new(vec![Cell::new(&id.to_string()), Cell::new(&name)]));
    }

    table.printstd();

    Ok(())
}

// Prints a recipe by prompting dish name
pub fn recipe_by_dish_name() -> Result<()> {
    let conn = get_connection();

    let dish_name = prompt("Dish name");

    if dish_name.trim().is_empty() {
        return Ok(());
    }

    let mut select_dish_ids_by_name_stmt =
        conn.prepare("SELECT id FROM dishes WHERE name = ?1;")?;
    let dish_id_result: Result<u32> =
        select_dish_ids_by_name_stmt.query_row([&dish_name], |row| row.get(0));

    let dish_id = match dish_id_result {
        Ok(id) => id,
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    };

    let mut select_recipe_ingredient_ids_stmt =
        conn.prepare("SELECT ingredient_id FROM recipes WHERE dish_id = ?1;")?;
    let ingredient_ids_iter =
        select_recipe_ingredient_ids_stmt.query_map([dish_id], |row| Ok(row.get::<_, u32>(0)?))?;

    let mut ingredient_names: Vec<String> = Vec::new();
    let mut ingredient_quantities: Vec<u32> = Vec::new();

    for ingredient_id in ingredient_ids_iter {
        let ingredient_id = ingredient_id?;
        let ingredient_name: String = conn.query_row(
            "SELECT name FROM ingredients WHERE id = ?1;",
            [ingredient_id],
            |row| row.get(0),
        )?;

        ingredient_names.push(ingredient_name);

        let ingredient_quantity: u32 = conn.query_row(
            "SELECT quantity FROM recipes WHERE dish_id = ?1 AND ingredient_id = ?2;",
            [dish_id, ingredient_id],
            |row| row.get(0),
        )?;

        ingredient_quantities.push(ingredient_quantity);
    }

    println!("{}", "-".repeat(50));
    println!("Recipe for {dish_name}:");
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Ingredient"),
        Cell::new("Quantity (normally g)"),
    ]));
    for (name, quantity) in ingredient_names.iter().zip(ingredient_quantities.iter()) {
        table.add_row(Row::new(vec![
            Cell::new(&name),
            Cell::new(&quantity.to_string()),
        ]));
    }
    table.printstd();

    Ok(())
}

// Prints a list of all ingredients
pub fn all_ingredients(args: &commands::ListAllIngredientsArgs) -> Result<()> {
    let conn = get_connection();

    let category_id = match &args.category {
        Some(category) => *category as u32,
        None => Category::All as u32,
    };

    let mut select_ingredients_stmt: rusqlite::Statement<'_>;
    if category_id > 0 {
        select_ingredients_stmt =
            conn.prepare("SELECT * FROM ingredients WHERE category_id = ?1;")?;
    } else {
        select_ingredients_stmt = conn.prepare("SELECT * FROM ingredients")?;
    }

    let ingredients_iter = select_ingredients_stmt.query_map([category_id], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
        ))
    })?;

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("ID"),
        Cell::new(&format!("Name")),
        Cell::new("Lifespan"),
        Cell::new("Price"),
    ]));

    for ingredient in ingredients_iter {
        let (id, name, lifespan) = ingredient?;
        let mut price_query = conn.prepare("SELECT price from prices where ingredient_id = ?1;")?;
        let prices_iter = price_query.query_map([id], |row| Ok(row.get::<_, f32>(0)?))?;

        let mut prices: Vec<f32> = Vec::new();

        for price in prices_iter {
            prices.push(price?);
        }

        let mean_price = calculate_mean(prices);

        table.add_row(Row::new(vec![
            Cell::new(&id.to_string()),
            Cell::new(&name),
            Cell::new(&lifespan),
            Cell::new(&format!("${mean_price:.2}")),
        ]));
    }

    table.printstd();

    Ok(())
}

// Prints information about an ingredient of choice.
pub fn an_ingredient_info(ingredient_id: u32) -> Result<()> {
    let conn = get_connection();

    let mut stmt =
        conn.prepare("SELECT id, category_id, name, lifespan FROM ingredients WHERE id = ?1")?;
    stmt.query_row([ingredient_id], |row| {
        let id: u32 = row.get(0)?;
        let category_id: u32 = row.get(1)?;
        let name: String = row.get(2)?;
        let lifespan: String = row.get(3)?;
        let price = match get::price(ingredient_id, &conn) {
            Some(num) => num,
            None => f32::NAN,
        };

        let mut table = Table::new();

        table.add_row(Row::new(vec![
            Cell::new("ID"),
            Cell::new("Category ID"),
            Cell::new("Name"),
            Cell::new("Lifespan"),
            Cell::new("Mean Price"),
        ]));

        table.add_row(Row::new(vec![
            Cell::new(&id.to_string()),
            Cell::new(&category_id.to_string()),
            Cell::new(&name),
            Cell::new(&lifespan),
            Cell::new(&format!("${price:.2}")),
        ]));

        table.printstd();

        Ok(())
    })?;

    Ok(())
}

pub mod dish_by_ingredients;
use crate::cli::commands;

use crate::database::Category;
use crate::miscellaneous::calculate_mean;
use prettytable::{Cell, Row, Table};
use rusqlite::Result;

use super::{get, get_connection};

// Prints all dishes by their names
pub fn all_dish_names(_args: &commands::ListAllDishesArgs) -> Result<()> {
    let conn = get_connection();
    let mut dish_names_stmt = conn.prepare("Select id, name FROM dishes")?;
    let name_iter = dish_names_stmt.query_map([], |row| {
        Ok((row.get::<_, u32>(0)?, row.get::<_, String>(1)?))
    })?;

    let mut table: Table = Table::new();
    table.add_row(Row::new(vec![Cell::new("ID"), Cell::new("Name")]));

    for dish in name_iter {
        let (id, name) = dish?;

        table.add_row(Row::new(vec![Cell::new(&id.to_string()), Cell::new(&name)]));
    }

    table.printstd();

    Ok(())
}

// Prints a recipe by prompting dish name
pub fn recipe_by_dish_name(args: &commands::RecipeOfArgs) -> Result<()> {
    let conn = get_connection();

    let dish = &args.dish.to_lowercase();

    let mut dish_id_stmt = conn.prepare("SELECT id FROM dishes WHERE name = ?1;")?;
    let dish_id = match dish_id_stmt.query_row([&dish], |row| row.get(0)) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    };

    let mut ingredient_id_stmt =
        conn.prepare("SELECT ingredient_id FROM recipes WHERE dish_id = ?1;")?;
    let ingredient_id_iter =
        ingredient_id_stmt.query_map([dish_id], |row| Ok(row.get::<_, u32>(0)?))?;

    let mut ingredient_vec: Vec<String> = Vec::new();
    let mut ingredient_quantity_vec: Vec<u32> = Vec::new();

    for ingredient_id in ingredient_id_iter {
        let ingredient_id = ingredient_id?;
        let ingredient_name: String = conn.query_row(
            "SELECT name FROM ingredients WHERE id = ?1;",
            [ingredient_id],
            |row| row.get(0),
        )?;

        ingredient_vec.push(ingredient_name);

        let ingredient_quantity: u32 = conn.query_row(
            "SELECT quantity FROM recipes WHERE dish_id = ?1 AND ingredient_id = ?2;",
            [dish_id, ingredient_id],
            |row| row.get(0),
        )?;

        ingredient_quantity_vec.push(ingredient_quantity);
    }

    println!("{}", "-".repeat(50));
    println!("Recipe for {dish}:");
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Ingredient"),
        Cell::new("Quantity (normally g)"),
    ]));
    for (name, quantity) in ingredient_vec.iter().zip(ingredient_quantity_vec.iter()) {
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

    let query = if category_id > 0 {
        "SELECT * FROM ingredients WHERE category_id = ?1;"
    } else {
        "SELECT * FROM ingredients"
    };

    let mut stmt = conn.prepare(query)?;

    let ingredients_iter: Box<dyn Iterator<Item = rusqlite::Result<(i32, String, String)>>> =
        if category_id > 0 {
            Box::new(stmt.query_map([category_id], |row| {
                Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })?)
        } else {
            Box::new(stmt.query_map([], |row| {
                Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })?)
        };

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
pub fn ingredient_info(ingredient_id: u32) -> Result<()> {
    let conn = get_connection();

    let mut stmt =
        conn.prepare("SELECT id, category_id, name, lifespan FROM ingredients WHERE id = ?1")?;
    stmt.query_row([ingredient_id], |row| {
        let id: u32 = row.get(0)?;
        let category_id: u32 = row.get(1)?;
        let category = match Category::from_u32(category_id) {
            Some(cat) => cat.as_str(),
            None => "none",
        };

        let name: String = row.get(2)?;
        let lifespan: String = row.get(3)?;
        let price = match get::price(ingredient_id, &conn) {
            Some(num) => num,
            None => f32::NAN,
        };

        let mut table = Table::new();

        table.add_row(Row::new(vec![
            Cell::new("ID"),
            Cell::new("Category"),
            Cell::new("Name"),
            Cell::new("Lifespan"),
            Cell::new("Mean Price"),
        ]));

        table.add_row(Row::new(vec![
            Cell::new(&id.to_string()),
            Cell::new(&category),
            Cell::new(&name),
            Cell::new(&lifespan),
            Cell::new(&format!("${price:.2}")),
        ]));

        table.printstd();

        Ok(())
    })?;

    Ok(())
}

use rusqlite::{Connection, Result};

pub fn ingredient(name: String, category: String, lifespan: String, conn: Connection) -> Result<()> {

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
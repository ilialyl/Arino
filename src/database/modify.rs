use rusqlite::Result;

use crate::{cli_operations::user_input::prompt, database::cloud::sync};

use super::{cloud::{fetch, has_internet_access, Database}, get, get_connection};

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

    match get::dish_id(&conn);

    Ok(())
}
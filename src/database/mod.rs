pub mod insert;
pub mod delete;
pub mod query;
pub mod cloud;

use rusqlite::Connection;
use std::fs::metadata;
use std::path::Path;

pub fn get_connection() -> Connection {
    let path: String = "database.db".to_string();
    let conn = Connection::open(path)
        .expect("Error connecting to database");

    conn
}

pub fn first_start() -> bool {
    let path = Path::new("database.db");
    
    if path.exists() && path.is_file() {
        let file_metadata = metadata(path).expect("Error checking file");
        return !(file_metadata.len() > 0);
    } else {
        return true;
    }
}

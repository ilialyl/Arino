pub mod insert;
pub mod delete;
pub mod show;
pub mod cloud;
pub mod modify;
pub mod get;

use rusqlite::Connection;
use std::fs::metadata;
use std::path::Path;

// Opens and returns a connection to the database
pub fn get_connection() -> Connection {
    let path: String = "database.db".to_string();
    let conn = Connection::open(path)
        .expect("Error connecting to database");

    conn
}

// Checks if database file exists.
pub fn database_exists() -> bool {
    let path = Path::new("database.db");
    
    if path.exists() && path.is_file() {
        let file_metadata = metadata(path).expect("Error checking file");
        return file_metadata.len() > 0;
    } else {
        return false;
    }
}

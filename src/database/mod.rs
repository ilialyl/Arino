pub mod insert;
pub mod delete;
pub mod query;
pub mod connection;
pub mod cloud;

use rusqlite::Connection;

pub fn get_connection() -> Connection {
    let path: String = "database.db".to_string();
    let conn = Connection::open(path)
        .expect("Error connecting to database");

    conn
}


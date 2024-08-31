use rusqlite::Connection;

pub fn get_connection(path: &str) -> Connection {
    let conn = Connection::open(path)
        .expect("Error connecting to database");

    conn
}
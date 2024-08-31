mod database;

use database::query_commands::query_recipes;
use database::connection::get_connection;

fn main() {
    let path: String = "d:\\lyns0\\Dev\\Database\\project_kechi.db".to_string();
    let connection = get_connection(&path);
    query_recipes(connection, "Fried egg")
        .expect("Error querying recipes");
}

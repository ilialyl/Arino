pub mod database;
pub mod cli_operations;
pub mod helper;

use std::io;

use cli_operations::user_input;
use database::connection::get_connection;
use helper::flush;

fn main() {
    let path: String = "d:\\lyns0\\Dev\\Database\\project_kechi.db".to_string();
    let connection = get_connection(&path);
    
    println!("Arino v0.1");
    loop {
        let mut user_input = String::new();
        print!(">");
        flush();
        io::stdin().read_line(&mut user_input)
            .expect("Error reading user input");
        user_input::match_commands(user_input, &connection);
    }
}

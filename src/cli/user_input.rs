use crate::miscellaneous::flush;
use std::io::stdin;

// Prompts user to input something based on the function argument and return the user input.
pub fn prompt(prompt: &str) -> String {
    let mut user_input = String::new();
    print!("{}> ", prompt);
    flush();
    match stdin().read_line(&mut user_input) {
        Ok(_) => return user_input.trim().to_lowercase().to_string(),
        Err(e) => {
            eprint!("{e}");
            return user_input;
        }
    }
}

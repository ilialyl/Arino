pub mod user_input;
pub mod commands;

use crate::helper::flush;

// Flushes the input stream and prints cancel message
pub fn cancel_prompt() {
    flush();
    print!("\rAction Canceled!");
}
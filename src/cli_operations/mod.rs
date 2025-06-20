pub mod commands;
pub mod user_input;

use crate::miscellaneous::flush;

// Flushes the input stream and prints cancel message
pub fn cancel_prompt() {
    flush();
    print!("\rAction Canceled!");
}

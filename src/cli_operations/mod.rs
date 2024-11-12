pub mod user_input;
pub mod commands;

use crate::helper::flush;

pub fn cancel_prompt() {
    flush();
    print!("\rAction Canceled!");
}
use std::io;

pub fn flush() {
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
}
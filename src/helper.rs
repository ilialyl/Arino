use std::io;

// Flushes the IO stream
pub fn flush() {
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
}

// Gets the mean of floating point values in a vector.
pub fn calculate_mean(float_vec: Vec<f32>) -> f32{
    let count = float_vec.len() as f32;
    let sum = float_vec.iter().fold(0.0, |acc, value| acc + value);
    let mean = sum / count;

    mean
}
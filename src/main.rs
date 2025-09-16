use std::fs::File;
use std::io::{self, Read};
use std::env;

fn is_jpeg(file_path: &str) -> io::Result<bool> {
    let mut file = File::open(file_path)?;
    let mut buffer = [0u8; 2]; // Read first 2 bytes
    file.read_exact(&mut buffer)?;
    Ok(buffer == [0xFF, 0xD8]) // JPEG starts with FF D8
}

fn main() {
    // Get command-line arguments (e.g., cargo run test.jpg)
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file path. Example: cargo run test.jpg");
        return;
    }

    let file_path = &args[1]; // Use the first argument as the file path
    match is_jpeg(file_path) {
        Ok(true) => println!("{} is a JPEG file!", file_path),
        Ok(false) => println!("{} is not a JPEG file.", file_path),
        Err(e) => println!("Error reading {}: {}", file_path, e),
    }
}
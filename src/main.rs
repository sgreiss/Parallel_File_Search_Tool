use std::env::args;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 3 {
        println!("Usage: {} <text> <location>", args[0]);
        return;
    }

    let text = &args[1];
    let location = &args[2];
    let file_content = read_to_string(location).expect("Failed to read the file");

    if file_content.contains(text) {
        for (index, line) in file_content.lines().enumerate() {
            if line.contains(text) {
                println!("Found '{}' at line {}: {}", text, index + 1, line);
            }
        }
    } else {
        println!("Text not found in the file.");
    }
}

use crate::util::{COLORS, clear_screen, color_str, highlight_text};
use std::env::args;
use std::fs::{read_dir, read_to_string};
use std::path::Path;

mod util;

fn search_dir(dir: &Path, text: &str) -> bool {
    let mut found_case = false;
    if let Ok(entries) = read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    if search_dir(&path, text) && !found_case {
                        found_case = true;
                    }
                } else if path.is_file() {
                    if search_file(&path, text) && !found_case {
                        found_case = true;
                    }
                }
            }
        }
    } else {
        println!(
            "Could not read directory: {}",
            color_str(&dir.display().to_string(), COLORS[0])
        );
    }
    found_case
}

fn search_file(file_path: &Path, text: &str) -> bool {
    let mut found_case = false;
    if let Ok(content) = read_to_string(file_path) {
        for (line_num, line) in content.lines().enumerate() {
            if line.contains(text) {
                if !found_case {
                    found_case = true;
                }

                let path_str = file_path.display().to_string();

                println!(
                    "'{}' found in {} at line #{}: {}",
                    color_str(text, COLORS[3]),
                    color_str(&path_str, COLORS[1]),
                    color_str(&(line_num + 1).to_string(), COLORS[2]),
                    highlight_text(line, text, COLORS[3])
                );
            }
        }
    } else {
        println!(
            "Could not read file: {}",
            color_str(&file_path.display().to_string(), COLORS[0])
        );
    }
    found_case
}

fn main() {
    clear_screen();

    let args: Vec<String> = args().collect();

    if args.len() != 3 {
        println!(
            "Usage: {} {}",
            color_str(&args[0], COLORS[1]),
            color_str("<text> <path>", COLORS[2])
        );

        print!("Actual: {}", color_str(&args[0], COLORS[1]));
        print!(" ");
        for arg in &args[1..] {
            print!("{}", color_str(&format!("{} ", arg), COLORS[0]));
        }
        println!();
        return;
    }

    let text = &args[1];
    let path = Path::new(&args[2]);
    let path_str = path.display().to_string();
    let mut found_case = false;

    if path.is_dir() {
        found_case = search_dir(path, text);
    } else if path.is_file() {
        found_case = search_file(path, text);
    } else {
        println!("Invalid path: {}", color_str(&path_str, COLORS[0]));
    }

    if !found_case {
        println!(
            "No occurrences of '{}' found in {}",
            color_str(text, COLORS[3]),
            color_str(&path_str, COLORS[1])
        );
    }
}

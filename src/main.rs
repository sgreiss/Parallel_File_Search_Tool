use crate::util::{COLORS, clear_screen, color_str, highlight_text};
use std::env::args;
use std::fs::{read_dir, read_to_string};
use std::path::Path;

mod util;

fn check_dir(dir: &Path, text: &str) {
    if let Ok(entries) = read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    check_dir(&path, text);
                } else if path.is_file() {
                    check_file(&path, text);
                }
            }
        }
    }
}

fn check_file(file_path: &Path, text: &str) {
    if let Ok(content) = read_to_string(file_path) {
        if content.contains(text) {
            for line in content.lines() {
                if line.contains(text) {
                    let line_num = content.lines().position(|l| l == line).unwrap() + 1;
                    println!(
                        "'{}' found in {} at line #{: >3}: {}",
                        color_str(text, COLORS[3]),
                        color_str(&file_path.display().to_string(), COLORS[1]),
                        color_str(&line_num.to_string(), COLORS[2]),
                        highlight_text(line, text, COLORS[3])
                    );
                }
            }
        }
    }
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

    if path.is_dir() {
        check_dir(path, text);
    } else if path.is_file() {
        check_file(path, text);
    } else {
        println!(
            "Invalid path: {}",
            color_str(&path.display().to_string(), COLORS[0])
        );
    }
}

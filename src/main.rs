use crate::util::*;
use std::env::args;
use std::fs::{read_dir, read_to_string};
use std::path::Path;
use std::process;
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Instant;

mod util;

fn search_dir(dir: &Path, text: &str, output_flag: &str) -> bool {
    let mut found_case = false;
    if let Ok(entries) = read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    if search_dir(&path, text, output_flag) && !found_case {
                        found_case = true;
                    }
                } else if path.is_file() {
                    if search_file(&path, text, output_flag) && !found_case {
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

fn search_dir_parallel(dir: &Path, text: &str, output_flag: &str, sender: mpsc::Sender<bool>) {
    if let Ok(entries) = read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let sender_clone = sender.clone();
                    let path_clone = path.clone();
                    let text_clone = text.to_string();
                    let output_flag_clone = output_flag.to_string();
                    thread::spawn(move || {
                        search_dir_parallel(
                            &path_clone,
                            &text_clone,
                            &output_flag_clone,
                            sender_clone,
                        );
                    });
                } else if path.is_file() {
                    let found = search_file(&path, text, output_flag);
                    sender.send(found).unwrap();
                }
            }
        }
    } else {
        println!(
            "Could not read directory: {}",
            color_str(&dir.display().to_string(), COLORS[0])
        );
    }
}

fn search_file(file_path: &Path, text: &str, output_flag: &str) -> bool {
    let mut found_case = false;
    if let Ok(content) = read_to_string(file_path) {
        for (line_num, line) in content.lines().enumerate() {
            if line.contains(text) {
                if !found_case {
                    found_case = true;
                }

                let path_str = file_path.display().to_string();

                if output_flag == "n" || (output_flag == "f" && found_case) {
                    continue;
                }

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

    let raw_args: Vec<String> = args().collect();
    let args = process_args(&raw_args).unwrap_or_else(|| {
        process::exit(1);
    });

    let mut flag = String::new();
    if args.len() == 4 {
        flag.push_str(&args[3][1..]);
    }

    let text = &args[1];
    let path = Path::new(&args[2]);
    let path_str = path.display().to_string();
    let mut found_case = false;

    // Single-threaded search

    let single_start_time = Instant::now();

    if path.is_dir() {
        found_case = search_dir(path, text, &flag);
    } else if path.is_file() {
        found_case = search_file(path, text, &flag);
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

    let single_duration = single_start_time.elapsed();
    println!(
        "Single-threaded search completed in {} seconds",
        color_str(&format!("{:.3}", single_duration.as_secs_f64()), COLORS[4])
    );

    // Multi-threaded search

    let multi_start_time = Instant::now();
    let (sender, receiver) = mpsc::channel();
    let text = Arc::new(text.to_string());
    let path = Arc::new(path.to_path_buf());

    let num_threads = num_cpus::get();
    for _ in 0..num_threads {
        let sender = sender.clone();
        let text = Arc::clone(&text);
        let path = Arc::clone(&path);
        let flag = flag.clone();
        thread::spawn(move || {
            search_dir_parallel(&path, &text, &flag, sender);
        });
    }
    drop(sender);

    let mut found_case_multi = false;
    while let Ok(found) = receiver.recv() {
        found_case_multi |= found;
    }

    if !found_case_multi {
        println!(
            "No occurrences of '{}' found in {}",
            color_str(text.as_ref(), COLORS[3]),
            color_str(&path_str, COLORS[1])
        );
    }

    let multi_duration = multi_start_time.elapsed();
    println!(
        "Multi-threaded search completed in {} seconds",
        color_str(&format!("{:.3}", multi_duration.as_secs_f64()), COLORS[4])
    );
}

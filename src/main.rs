use crate::util::*;
use mpsc::Sender;
use mpsc::channel;
use std::env::args;
use std::fs::{read_dir, read_to_string};
use std::path::{Path, PathBuf};
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Instant;

mod util;

fn collect_files(dir: &Path, sender: &Sender<PathBuf>) {
    if let Ok(entries) = read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_files(&path, sender);
            } else if path.is_file() {
                sender.send(path).unwrap();
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

    // Multi-threaded search

    let multi_start_time = Instant::now();

    let (path_sender, path_receiver) = channel::<PathBuf>();
    let (result_sender, result_receiver) = channel::<bool>();

    let path_receiver = Arc::new(Mutex::new(path_receiver));

    let num_threads = num_cpus::get();
    let files_searched = Arc::new(AtomicUsize::new(0));

    for _ in 0..num_threads {
        let path_receiver = Arc::clone(&path_receiver);
        let result_sender = result_sender.clone();
        let files_searched = Arc::clone(&files_searched);
        let text = text.clone();
        let flag = flag.clone();

        thread::spawn(move || {
            loop {
                let file_path = {
                    let lock = path_receiver.lock().unwrap();
                    lock.recv()
                };

                match file_path {
                    Ok(path) => {
                        let found = search_file(&path, &text, &flag);
                        files_searched.fetch_add(1, Ordering::Relaxed);
                        if found {
                            result_sender.send(found).unwrap();
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    }

    if path.is_dir() {
        collect_files(path, &path_sender);
    } else if path.is_file() {
        path_sender.send(path.to_path_buf()).unwrap();
    } else {
        println!("Invalid path: {}", color_str(&path_str, COLORS[0]));
    }

    drop(path_sender);
    drop(result_sender);

    let mut found_case_multi = false;

    while let Ok(found) = result_receiver.recv() {
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
        "Multi-threaded search completed in {} seconds with {} files searched.",
        color_str(&format!("{:.3}", multi_duration.as_secs_f64()), COLORS[4]),
        color_str(&files_searched.load(Ordering::Relaxed).to_string(), COLORS[2])
    );
}

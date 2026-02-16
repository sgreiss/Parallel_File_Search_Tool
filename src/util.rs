use crossterm::{
    execute,
    style::{Color, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::stdout;

pub fn process_args(args: &Vec<String>) -> Option<Vec<String>> {
    if args.len() < 3 || args.len() > 4 {
        println!(
            "Usage: {} {} {}",
            color_str(&args[0], COLORS[1]),
            color_str("<text> <path>", COLORS[2]),
            color_str("[-# print_flag]", COLORS[4])
        );

        print!("Actual: {}", color_str(&args[0], COLORS[1]));
        print!(" ");
        for arg in &args[1..] {
            print!("{}", color_str(&format!("{} ", arg), COLORS[0]));
        }
        println!();
        return None;
    } else if args.len() == 4 && args[3].starts_with('-') {
        let flag = &args[3][1..];
        let valid_flags = ["n", "f"]; // -n for no matches printed, -f for only first match printed
        if !valid_flags.contains(&flag) {
            println!(
                "Invalid print_flag: {}. Expected '{}' or '{}'.",
                color_str(&args[3], COLORS[0]),
                color_str("-n", COLORS[4]),
                color_str("-f", COLORS[4])
            );
            return None;
        }
    }
    Some(args.clone())
}

/// Clears the terminal screen.
pub fn clear_screen() {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All)).unwrap();
}

/// Converts given text to desired color and returns it as a String.
pub fn color_str(text: &str, color: Color) -> String {
    format!(
        "{}{}{}",
        SetForegroundColor(color),
        text,
        SetForegroundColor(Color::Reset)
    )
}

/// Highlight specific substring in a line with a given color.
pub fn highlight_text(line: &str, text: &str, color: Color) -> String {
    line.replace(text, color_str(text, color).as_str())
}

/// A list of colors to use for highlighting found text.
pub const COLORS: [Color; 6] = [
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
];

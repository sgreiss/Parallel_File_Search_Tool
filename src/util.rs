use crossterm::{
    execute,
    style::{Color, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::stdout;

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
    line.replace(
        text,
        color_str(text, color).as_str(),
    )
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

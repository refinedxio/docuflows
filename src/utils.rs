use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::stdout;

pub fn print_error<E: std::fmt::Display>(err: E) {
    execute!(
        stdout(),
        Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::Red),
        Print(format!("Error: {}\n", err)),
        ResetColor
    )
    .unwrap();
}
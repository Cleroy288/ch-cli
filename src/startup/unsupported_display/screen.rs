use std::io::{self, Write};

use crossterm::{
    cursor, execute, terminal,
    style::{
        Color, Print, ResetColor,
        SetForegroundColor,
    },
};

pub fn clear_screen_and_show_header(
    stdout: &mut io::Stdout,
) -> io::Result<()> {
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    writeln!(stdout)?;
    show_title_and_warning(stdout)
}

fn show_title_and_warning(
    stdout: &mut io::Stdout,
) -> io::Result<()> {
    execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print("  rustean"),
        ResetColor,
        Print(" - Semantic Code Indexer\n\n")
    )?;
    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print("  Language Not Supported\n\n"),
        ResetColor
    )
}

pub fn clear_screen(
    stdout: &mut io::Stdout,
) -> io::Result<()> {
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )
}

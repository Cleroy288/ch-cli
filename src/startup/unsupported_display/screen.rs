//! Screen management utilities

use std::io;

use crossterm::{cursor, execute, terminal};

/// Clear screen and move cursor to top
pub fn clear_screen_and_show_header(
    stdout: &mut io::Stdout,
) -> io::Result<()> {
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    println!();
    execute!(
        stdout,
        crossterm::style::SetForegroundColor(
            crossterm::style::Color::Cyan
        ),
        crossterm::style::Print("  ch-cli"),
        crossterm::style::ResetColor,
        crossterm::style::Print(
            " - Semantic Code Indexer\n\n"
        )
    )?;

    execute!(
        stdout,
        crossterm::style::SetForegroundColor(
            crossterm::style::Color::Yellow
        ),
        crossterm::style::Print(
            "  Language Not Supported\n\n"
        ),
        crossterm::style::ResetColor
    )?;

    Ok(())
}

/// Clear screen
pub fn clear_screen(stdout: &mut io::Stdout) -> io::Result<()> {
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    Ok(())
}

//! Screen management utilities

use std::io::{self, Write};

use crossterm::{cursor, execute, terminal};

/// Clear screen and show the unsupported header
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

/// Display the title and warning lines
fn show_title_and_warning(
    stdout: &mut io::Stdout,
) -> io::Result<()> {
    execute!(
        stdout,
        crossterm::style::SetForegroundColor(
            crossterm::style::Color::Cyan
        ),
        crossterm::style::Print("  rustean"),
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
    )
}

/// Clear screen
pub fn clear_screen(
    stdout: &mut io::Stdout,
) -> io::Result<()> {
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    Ok(())
}

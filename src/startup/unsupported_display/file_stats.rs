//! File statistics display

use std::io::{self, Write};

use crossterm::{execute, style::*};

/// Show file statistics
pub fn show_file_stats(
    stdout: &mut io::Stdout,
    supported_count: usize,
    total_count: usize,
) -> io::Result<()> {
    if supported_count > 0 {
        execute!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print(format!(
                "  Found {} supported file(s) \
                out of {} total source files.\n",
                supported_count, total_count
            )),
            Print(
                "  Indexing will be limited to \
                supported files only.\n\n"
            ),
            ResetColor
        )?;
    } else {
        execute!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print(
                "  No supported files found \
                in this codebase.\n"
            ),
            Print(
                "  Semantic indexing will be \
                skipped.\n\n"
            ),
            ResetColor
        )?;
    }

    execute!(
        stdout,
        SetForegroundColor(Color::White),
        Print("  Press any key to continue to the TUI...\n"),
        ResetColor
    )?;
    stdout.flush()?;

    Ok(())
}

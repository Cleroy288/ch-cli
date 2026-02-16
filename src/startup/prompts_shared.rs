//! Shared display helpers for startup prompts.
//!
//! Reused by both update prompts (prompts.rs) and
//! first-launch prompts (prompts_analysis.rs).

use std::io::{self, Write};

use crossterm::{
    cursor, execute,
    style::{
        Color, Print, ResetColor, SetForegroundColor,
    },
    terminal::{self, ClearType},
};

/// Clear screen and show the rustean header
pub(super) fn clear_and_show_header(
    stdout: &mut io::Stdout,
) -> io::Result<()> {
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    writeln!(stdout)?;
    execute!(
        stdout,
        SetForegroundColor(Color::Cyan),
        Print("  rustean"),
        ResetColor,
        Print(" - Semantic Code Indexer\n\n")
    )
}

/// Display Y/N/Q button row with a question label
pub(super) fn display_ynq_buttons(
    stdout: &mut io::Stdout,
    question: &str,
) -> io::Result<()> {
    execute!(
        stdout,
        Print(question),
        SetForegroundColor(Color::White),
        Print("    ["),
        SetForegroundColor(Color::Green),
        Print("Y"),
        SetForegroundColor(Color::White),
        Print("]es  "),
        Print("["),
        SetForegroundColor(Color::Red),
        Print("N"),
        SetForegroundColor(Color::White),
        Print("]o  "),
        Print("["),
        SetForegroundColor(Color::Yellow),
        Print("Q"),
        SetForegroundColor(Color::White),
        Print("]uit\n\n"),
        ResetColor
    )
}

/// Display "Press Y, N, or Q" with loading hint
pub(super) fn display_loading_hint(
    stdout: &mut io::Stdout,
) -> io::Result<()> {
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print("  Press Y, N, or Q: "),
        ResetColor,
        Print("\n"),
        SetForegroundColor(Color::DarkGrey),
        Print(
            "  \u{2504} Loading ML models \
            in background...",
        ),
        ResetColor
    )?;
    stdout.flush()
}

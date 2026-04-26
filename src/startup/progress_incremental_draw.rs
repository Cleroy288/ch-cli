use std::io;

use crossterm::{
    cursor, execute,
    style::{
        Color, Print, ResetColor, SetForegroundColor,
    },
    terminal::{self, ClearType},
};

use super::progress_shared::{
    SPINNER, format_progress_bar, truncate_name,
};

pub(super) fn draw_incremental_bar(
    stdout: &mut io::Stdout,
    processed: usize,
    total: usize,
    spin_idx: usize,
) -> io::Result<()> {
    let (progress_bar, progress) =
        format_progress_bar(processed, total);
    let spinner = SPINNER[spin_idx % SPINNER.len()];

    execute!(
        stdout,
        cursor::MoveTo(0, 5),
        terminal::Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::Yellow),
        Print(format!("  {} ", spinner)),
        SetForegroundColor(Color::Cyan),
        Print("["),
        SetForegroundColor(Color::Yellow),
        Print(&progress_bar),
        SetForegroundColor(Color::Cyan),
        Print("]"),
        SetForegroundColor(Color::White),
        Print(format!(
            " {:.0}%",
            progress * 100.0
        )),
        ResetColor
    )
}

pub(super) fn draw_incremental_stats(
    stdout: &mut io::Stdout,
    processed: usize,
    total: usize,
    current: &str,
) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(0, 7),
        terminal::Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::DarkGrey),
        Print(format!(
            "  Files: {}/{}",
            processed, total
        )),
        ResetColor
    )?;

    let display = truncate_name(current, 50);
    execute!(
        stdout,
        cursor::MoveTo(0, 8),
        terminal::Clear(ClearType::CurrentLine),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("  Current: {}", display)),
        ResetColor
    )
}

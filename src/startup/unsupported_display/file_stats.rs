use std::io::{self, Write};

use crossterm::{
	execute,
	style::{
		Color, Print, ResetColor,
		SetForegroundColor,
	},
};

/// Show file statistics
pub fn show_file_stats(
    stdout: &mut io::Stdout,
    supported_count: usize,
    total_count: usize,
) -> io::Result<()> {
    if supported_count > 0 {
        show_supported_stats(
            stdout, supported_count, total_count,
        )?;
    } else {
        show_no_supported(stdout)?;
    }

    execute!(
        stdout,
        SetForegroundColor(Color::White),
        Print(
            "  Press any key to continue \
            to the TUI...\n",
        ),
        ResetColor
    )?;
    stdout.flush()
}

/// Show stats when supported files exist
fn show_supported_stats(
    stdout: &mut io::Stdout,
    supported_count: usize,
    total_count: usize,
) -> io::Result<()> {
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
            supported files only.\n\n",
        ),
        ResetColor
    )
}

/// Show message when no supported files found
fn show_no_supported(
    stdout: &mut io::Stdout,
) -> io::Result<()> {
    execute!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print(
            "  No supported files found \
            in this codebase.\n",
        ),
        Print(
            "  Semantic indexing will be \
            skipped.\n\n",
        ),
        ResetColor
    )
}

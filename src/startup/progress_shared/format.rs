use std::io;
use std::time::Instant;

use crossterm::{
	cursor, execute,
	style::{
		Color, Print, ResetColor, SetForegroundColor,
	},
	terminal::{self, ClearType},
};

pub(crate) const SPINNER: [&str; 10] = [
	"⠋", "⠙", "⠹", "⠸", "⠼",
	"⠴", "⠦", "⠧", "⠇", "⠏",
];

pub(crate) const BAR_WIDTH: usize = 40;

/// Build a filled/empty progress bar string and its
/// normalized progress ratio.
pub(crate) fn format_progress_bar(
	processed: usize,
	total: usize,
) -> (String, f64) {
	let progress =
		(processed as f64 / total as f64).min(1.0);
	let filled =
		(progress * BAR_WIDTH as f64) as usize;
	let empty = BAR_WIDTH - filled;
	let filled_str = format!(
		"{}{}",
		"█".repeat(filled),
		"░".repeat(empty),
	);
	(filled_str, progress)
}

/// Redraw the "Elapsed: Xs" line at the given row.
pub(crate) fn draw_elapsed_time(
	stdout: &mut io::Stdout,
	start: &Instant,
	row: u16,
) -> io::Result<()> {
	let elapsed = start.elapsed().as_secs();
	execute!(
		stdout,
		cursor::MoveTo(0, row),
		terminal::Clear(ClearType::CurrentLine),
		SetForegroundColor(Color::DarkGrey),
		Print(format!("  Elapsed: {}s", elapsed)),
		ResetColor
	)
}

/// Truncate a name to max_len, prepending "..." when
/// truncation occurs. Safe on UTF-8 char boundaries.
pub(crate) fn truncate_name(
	name: &str,
	max_len: usize,
) -> String {
	if name.len() <= max_len {
		return name.to_string();
	}
	let target = name.len() - (max_len - 3);
	let start = snap_char_boundary_up(name, target);
	format!("...{}", &name[start..])
}

/// Return the next valid char boundary at or after idx.
fn snap_char_boundary_up(s: &str, idx: usize) -> usize {
	if idx >= s.len() {
		return s.len();
	}
	let mut i = idx;
	while i < s.len() && !s.is_char_boundary(i) {
		i += 1;
	}
	i
}

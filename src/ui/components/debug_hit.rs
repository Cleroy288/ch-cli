/// Hit-test code blocks in the debug output panel.
///
/// Uses cached screen-Y ranges computed during render
/// instead of rebuilding markdown on every click.

use ratatui::layout::Rect;
use ratatui::text::Line;

use crate::app::App;

/// Returns the code block index within the latest
/// response, or None if the click missed.
pub(crate) fn hit_test_code_block(
	app: &App,
	_area: Rect,
	row: u16,
) -> Option<usize> {
	if app.is_claude_loading() {
		return None;
	}
	app.last_claude_response()?;
	let ranges = app.block_ranges();
	ranges.iter().position(|(start, end)| {
		row >= *start && row <= *end
	})
}

/// Scan wrapped lines for code block boundaries
/// and return screen-Y ranges for visible blocks.
/// Called during render in debug.rs.
pub(super) fn build_block_ranges(
	lines: &[Line],
	skip: usize,
	height: usize,
	area: Rect,
) -> Vec<(u16, u16)> {
	let blocks = find_all_blocks(lines);
	let end = skip + height;
	blocks
		.into_iter()
		.filter(|(s, e)| *e >= skip && *s < end)
		.map(|(s, e)| {
			let sy = area.y
				+ s.saturating_sub(skip) as u16;
			let ey = area.y
				+ e.saturating_sub(skip)
					.min(height.saturating_sub(1))
					as u16;
			(sy, ey)
		})
		.collect()
}

/// Find all (start_line, end_line) pairs for code
/// blocks in the full wrapped content.
fn find_all_blocks(
	lines: &[Line],
) -> Vec<(usize, usize)> {
	let mut result = Vec::new();
	let mut inside = false;
	let mut start = 0;
	for (i, line) in lines.iter().enumerate() {
		if is_open(line) {
			inside = true;
			start = i;
		}
		if is_close(line) && inside {
			inside = false;
			result.push((start, i));
		}
	}
	result
}

fn is_open(line: &Line) -> bool {
	line.spans.first().is_some_and(|s| {
		s.content.starts_with("  \u{250C}")
	})
}

fn is_close(line: &Line) -> bool {
	line.spans.first().is_some_and(|s| {
		s.content.starts_with("  \u{2514}")
	})
}

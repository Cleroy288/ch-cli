use ratatui::{
	layout::Rect,
	style::Style,
	text::Line,
	Frame,
};

use crate::app::App;
use crate::ui::styles::colors;
use crate::ui::wrap::pre_wrap;

use super::debug_render::{
	build_claude_loading_lines,
	build_claude_response_lines,
	build_claude_streaming_lines,
	build_message_history_lines,
};

/// Types of content for the output panel
#[derive(
	Debug, Clone, Copy, Default, PartialEq, Eq,
)]
pub enum DebugInfoType {
	/// Display parsed message history
	#[default]
	MessageHistory,
	/// Claude response to display
	ClaudeResponse,
	/// Streaming text arriving in real-time
	ClaudeStreaming,
	/// Waiting for Claude CLI response
	ClaudeLoading,
}

impl DebugInfoType {
	pub fn panel_title(&self) -> &'static str {
		match self {
			Self::MessageHistory => " output ",
			Self::ClaudeResponse
			| Self::ClaudeStreaming
			| Self::ClaudeLoading => " claude ",
		}
	}
}

/// Output panel — direct buffer rendering.
///
/// Pre-wraps all lines, computes the visible
/// window, then writes each line directly to
/// the frame buffer. No Paragraph widget.
pub fn render_debug_panel(
	frame: &mut Frame,
	area: Rect,
	app: &App,
) {
	if area.width == 0 || area.height == 0 {
		return;
	}
	let lines = build_stacked_lines(app);
	let wrapped = pre_wrap(lines, area.width);
	let total = wrapped.len();
	let h = area.height as usize;
	let skip = scroll_skip(app, total, h);
	let ranges = super::debug_hit::build_block_ranges(
		&wrapped, skip, h, area,
	);
	app.set_block_ranges(ranges);
	let base = Style::default().fg(colors::DIM_TEXT);
	render_lines(frame, area, &wrapped, skip, base);
}

/// Write visible lines directly to the buffer.
///
/// Clears every cell in the area first, then
/// writes one Line per row with set_line.
fn render_lines(
	frame: &mut Frame,
	area: Rect,
	lines: &[Line<'_>],
	skip: usize,
	base: Style,
) {
	let buf = frame.buffer_mut();
	let h = area.height as usize;
	let w = area.width;
	// Clear entire area
	buf.set_style(area, Style::reset());
	buf.set_style(area, base);
	// Write each visible line
	for i in 0..h {
		let y = area.y + i as u16;
		if let Some(line) = lines.get(skip + i) {
			buf.set_line(area.x, y, line, w);
		}
	}
}

/// How many wrapped lines to skip.
fn scroll_skip(
	app: &App,
	total: usize,
	height: usize,
) -> usize {
	let max = total.saturating_sub(height);
	if app.is_claude_loading() {
		return max;
	}
	(app.scroll_offset() as usize).min(max)
}

fn build_stacked_lines(
	app: &App,
) -> Vec<Line<'static>> {
	let mut lines =
		build_message_history_lines(app.history());
	lines.extend(build_claude_section(app));
	lines
}

fn build_claude_section(
	app: &App,
) -> Vec<Line<'static>> {
	if !app.is_claude_loading() {
		return build_claude_response_lines(
			app.last_claude_response(),
		);
	}
	if !app.streaming_text().is_empty() {
		return build_claude_streaming_lines(
			app.streaming_text(),
			app.tool_status(),
		);
	}
	build_claude_loading_lines(app.tool_status())
}

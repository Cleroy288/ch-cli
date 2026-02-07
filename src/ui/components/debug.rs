use ratatui::{
	layout::{Alignment, Rect},
	style::Style,
	text::Line,
	widgets::{Block, Borders, Paragraph},
	Frame,
};

use crate::app::App;
use crate::ui::styles::colors;

use super::debug_render::build_message_history_lines;

/// Types of debug information for the debug panel.
///
/// To add a new debug type:
/// 1. Add a variant to this enum
/// 2. Add a match arm in `build_debug_lines()`
/// 3. Implement a `build_*_lines()` in debug_render
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugInfoType {
	/// Display parsed message history
	MessageHistory,
}

impl Default for DebugInfoType {
	fn default() -> Self {
		Self::MessageHistory
	}
}

impl DebugInfoType {
	/// Get a human-readable name for this type
	pub fn name(&self) -> &'static str {
		match self {
			Self::MessageHistory => {
				"Message History"
			}
		}
	}
}

/// Render the debug section.
///
/// Main rendering function that coordinates building
/// and displaying debug information.
pub fn render_debug_panel(
	frame: &mut Frame,
	area: Rect,
	app: &App,
) {
	let debug_type = DebugInfoType::default();
	let lines = build_debug_lines(app, debug_type);
	let widget = create_debug_panel_widget(lines);
	frame.render_widget(widget, area);
}

/// Build debug lines based on selected debug type.
///
/// Routes to the appropriate builder function.
fn build_debug_lines(
	app: &App,
	debug_type: DebugInfoType,
) -> Vec<Line<'static>> {
	match debug_type {
		DebugInfoType::MessageHistory => {
			build_message_history_lines(
				app.history(),
			)
		}
	}
}

/// Create the debug panel widget with given lines.
fn create_debug_panel_widget(
	lines: Vec<Line<'static>>,
) -> Paragraph<'static> {
	Paragraph::new(lines)
		.block(
			Block::default()
				.borders(Borders::ALL)
				.title(" Debug Section ")
				.title_alignment(Alignment::Left)
				.style(
					Style::default()
						.fg(colors::DEBUG),
				),
		)
		.scroll((0, 0))
}


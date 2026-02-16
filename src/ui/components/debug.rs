use ratatui::{
	layout::{Alignment, Rect},
	style::Style,
	text::Line,
	widgets::{Block, Borders, Paragraph, Wrap},
	Frame,
};

use crate::app::App;
use crate::ui::strings::tui_labels;
use crate::ui::styles::colors;

use super::debug_render::{
	build_claude_loading_lines,
	build_claude_response_lines,
	build_doc_preview_lines,
	build_message_history_lines,
};

/// Types of content for the output panel.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DebugInfoType {
	/// Display parsed message history
	#[default]
	MessageHistory,
	/// Documentation for a selected symbol
	DocPreview,
	/// Doc fetch completed, no docs found
	DocNotFound,
	/// Claude response to display
	ClaudeResponse,
	/// Waiting for Claude CLI response
	ClaudeLoading,
}

impl DebugInfoType {
	/// Get the panel title for this content type
	pub fn panel_title(&self) -> &'static str {
		match self {
			Self::MessageHistory => {
				tui_labels::PANEL_OUTPUT
			}
			Self::ClaudeResponse
			| Self::ClaudeLoading => {
				tui_labels::PANEL_CLAUDE
			}
			_ => tui_labels::PANEL_DOC_PREVIEW,
		}
	}
}

/// Render the output panel.
///
/// Shows doc preview/loading/not-found when a
/// symbol fetch is active, otherwise message history.
pub fn render_debug_panel(
	frame: &mut Frame,
	area: Rect,
	app: &App,
) {
	let info_type = detect_info_type(app);
	let lines = build_debug_lines(app, info_type);
	let title = info_type.panel_title();
	let scroll = app.scroll_offset();
	let widget =
		create_panel_widget(lines, title, scroll);
	frame.render_widget(widget, area);
}

/// Detect which content type to show.
fn detect_info_type(app: &App) -> DebugInfoType {
	if app.is_claude_loading() {
		return DebugInfoType::ClaudeLoading;
	}
	if app.last_claude_response().is_some() {
		return DebugInfoType::ClaudeResponse;
	}
	if app.doc_preview().is_some() {
		return DebugInfoType::DocPreview;
	}
	if app.doc_fetch_no_result() {
		return DebugInfoType::DocNotFound;
	}
	DebugInfoType::MessageHistory
}

/// Build lines based on selected content type.
fn build_debug_lines(
	app: &App,
	info_type: DebugInfoType,
) -> Vec<Line<'static>> {
	match info_type {
		DebugInfoType::MessageHistory => {
			build_message_history_lines(
				app.history(),
			)
		}
		DebugInfoType::DocPreview => {
			build_doc_preview_lines(
				app.doc_preview(),
			)
		}
		DebugInfoType::DocNotFound => {
			build_doc_preview_lines(None)
		}
		DebugInfoType::ClaudeResponse => {
			build_claude_response_lines(
				app.last_claude_response(),
			)
		}
		DebugInfoType::ClaudeLoading => {
			build_claude_loading_lines()
		}
	}
}

/// Create the output panel widget with given lines.
fn create_panel_widget(
	lines: Vec<Line<'static>>,
	title: &str,
	scroll: u16,
) -> Paragraph<'static> {
	Paragraph::new(lines)
		.block(
			Block::default()
				.borders(Borders::ALL)
				.border_style(
					Style::default()
						.fg(colors::BORDER),
				)
				.title(title.to_string())
				.title_alignment(Alignment::Left)
				.title_style(
					Style::default()
						.fg(colors::DIM_TEXT),
				),
		)
		.wrap(Wrap { trim: false })
		.scroll((scroll, 0))
}

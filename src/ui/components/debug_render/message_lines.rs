use ratatui::{
	style::{Modifier, Style},
	text::{Line, Span},
};

use crate::message::UserMessage;
use crate::ui::markdown::render_markdown;
use crate::ui::styles::colors;

use super::claude_chrome;

pub(super) fn build_message_lines(
	message: &UserMessage,
) -> Vec<Line<'static>> {
	let mut lines = vec![
		user_role_line(),
		content_line(message),
		Line::from(""),
	];
	if let Some(ref resp) = message.response {
		lines.extend(response_lines(resp));
	}
	lines
}

/// User role label with glyph.
fn user_role_line() -> Line<'static> {
	Line::from(Span::styled(
		"  \u{25C6} You",
		Style::default()
			.fg(colors::USER_LABEL)
			.add_modifier(Modifier::BOLD),
	))
}

/// Message content line
fn content_line(
	message: &UserMessage,
) -> Line<'static> {
	Line::from(Span::styled(
		format!("  {}", message.raw_input),
		Style::default().fg(colors::INPUT_TEXT),
	))
}

fn response_lines(
	text: &str,
) -> Vec<Line<'static>> {
	let mut lines = vec![
		Line::from(""),
		claude_chrome::role_line(),
	];
	lines.extend(render_markdown(text));
	lines.push(Line::from(""));
	lines
}

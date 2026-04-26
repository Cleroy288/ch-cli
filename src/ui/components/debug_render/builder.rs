use ratatui::style::Style;
use ratatui::text::{Line, Span};

use crate::message::ConversationHistory;
use crate::ui::styles::colors;

use super::message_lines::build_message_lines;

#[doc(hidden)]
pub fn build_message_history_lines(
	history: &ConversationHistory,
) -> Vec<Line<'static>> {
	if history.is_empty() {
		return build_empty_lines();
	}
	build_history_lines(history)
}

fn build_empty_lines() -> Vec<Line<'static>> {
	vec![
		Line::from(""),
		Line::from(Span::styled(
			"  No messages yet. Type something \
			and press Enter!",
			Style::default().fg(colors::PLACEHOLDER),
		)),
	]
}

fn build_history_lines(
	history: &ConversationHistory,
) -> Vec<Line<'static>> {
	use crate::domain::HISTORY_DISPLAY_COUNT;
	use crate::message::UserMessage;

	let mut lines = Vec::new();
	let msgs: Vec<&UserMessage> = history
		.messages()
		.iter()
		.rev()
		.take(HISTORY_DISPLAY_COUNT)
		.collect();

	for msg in msgs.iter().rev() {
		lines.extend(build_message_lines(msg));
	}
	lines
}

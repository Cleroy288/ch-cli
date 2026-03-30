use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use crate::domain::claude::{
	ClaudeResponse, ToolActivity,
};
use crate::ui::components::spinner;
use crate::ui::markdown::render_markdown;
use crate::ui::styles::colors;

use super::claude_chrome as chrome;

pub fn build_claude_response_lines(
	response: Option<&ClaudeResponse>,
) -> Vec<Line<'static>> {
	let Some(resp) = response else {
		return Vec::new();
	};
	vec![chrome::footer_line(resp)]
}

pub fn build_claude_loading_lines(
	tool: Option<&ToolActivity>,
) -> Vec<Line<'static>> {
	let activity = match tool {
		Some(act) => chrome::tool_line(act),
		None => chrome::thinking_line(),
	};
	vec![chrome::role_line(), activity]
}

pub fn build_claude_streaming_lines(
	text: &str,
	tool: Option<&ToolActivity>,
) -> Vec<Line<'static>> {
	let mut lines = vec![chrome::role_line()];
	if let Some(act) = tool {
		lines.push(chrome::tool_line(act));
		lines.push(Line::from(""));
	}
	lines.extend(render_markdown(text));
	lines.push(Line::from(Span::styled(
		format!("  {}", spinner::spinner_char()),
		Style::default()
			.fg(colors::TOOL_BLUE)
			.add_modifier(Modifier::BOLD),
	)));
	lines
}

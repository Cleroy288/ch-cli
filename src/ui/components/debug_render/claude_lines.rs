//! Claude response rendering for the output panel.
//!
//! Converts a ClaudeResponse into styled lines for
//! display using markdown rendering, or shows a
//! loading indicator.

use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use crate::domain::claude::ClaudeResponse;
use crate::ui::markdown::render_markdown;
use crate::ui::styles::colors;

/// Loading indicator text
const LOADING_TEXT: &str = "Thinking...";
/// Tokens label prefix
const TOKENS_LABEL: &str = "tokens:";
/// Duration label prefix
const DURATION_LABEL: &str = "time:";
/// Turns label prefix
const TURNS_LABEL: &str = "turns:";

/// Build styled lines for a Claude response.
///
/// Parses the result as markdown for proper
/// rendering of code blocks, bold, lists, etc.
pub fn build_claude_response_lines(
	response: Option<&ClaudeResponse>,
) -> Vec<Line<'static>> {
	let Some(resp) = response else {
		return Vec::new();
	};

	let mut lines = vec![Line::from("")];
	lines.extend(render_markdown(&resp.result));
	lines.push(Line::from(""));
	lines.push(build_footer_line(resp));
	lines
}

/// Build loading indicator lines.
pub fn build_claude_loading_lines(
) -> Vec<Line<'static>> {
	vec![
		Line::from(""),
		Line::from(Span::styled(
			format!("  {}", LOADING_TEXT),
			Style::default()
				.fg(colors::AMBER)
				.add_modifier(Modifier::BOLD),
		)),
	]
}

/// Build the dim footer with usage stats
fn build_footer_line(
	resp: &ClaudeResponse,
) -> Line<'static> {
	let total_tokens = resp.usage.input_tokens
		+ resp.usage.output_tokens;
	let duration_secs =
		resp.duration_ms as f64 / 1000.0;

	let text = format!(
		"  {} {} | {} {} | {} {:.1}s",
		TOKENS_LABEL,
		total_tokens,
		TURNS_LABEL,
		resp.num_turns,
		DURATION_LABEL,
		duration_secs,
	);
	Line::from(Span::styled(
		text,
		Style::default().fg(colors::DIM_TEXT),
	))
}

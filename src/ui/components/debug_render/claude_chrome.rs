use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use crate::domain::claude::{
	ClaudeResponse, ToolActivity,
};
use crate::ui::components::spinner;
use crate::ui::styles::colors;

/// `  ✦ Assistant`
pub(super) fn role_line() -> Line<'static> {
	Line::from(Span::styled(
		"  \u{2726} Assistant",
		Style::default()
			.fg(colors::TOOL_BLUE)
			.add_modifier(Modifier::BOLD),
	))
}

/// `  ● Read src/file.rs`
pub(super) fn tool_line(
	tool: &ToolActivity,
) -> Line<'static> {
	let spin = spinner::spinner_char();
	let accent =
		Style::default().fg(colors::TOOL_BLUE);
	let bold = Style::default()
		.fg(colors::TOOL_NAME)
		.add_modifier(Modifier::BOLD);
	let dim =
		Style::default().fg(colors::DIM_TEXT);
	if tool.summary.is_empty() {
		return Line::from(vec![
			Span::styled(
				format!("  {} ", spin), accent,
			),
			Span::styled(
				format!("{}...", tool.tool_name),
				bold,
			),
		]);
	}
	Line::from(vec![
		Span::styled(
			format!("  {} ", spin), accent,
		),
		Span::styled(
			tool.tool_name.clone(), bold,
		),
		Span::styled(" ", dim),
		Span::styled(tool.summary.clone(), dim),
	])
}

pub(super) fn thinking_line() -> Line<'static> {
	Line::from(Span::styled(
		format!(
			"  {}", spinner::thinking_label()
		),
		Style::default()
			.fg(colors::TOOL_BLUE)
			.add_modifier(Modifier::BOLD),
	))
}

/// Response metadata block: separator + stats.
pub(super) fn meta_lines(
	resp: &ClaudeResponse,
) -> Vec<Line<'static>> {
	let dim = Style::default().fg(colors::DIM_TEXT);
	let sep = Style::default().fg(colors::SEPARATOR);
	let ok =
		Style::default().fg(colors::SUCCESS);
	let mut spans = vec![
		Span::styled("  \u{2713} ", ok),
		Span::styled(
			format!(
				"\u{2191}{}", resp.usage.input_tokens,
			),
			dim,
		),
		Span::styled("  ", dim),
		Span::styled(
			format!(
				"\u{2193}{}", resp.usage.output_tokens,
			),
			dim,
		),
		Span::styled("  \u{00b7}  ", sep),
		Span::styled(
			format!(
				"{:.1}s",
				resp.duration_ms as f64 / 1000.0,
			),
			dim,
		),
	];
	if let Some(cost) = resp.cost_usd {
		spans.push(Span::styled(
			"  \u{00b7}  ", sep,
		));
		spans.push(Span::styled(
			format!("${:.4}", cost), dim,
		));
	}
	vec![
		Line::from(""),
		Line::from(spans),
		Line::from(""),
	]
}

/// Build display lines for the preflight panel.

use ratatui::{
	style::Style,
	text::{Line, Span},
};

use crate::domain::preflight::PreFlightData;
use crate::ui::strings::tui_labels;
use crate::ui::styles::colors;

/// Max characters of main prompt to show.
const PROMPT_TRUNCATE: usize = 60;

/// Dim label style reused across all lines.
const DIM: Style = Style::new().fg(colors::DIM_TEXT);

/// Build all lines for the preflight recap.
pub fn build_lines(
	data: &PreFlightData,
) -> Vec<Line<'static>> {
	let mut lines = Vec::new();
	lines.push(prompt_line(&data.main_prompt));
	lines.push(mode_line(&data.mode_label));
	lines.push(counts_line(
		data.file_count,
		data.symbol_count,
	));
	for agent in &data.agents {
		lines.push(agent_line(agent));
	}
	lines.push(Line::from(Span::styled(
		tui_labels::PREFLIGHT_FOOTER.to_string(),
		DIM,
	)));
	lines
}

/// Truncated main prompt line.
fn prompt_line(text: &str) -> Line<'static> {
	let display = if text.len() > PROMPT_TRUNCATE {
		format!("{}...", &text[..PROMPT_TRUNCATE])
	} else {
		text.to_string()
	};
	Line::from(vec![
		Span::styled(
			tui_labels::LABEL_PROMPT.to_string(),
			DIM,
		),
		Span::styled(
			display,
			Style::new().fg(colors::TEXT_LIGHT),
		),
	])
}

/// Query mode display line.
fn mode_line(label: &str) -> Line<'static> {
	Line::from(vec![
		Span::styled(
			tui_labels::LABEL_MODE.to_string(),
			DIM,
		),
		Span::styled(
			label.to_string(),
			Style::new().fg(colors::ACCENT),
		),
	])
}

/// File/symbol count line.
fn counts_line(
	files: usize,
	symbols: usize,
) -> Line<'static> {
	Line::from(vec![
		Span::styled(
			tui_labels::LABEL_FILES.to_string(),
			DIM,
		),
		Span::styled(
			format!("{files}  Symbols: {symbols}"),
			Style::new().fg(colors::TEXT_LIGHT),
		),
	])
}

/// Single agent line.
fn agent_line(
	agent: &crate::domain::agent_spec::AgentSpec,
) -> Line<'static> {
	Line::from(vec![
		Span::styled(
			tui_labels::AGENT_OPEN.to_string(),
			DIM,
		),
		Span::styled(
			agent.model.clone(),
			Style::new().fg(colors::ACCENT),
		),
		Span::styled(
			tui_labels::AGENT_SEP.to_string(),
			DIM,
		),
		Span::styled(
			agent.description.clone(),
			Style::new().fg(colors::TEXT_LIGHT),
		),
	])
}

use ratatui::{
	layout::{Constraint, Layout, Rect},
	style::{Color, Style},
	text::{Line, Span},
	widgets::Paragraph,
	Frame,
};

use crate::app::App;
use crate::ui::styles::colors;

/// Right-column width for the context bar.
const CONTEXT_WIDTH: u16 = 17;
/// Bar width in block characters.
const BAR_WIDTH: usize = 10;
const BLOCK_FULL: &str = "\u{2588}";
const BLOCK_EMPTY: &str = "\u{2591}";
/// Dim color for empty bar blocks.
const BAR_DIM: Color = Color::Rgb(69, 71, 90);

/// Status bar — left: model info or message;
/// right: context bar (always visible).
pub fn render_status_line(
	frame: &mut Frame,
	area: Rect,
	app: &App,
) {
	if area.width < 10 {
		return;
	}
	let [left, right] = Layout::horizontal([
		Constraint::Fill(1),
		Constraint::Length(CONTEXT_WIDTH),
	])
	.areas(area);
	let left_line = build_left_line(app);
	frame.render_widget(
		Paragraph::new(left_line), left,
	);
	render_context_bar(frame, right, app);
}

/// Left side: status message if set, else model + stats.
fn build_left_line(app: &App) -> Line<'static> {
	if let Some(msg) = app.status_message() {
		return Line::from(Span::styled(
			format!(" {}", msg),
			Style::default().fg(colors::ACCENT_DIM),
		));
	}
	let mut spans = build_model_spans(app);
	append_stats(&mut spans, app);
	Line::from(spans)
}

/// Right side: context usage bar, always rendered.
fn render_context_bar(
	frame: &mut Frame,
	area: Rect,
	app: &App,
) {
	let pct = app.context_percent().unwrap_or(0);
	let filled = (pct as usize * BAR_WIDTH) / 100;
	let empty = BAR_WIDTH.saturating_sub(filled);
	let color = match pct {
		0..=50 => Color::Rgb(166, 227, 161),
		51..=75 => Color::Rgb(249, 226, 175),
		_ => Color::Rgb(243, 139, 168),
	};
	let spans = vec![
		Span::raw(" "),
		Span::styled(
			BLOCK_FULL.repeat(filled),
			Style::default().fg(color),
		),
		Span::styled(
			BLOCK_EMPTY.repeat(empty),
			Style::default().fg(BAR_DIM),
		),
		Span::styled(
			format!(" {}%", pct),
			Style::default().fg(color),
		),
	];
	frame.render_widget(
		Paragraph::new(Line::from(spans)), area,
	);
}

/// Backend · model · effort spans.
fn build_model_spans(
	app: &App,
) -> Vec<Span<'static>> {
	let dim =
		Style::default().fg(colors::STATUS_DIM);
	let accent =
		Style::default().fg(colors::ACCENT_DIM);
	let sep =
		Style::default().fg(colors::SEPARATOR);
	vec![
		Span::styled(" ", dim),
		Span::styled(
			app.backend_name().to_owned(), dim,
		),
		Span::styled(" \u{00b7} ", sep),
		Span::styled(
			app.model_name().to_owned(), accent,
		),
		Span::styled(" \u{00b7} ", sep),
		Span::styled(
			app.effort_level().to_owned(), dim,
		),
	]
}

/// Append last response stats to spans.
fn append_stats(
	spans: &mut Vec<Span<'static>>,
	app: &App,
) {
	let Some(resp) =
		app.last_claude_response()
	else {
		return;
	};
	let dim =
		Style::default().fg(colors::STATUS_DIM);
	let sep =
		Style::default().fg(colors::SEPARATOR);
	let total = resp.usage.input_tokens
		+ resp.usage.output_tokens;
	let secs =
		resp.duration_ms as f64 / 1000.0;
	spans.push(Span::styled(
		"  \u{00b7}  ", sep,
	));
	spans.push(Span::styled(
		format!("{} tokens", total), dim,
	));
	spans.push(Span::styled(
		"  \u{00b7}  ", sep,
	));
	spans.push(Span::styled(
		format!("{:.1}s", secs), dim,
	));
	if let Some(cost) = resp.cost_usd {
		spans.push(Span::styled(
			"  \u{00b7}  ", sep,
		));
		spans.push(Span::styled(
			format!("${:.4}", cost), dim,
		));
	}
}

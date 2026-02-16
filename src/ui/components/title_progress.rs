//! Progress indicator for title bar bottom-right.
//!
//! Builds a styled Span showing doc generation
//! progress percentage or "done" status.

use ratatui::style::Style;
use ratatui::text::Span;

use crate::app::doc_progress::DocProgressState;
use crate::ui::strings::tui_labels::{
	DOC_PROGRESS_DONE, DOC_PROGRESS_FMT,
};
use crate::ui::styles::colors;

/// Build a progress Span for the title bar.
///
/// Returns None if progress is not visible.
pub fn build_progress_span(
	progress: &DocProgressState,
) -> Option<Span<'static>> {
	if !progress.is_visible() {
		return None;
	}
	let text = format_progress_text(progress);
	let style = progress_style(progress);
	Some(Span::styled(text, style))
}

/// Format the progress text string.
///
/// Shows percentage while running, "done" when complete.
fn format_progress_text(
	state: &DocProgressState,
) -> String {
	if state.in_progress {
		return DOC_PROGRESS_FMT
			.replace("{}", &state.percent().to_string());
	}
	DOC_PROGRESS_DONE.to_string()
}

/// Choose style based on progress state.
///
/// Amber while generating, dim when done.
fn progress_style(
	state: &DocProgressState,
) -> Style {
	if state.in_progress {
		Style::default().fg(colors::AMBER)
	} else {
		Style::default().fg(colors::DIM_TEXT)
	}
}

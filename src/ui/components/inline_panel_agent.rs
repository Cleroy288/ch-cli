/// Render agent header above code blocks.

use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::Frame;

use crate::review::InlineBlock;
use crate::ui::styles::colors;

/// Render agent source label at the given y.
/// Returns the number of rows consumed (0 or 1).
pub(super) fn render_agent_header(
	frame: &mut Frame,
	area: Rect,
	y: u16,
	bottom: u16,
	block: &InlineBlock,
	skip: u16,
) -> (u16, u16) {
	let Some(ref agent) = block.agent_source else {
		return (y, skip);
	};
	if skip > 0 {
		return (y, skip - 1);
	}
	if y >= bottom {
		return (y, 0);
	}
	let line = agent_line(agent);
	let buf = frame.buffer_mut();
	buf.set_line(area.x, y, &line, area.width);
	(y + 1, 0)
}

/// Build styled line: `[model] description`
fn agent_line(model: &str) -> Line<'static> {
	Line::from(vec![Span::styled(
		format!("  [{}]", model),
		Style::default().fg(colors::ACCENT_DIM),
	)])
}

/// Height contribution of agent header (0 or 1).
pub(super) fn agent_header_height(
	block: &InlineBlock,
) -> u16 {
	if block.agent_source.is_some() { 1 } else { 0 }
}

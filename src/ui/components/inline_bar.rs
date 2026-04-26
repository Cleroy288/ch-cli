use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::Frame;

use crate::domain::review::BlockIdx;
use crate::review::InlineBlocks;
use crate::ui::styles::colors;

const KEY: Style = Style::new()
	.fg(colors::FOCUS)
	.add_modifier(Modifier::BOLD);

const DESC: Style =
	Style::new().fg(colors::DEBUG);

pub fn render_inline_bar(
	frame: &mut Frame,
	rect: Rect,
	inline: &InlineBlocks,
) {
	let line = if let Some(idx) =
		inline.impl_step()
	{
		implement_bar(inline, idx)
	} else {
		normal_bar(inline)
	};
	let buf = frame.buffer_mut();
	buf.set_line(rect.x, rect.y, &line, rect.width);
}

fn normal_bar(
	inline: &InlineBlocks,
) -> Line<'static> {
	if !inline.is_implementation() {
		return Line::from(Span::styled(
			" Click a code block to edit", DESC,
		));
	}
	Line::from(vec![
		Span::styled(
			" Click to edit  ", DESC,
		),
		Span::styled("[i]", KEY),
		Span::styled(" Implement", DESC),
	])
}

fn implement_bar(
	inline: &InlineBlocks,
	idx: BlockIdx,
) -> Line<'static> {
	let step = inline.impl_step_display();
	let total = inline.writable_count();
	let block = inline.blocks().get(idx.val());
	let path = block
		.and_then(|b| b.file_path.as_deref())
		.unwrap_or("(no path)");
	let agent_tag = block
		.and_then(|b| b.agent_source.as_deref())
		.map(|a| format!(" [{}]", a))
		.unwrap_or_default();
	let label = format!(
		" Step {}/{}: {}{}  ",
		step, total, path, agent_tag,
	);
	Line::from(vec![
		Span::styled(label, DESC),
		Span::styled("[Enter]", KEY),
		Span::styled(" Execute  ", DESC),
		Span::styled("[s]", KEY),
		Span::styled(" Skip  ", DESC),
		Span::styled("[Esc]", KEY),
		Span::styled(" Cancel", DESC),
	])
}


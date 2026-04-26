use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;

use crate::domain::review::BlockIdx;
use crate::review::{
	InlineBlock, InlineBlocks, InlineFocus,
};
use crate::ui::markdown::code_block;
use crate::ui::markdown::code_metrics::LineCount;
use crate::ui::styles::colors;

use super::inline_blocks;
use super::inline_panel::section_rect;
use super::inline_panel_agent;

/// Render a full code section (frame + question
/// + answer), skipping the first `skip` lines
/// that are scrolled off the top.
pub(super) fn render_code_section(
	frame: &mut Frame,
	area: Rect,
	y: u16,
	bottom: u16,
	idx: BlockIdx,
	inline: &InlineBlocks,
	skip: u16,
	logical_y: u16,
	full_h: u16,
) -> u16 {
	let Some(block) =
		inline.blocks().get(idx.val())
	else {
		return y;
	};
	let (mut cur_y, mut sk) = (y, skip);
	(cur_y, sk) =
		inline_panel_agent::render_agent_header(
			frame, area, cur_y, bottom,
			block, sk,
		);
	let frame_h = block.edited.line_count()
		+ code_block::FRAME_ROWS;
	if sk < frame_h {
		cur_y = render_block_frame(
			frame, area, cur_y, bottom,
			inline, idx, sk,
			logical_y, full_h,
		);
		sk = 0;
	} else {
		sk -= frame_h;
	}
	cur_y = render_tail(
		frame, area, cur_y, bottom,
		block, inline, idx, sk,
	);
	cur_y
}

/// Question input + answer, with skip applied.
fn render_tail(
	frame: &mut Frame,
	area: Rect,
	y: u16,
	bottom: u16,
	block: &InlineBlock,
	inline: &InlineBlocks,
	idx: BlockIdx,
	skip: u16,
) -> u16 {
	let mut cur_y = y;
	let mut sk = skip;

	if !inline.is_frozen() {
		if sk > 0 {
			sk -= 1;
		} else if cur_y < bottom {
			let r = section_rect(
				area, cur_y, 1, bottom,
			);
			let focused = matches!(
				inline.focus(),
				Some(InlineFocus::Question(i))
					if i == idx
			);
			inline_blocks::render_question_input(
				frame, r, &block.question,
				focused,
			);
			cur_y += 1;
		}
	}

	if let Some(ref ans) = block.answer {
		let h = ans.line_count();
		let vis = h.saturating_sub(sk);
		if vis > 0 && cur_y < bottom {
			let r = section_rect(
				area, cur_y, vis, bottom,
			);
			inline_blocks::render_answer(
				frame, r, ans, sk,
			);
			cur_y +=
				vis.min(bottom.saturating_sub(cur_y));
		}
	}
	cur_y
}

/// Code block frame (border + code lines),
/// rendering from line `skip` onward.
fn render_block_frame(
	frame: &mut Frame,
	area: Rect,
	y: u16,
	bottom: u16,
	inline: &InlineBlocks,
	idx: BlockIdx,
	skip: u16,
	logical_y: u16,
	section_h: u16,
) -> u16 {
	let block = &inline.blocks()[idx.val()];
	let focused = matches!(
		inline.focus(),
		Some(InlineFocus::CodeBlock(i))
			if i == idx
	);
	let full_h = block.edited.line_count()
		+ code_block::FRAME_ROWS;
	let vis_h = full_h.saturating_sub(skip);
	let rect = section_rect(
		area, y, vis_h, bottom,
	);
	let logical = Rect::new(
		area.x, logical_y,
		area.width, section_h,
	);
	block.render_rect.set(logical);

	render_static_header(
		frame, rect, block, focused, skip,
	);
	if focused && !inline.is_frozen() {
		inline_blocks::overlay_cursor(
			frame, rect, block, skip,
		);
	}
	y + vis_h
}

/// Block widget (left border) + code lines,
/// starting from line `skip`.
fn render_static_header(
	frame: &mut Frame,
	rect: Rect,
	block: &InlineBlock,
	focused: bool,
	skip: u16,
) {
	let color = if focused {
		colors::FOCUS
	} else {
		colors::DEBUG
	};
	let wrapper = build_wrapper(
		block, color, skip,
	);
	let inner = wrapper.inner(rect);
	frame.render_widget(wrapper, rect);

	let cached = block.rendered_code_lines();
	let mut lines = Vec::with_capacity(
		2 + cached.len(),
	);
	lines.push(
		code_block::render_open(&block.lang),
	);
	lines.extend(cached.iter().cloned());
	lines.push(code_block::render_close());

	let buf = frame.buffer_mut();
	let sk = skip as usize;
	for i in 0..inner.height as usize {
		let y = inner.y + i as u16;
		if let Some(line) = lines.get(sk + i) {
			buf.set_line(
				inner.x, y, line, inner.width,
			);
		}
	}
}

fn build_wrapper(
	block: &InlineBlock,
	color: ratatui::style::Color,
	skip: u16,
) -> Block<'static> {
	let base = Block::default()
		.borders(Borders::LEFT)
		.border_style(
			Style::default().fg(color),
		);
	if skip > 0 {
		return base;
	}
	let header = block
		.file_path
		.as_deref()
		.unwrap_or("(no file)");
	base.title(Span::styled(
		format!(" {} ", header),
		Style::default().fg(color),
	))
}

pub(super) fn code_section_height(
	inline: &InlineBlocks,
	idx: BlockIdx,
) -> u16 {
	let Some(block) =
		inline.blocks().get(idx.val())
	else {
		return 0;
	};
	let mut h = block.edited.line_count()
		+ code_block::FRAME_ROWS
		+ inline_panel_agent::agent_header_height(
			block,
		);
	if !inline.is_frozen() {
		h += 1;
	}
	if let Some(ref ans) = block.answer {
		h += ans.line_count();
	}
	h
}

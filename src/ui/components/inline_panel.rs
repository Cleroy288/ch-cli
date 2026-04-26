use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::Frame;

use crate::domain::review::BlockIdx;
use crate::review::InlineBlocks;
use crate::ui::markdown::code_metrics::{
	LineCount, SatU16,
};
use crate::ui::markdown::sections::ContentSection;

use super::inline_bar;
use super::inline_blocks;
use super::inline_panel_code;
use super::inline_panel_scroll;

pub(super) enum SectionSlot<'a> {
	Text { text: &'a str, height: u16 },
	Code { idx: BlockIdx, height: u16 },
}

/// Shared by render, hit-test, and scroll height
pub(super) fn section_layout<'a>(
	inline: &'a InlineBlocks,
) -> Vec<SectionSlot<'a>> {
	let mut ci: usize = 0;
	inline
		.cached_sections()
		.iter()
		.map(|s| match s {
			ContentSection::Text(text) => {
				let h = text.line_count();
				SectionSlot::Text {
					text: text.as_str(),
					height: h,
				}
			}
			ContentSection::Code { .. } => {
				let bi = BlockIdx(ci);
				let h = inline_panel_code
					::code_section_height(
						inline, bi,
					);
				let slot = SectionSlot::Code {
					idx: bi,
					height: h,
				};
				ci += 1;
				slot
			}
		})
		.collect()
}

pub fn render_inline_panel(
	frame: &mut Frame,
	area: Rect,
	inline: &InlineBlocks,
) {
	let [content, bar] = Layout::vertical([
		Constraint::Fill(1),
		Constraint::Length(1),
	])
	.areas(area);

	inline.set_content_area(content);
	let layout = section_layout(inline);
	let scrollable = prepare_scroll(
		&layout, content.height, inline,
	);
	render_content(
		frame, content, inline, scrollable,
		&layout,
	);
	if scrollable {
		let total_h = total_height(&layout);
		inline_panel_scroll::render_scrollbar(
			frame, content, inline,
			total_h, content.height as usize,
		);
	}
	inline_bar::render_inline_bar(
		frame, bar, inline,
	);
}

/// Clamp scroll, return whether content is scrollable
fn prepare_scroll(
	layout: &[SectionSlot<'_>],
	vp_height: u16,
	inline: &InlineBlocks,
) -> bool {
	let total_h = total_height(layout);
	let vp_h = vp_height as usize;
	let scrollable = total_h > vp_h;
	let max = if scrollable {
		total_h.saturating_sub(vp_h).sat_u16()
	} else {
		0
	};
	inline.set_max_scroll(max);
	inline.clamp_scroll(max);
	scrollable
}

/// Sum all section heights with 1-row gaps
pub(super) fn total_height(
	layout: &[SectionSlot<'_>],
) -> usize {
	let gaps = layout.len().saturating_sub(1);
	let sum: usize = layout
		.iter()
		.map(|s| match s {
			SectionSlot::Text { height, .. }
			| SectionSlot::Code { height, .. } => {
				*height as usize
			}
		})
		.sum();
	sum + gaps
}

struct Viewport {
	inner: Rect,
	top: i32,
	btm: i32,
}

fn render_content(
	frame: &mut Frame,
	area: Rect,
	inline: &InlineBlocks,
	scrollable: bool,
	layout: &[SectionSlot<'_>],
) {
	let scroll =
		inline.scroll_offset().val() as i32;
	let w = if scrollable {
		area.width.saturating_sub(1)
	} else {
		area.width
	};
	let vp = Viewport {
		inner: Rect::new(
			area.x, area.y, w, area.height,
		),
		top: area.y as i32,
		btm: (area.y + area.height) as i32,
	};
	let mut vy: i32 = area.y as i32 - scroll;
	for (i, slot) in layout.iter().enumerate() {
		if vy >= vp.btm { break; }
		if i > 0 { vy += 1; }
		vy = match slot {
			SectionSlot::Text { text, height } => {
				render_text_at(
					frame, &vp, vy, text, *height,
				)
			}
			SectionSlot::Code { idx, height } => {
				render_code_at(
					frame, &vp, vy, *idx,
					*height, inline,
				)
			}
		};
	}
}

fn render_text_at(
	frame: &mut Frame,
	vp: &Viewport,
	vy: i32,
	text: &str,
	h: u16,
) -> i32 {
	let h = h as i32;
	if vy + h > vp.top {
		let skip = (vp.top - vy).max(0) as u16;
		let ry = vy.max(vp.top) as u16;
		let vis = (vy + h).min(vp.btm)
			- vy.max(vp.top);
		let rect = section_rect(
			vp.inner, ry,
			vis as u16, vp.btm as u16,
		);
		inline_blocks::render_text_section(
			frame, rect, text, skip,
		);
	}
	vy + h
}

fn render_code_at(
	frame: &mut Frame,
	vp: &Viewport,
	vy: i32,
	idx: BlockIdx,
	height: u16,
	inline: &InlineBlocks,
) -> i32 {
	let h = height as i32;
	if vy + h > vp.top {
		let skip =
			(vp.top - vy).max(0) as u16;
		let ry = vy.max(vp.top) as u16;
		let logical_y = vy.max(0) as u16;
		inline_panel_code::render_code_section(
			frame, vp.inner, ry,
			vp.btm as u16, idx, inline,
			skip, logical_y, height,
		);
	}
	vy + h
}

pub(super) fn section_rect(
	area: Rect,
	y: u16,
	height: u16,
	bottom: u16,
) -> Rect {
	let clamped =
		height.min(bottom.saturating_sub(y));
	Rect::new(area.x, y, area.width, clamped)
}

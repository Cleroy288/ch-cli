use ratatui::{
	layout::{Alignment, Rect},
	style::Style,
	text::{Line, Span},
	widgets::{Block, Borders, List, ListItem},
	Frame,
};

use crate::picker::mcp_display::McpDisplayItem;
use crate::picker::Picker;
use crate::ui::styles;
use crate::ui::styles::colors;

use super::render::render_empty_results;

pub fn render_mcp_tools_list(
	frame: &mut Frame,
	area: Rect,
	picker: &Picker,
) {
	let items = picker.filtered_mcp_items();
	if items.is_empty() {
		render_empty_results(frame, area, picker);
		return;
	}
	let selected = picker.selected_index();
	let visible =
		area.height.saturating_sub(2) as usize;
	let list_items =
		build_mcp_items(&items, selected, visible);
	let title = format!(
		" MCP Tools [{}/{}]",
		selected + 1,
		items.len()
	);
	let list = List::new(list_items).block(
		Block::default()
			.borders(Borders::ALL)
			.title(title)
			.title_alignment(Alignment::Left)
			.style(
				Style::default().fg(colors::PICKER),
			),
	);
	frame.render_widget(list, area);
}

fn build_mcp_items(
	items: &[McpDisplayItem],
	selected: usize,
	visible: usize,
) -> Vec<ListItem<'static>> {
	let (name_w, src_w) = col_widths(items);
	let scroll = if selected >= visible {
		selected - visible + 1
	} else {
		0
	};
	items
		.iter()
		.enumerate()
		.skip(scroll)
		.take(visible)
		.map(|(idx, entry)| {
			build_one_mcp_item(
				entry, idx == selected,
				name_w, src_w,
			)
		})
		.collect()
}

fn build_one_mcp_item(
	entry: &McpDisplayItem,
	selected: bool,
	name_w: usize,
	src_w: usize,
) -> ListItem<'static> {
	let style = if selected {
		styles::file_list_selected_style()
	} else {
		Style::default().fg(colors::INPUT_TEXT)
	};
	let dim = Style::default().fg(colors::DIM_TEXT);
	let name = format!(
		" {:<w$}", entry.name, w = name_w,
	);
	let source = format!(
		" {:<w$}", entry.source, w = src_w,
	);
	let spans = vec![
		Span::styled(name, style),
		Span::styled(source, dim),
		Span::styled(
			format!("  {}", entry.description),
			dim,
		),
	];
	ListItem::new(Line::from(spans))
}

fn col_widths(
	items: &[McpDisplayItem],
) -> (usize, usize) {
	items.iter().fold((0, 0), |(nw, sw), i| {
		(
			nw.max(i.name.chars().count()),
			sw.max(i.source.chars().count()),
		)
	})
}

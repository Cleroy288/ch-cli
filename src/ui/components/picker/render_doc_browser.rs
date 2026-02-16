//! Render the doc browser picker list.
//!
//! Shows documented symbols with file paths when
//! the user selects the Doc Browser tool.

use ratatui::{
	layout::{Alignment, Rect},
	style::Style,
	text::{Line, Span},
	widgets::{Block, Borders, List, ListItem},
	Frame,
};

use crate::picker::Picker;
use crate::picker::doc_browser::DocBrowserEntry;
use crate::ui::styles;
use crate::ui::styles::colors;

use super::render::{
	build_picker_title, render_empty_results,
	render_picker_help_text,
};

/// Render the doc browser list.
pub fn render_doc_list(
	frame: &mut Frame,
	area: Rect,
	picker: &Picker,
) {
	let Some(browser) = picker.doc_browser() else {
		return;
	};
	let items = browser.current_items(picker.query());
	if items.is_empty() {
		render_empty_results(frame, area, picker);
		return;
	}
	let selected = picker.selected_index();
	let title = build_doc_title(
		picker, selected, items.len(),
	);
	let list_items = build_doc_items(
		&items, selected, area,
	);
	let list = List::new(list_items).block(
		Block::default()
			.borders(Borders::ALL)
			.title(title)
			.title_alignment(Alignment::Left)
			.style(Style::default().fg(colors::PICKER)),
	);
	frame.render_widget(list, area);
	render_picker_help_text(frame, area);
}

/// Build visible doc items with scrolling
fn build_doc_items(
	items: &[&DocBrowserEntry],
	selected: usize,
	area: Rect,
) -> Vec<ListItem<'static>> {
	let visible =
		area.height.saturating_sub(2) as usize;
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
			build_doc_item(entry, idx == selected)
		})
		.collect()
}

/// Build a single doc browser list item
fn build_doc_item(
	entry: &DocBrowserEntry,
	selected: bool,
) -> ListItem<'static> {
	let style = if selected {
		styles::file_list_selected_style()
	} else {
		Style::default().fg(colors::INPUT_TEXT)
	};
	let spans = vec![
		Span::styled(
			format!("{} ", entry.name),
			style,
		),
		Span::styled(
			format!("({}) ", entry.kind),
			Style::default().fg(colors::DIM_TEXT),
		),
		Span::styled(
			format!("- {}", entry.rel_path),
			Style::default().fg(colors::PLACEHOLDER),
		),
	];
	ListItem::new(Line::from(spans))
}

/// Build the doc browser title with count
fn build_doc_title(
	picker: &Picker,
	selected: usize,
	count: usize,
) -> String {
	format!(
		"{} [{}/{}]",
		build_picker_title(picker),
		selected + 1,
		count
	)
}

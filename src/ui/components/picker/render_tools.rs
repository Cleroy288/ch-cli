//! Render the tools picker list.
//!
//! Shows available tools when '#' is pressed.

use ratatui::{
	layout::{Alignment, Rect},
	style::Style,
	text::{Line, Span},
	widgets::{Block, Borders, List, ListItem},
	Frame,
};

use crate::picker::Picker;
use crate::ui::strings::tui_labels::{
	TOOL_DOC_BROWSER, TOOL_DOC_BROWSER_DESC,
};
use crate::ui::styles;
use crate::ui::styles::colors;

use super::render::render_picker_help_text;

/// Render the tools list picker.
pub fn render_tools_list(
	frame: &mut Frame,
	area: Rect,
	picker: &Picker,
) {
	let selected = picker.selected_index();
	let items = build_tools_items(selected);
	let title = " Tools [1/1] ";
	let list = List::new(items).block(
		Block::default()
			.borders(Borders::ALL)
			.title(title)
			.title_alignment(Alignment::Left)
			.style(Style::default().fg(colors::PICKER)),
	);
	frame.render_widget(list, area);
	render_picker_help_text(frame, area);
}

/// Build the list items for available tools
fn build_tools_items(
	selected: usize,
) -> Vec<ListItem<'static>> {
	let style = if selected == 0 {
		styles::file_list_selected_style()
	} else {
		Style::default().fg(colors::INPUT_TEXT)
	};
	let spans = vec![
		Span::styled(
			format!("D {}", TOOL_DOC_BROWSER),
			style,
		),
		Span::styled(
			format!(" - {}", TOOL_DOC_BROWSER_DESC),
			Style::default().fg(colors::DIM_TEXT),
		),
	];
	vec![ListItem::new(Line::from(spans))]
}

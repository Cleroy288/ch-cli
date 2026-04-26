use std::path::PathBuf;

use ratatui::{
	layout::{Alignment, Rect},
	style::Style,
	text::{Line, Span},
	widgets::{Block, Borders, List, ListItem},
	Frame,
};

use crate::picker::Picker;
use crate::ui::styles;
use crate::ui::styles::colors;

/// Render the git repo selection list.
pub fn render_git_repo_select(
	frame: &mut Frame,
	area: Rect,
	picker: &Picker,
) {
	let repos = picker.filtered_git_repos();
	let selected = picker.selected_index();
	let visible =
		area.height.saturating_sub(2) as usize;
	let scroll = selected.saturating_sub(
		visible.saturating_sub(1),
	);
	let items = build_items(
		&repos, selected, scroll, visible,
	);
	let count = repos.len();
	let display_idx = if count == 0 {
		0
	} else {
		selected + 1
	};
	let title = format!(
		" Git Repos [{display_idx}/{count}] ",
	);
	let list = List::new(items).block(
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

fn build_items(
	repos: &[&PathBuf],
	selected: usize,
	scroll: usize,
	visible: usize,
) -> Vec<ListItem<'static>> {
	repos
		.iter()
		.enumerate()
		.skip(scroll)
		.take(visible)
		.map(|(i, r)| build_item(r, i == selected))
		.collect()
}

fn build_item(
	path: &PathBuf,
	is_selected: bool,
) -> ListItem<'static> {
	let name = path
		.file_name()
		.and_then(|n| n.to_str())
		.unwrap_or("?");
	let parent = path
		.parent()
		.and_then(|p| p.file_name())
		.and_then(|n| n.to_str())
		.unwrap_or("");
	let style = if is_selected {
		styles::file_list_selected_style()
	} else {
		Style::default().fg(colors::INPUT_TEXT)
	};
	let dim =
		Style::default().fg(colors::PLACEHOLDER);
	let line = Line::from(vec![
		Span::styled(name.to_string(), style),
		Span::styled(
			format!("  {parent}/"), dim,
		),
	]);
	ListItem::new(line)
}

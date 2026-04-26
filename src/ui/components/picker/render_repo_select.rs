use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem,
    },
    Frame,
};

use crate::domain::repo_info::RepoEntry;
use crate::picker::Picker;
use crate::ui::styles;
use crate::ui::styles::colors;

pub fn render_repo_select(
    frame: &mut Frame,
    area: Rect,
    picker: &Picker,
) {
    let repos = picker.filtered_repos();
    if repos.is_empty() {
        render_empty_repos(frame, area);
        return;
    }
    render_repo_list(frame, area, picker, &repos);
}

fn render_empty_repos(
    frame: &mut Frame,
    area: Rect,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Select Repo ")
        .title_alignment(Alignment::Left)
        .style(Style::default().fg(colors::PICKER));
    let msg = ratatui::widgets::Paragraph::new(
        Line::from("No repos found"),
    )
    .style(Style::default().fg(colors::PLACEHOLDER))
    .alignment(Alignment::Center)
    .block(block);
    frame.render_widget(msg, area);
}

fn render_repo_list(
    frame: &mut Frame,
    area: Rect,
    picker: &Picker,
    repos: &[&RepoEntry],
) {
    let selected = picker.selected_index();
    let visible =
        area.height.saturating_sub(2) as usize;
    let scroll = selected.saturating_sub(
        visible.saturating_sub(1),
    );
    let items = build_repo_items(
        repos, selected, scroll, visible,
    );
    let title = format!(
        " Select Repo [{}/{}] ",
        selected + 1,
        repos.len(),
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

fn build_repo_items(
    repos: &[&RepoEntry],
    selected: usize,
    scroll: usize,
    visible: usize,
) -> Vec<ListItem<'static>> {
    repos
        .iter()
        .enumerate()
        .skip(scroll)
        .take(visible)
        .map(|(idx, repo)| {
            build_repo_item(repo, idx == selected)
        })
        .collect()
}

fn build_repo_item(
    repo: &RepoEntry,
    is_selected: bool,
) -> ListItem<'static> {
    let style = if is_selected {
        styles::file_list_selected_style()
    } else {
        Style::default().fg(colors::INPUT_TEXT)
    };
    let dim = Style::default()
        .fg(colors::PLACEHOLDER);
    let line = Line::from(vec![
        Span::styled(
            repo.folder.clone(), style,
        ),
        Span::styled(
            format!(
                "  {}/{}",
                repo.workspace, repo.repo_slug,
            ),
            dim,
        ),
    ]);
    ListItem::new(line)
}

use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::picker::{Picker, Symbol, SymbolKind};
use crate::picker::symbol_browser::SymbolBrowser;
use crate::ui::styles::colors;
use crate::ui::styles;

use super::render::{
    build_picker_title, render_empty_results,
    render_picker_help_text,
};

pub fn render_symbol_list(
    frame: &mut Frame,
    area: Rect,
    picker: &Picker,
) {
    let Some(browser) = picker.symbol_browser()
    else {
        return;
    };
    let items = browser.current_items(picker.query());
    if items.is_empty() {
        render_empty_results(frame, area, picker);
        return;
    }
    let selected = picker.selected_index();
    let list_items = build_visible_items(
        &items, selected, area, browser,
    );
    let title = format!(
        "{} [{}/{}]",
        build_picker_title(picker),
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
    render_picker_help_text(frame, area);
}

fn build_visible_items(
    items: &[&Symbol],
    selected: usize,
    area: Rect,
    browser: &SymbolBrowser,
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
        .map(|(idx, sym)| {
            let has_doc =
                browser.has_doc(&sym.name);
            build_symbol_item(
                sym, idx == selected, has_doc,
            )
        })
        .collect()
}

fn build_symbol_item(
    sym: &Symbol,
    selected: bool,
    has_doc: bool,
) -> ListItem<'static> {
    let icon = symbol_kind_icon(sym.kind);
    let style = if selected {
        styles::file_list_selected_style()
    } else {
        Style::default().fg(colors::INPUT_TEXT)
    };
    let mut spans = vec![Span::styled(
        format!("{} {}", icon, sym.name),
        style,
    )];
    if has_doc {
        spans.push(Span::styled(
            " (doc)",
            Style::default().fg(colors::DIM_TEXT),
        ));
    }
    ListItem::new(Line::from(spans))
}

pub fn symbol_kind_icon(
    kind: SymbolKind,
) -> &'static str {
    match kind {
        SymbolKind::Function => "ƒ",
        SymbolKind::Method => "ƒ",
        SymbolKind::Struct => "S",
        SymbolKind::Enum => "E",
        SymbolKind::Trait => "T",
        SymbolKind::Impl => "I",
        SymbolKind::Constant => "C",
        SymbolKind::Static => "s",
        SymbolKind::TypeAlias => "t",
        SymbolKind::Module => "m",
        SymbolKind::Macro => "!",
        SymbolKind::EnumVariant => "v",
        SymbolKind::Field => "·",
        SymbolKind::DocumentChunk => "d",
    }
}

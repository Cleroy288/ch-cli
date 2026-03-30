use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use super::input_styling;
use crate::app::App;
use crate::ui::strings::tui_labels;
use crate::ui::styles::colors;

/// Horizontal separator above the input line.
pub(super) fn build_separator(
    width: u16,
) -> Line<'static> {
    build_rule(width, colors::SEPARATOR)
}

/// Horizontal separator below the input text.
pub(super) fn build_bottom_separator(
    width: u16,
) -> Line<'static> {
    build_rule(width, colors::SEPARATOR)
}

/// Build input lines (multi-line aware).
///
/// Returns one `Line` per logical line.
/// The first line gets the `❯ ` prompt.
pub(super) fn build_input_lines(
    app: &App,
) -> Vec<Line<'static>> {
    if !app.input().is_empty() {
        return prepend_prompt_lines(
            input_styling::build_styled_input_lines(
                app,
            ),
        );
    }
    if app.is_claude_loading() {
        return vec![Line::from(vec![
            prompt_span(),
            Span::styled(
                tui_labels::INPUT_WAITING,
                Style::default()
                    .fg(colors::ACCENT_DIM)
                    .bold(),
            ),
        ])];
    }
    vec![Line::from(vec![
        prompt_span(),
        Span::styled(
            tui_labels::INPUT_PLACEHOLDER,
            Style::default().fg(colors::PLACEHOLDER),
        ),
    ])]
}

/// Colored rule spanning `width` columns.
fn build_rule(
    width: u16,
    color: Color,
) -> Line<'static> {
    let rule = "\u{2500}"
        .repeat(width as usize);
    Line::from(Span::styled(
        rule,
        Style::default().fg(color),
    ))
}

/// The `❯` prompt character span.
fn prompt_span() -> Span<'static> {
    Span::styled(
        tui_labels::PROMPT_CHAR,
        Style::default().fg(colors::ACCENT).bold(),
    )
}

/// Prepend `❯ ` to the first line only.
fn prepend_prompt_lines(
    lines: Vec<Line<'static>>,
) -> Vec<Line<'static>> {
    let mut result =
        Vec::with_capacity(lines.len());
    for (i, line) in lines.into_iter().enumerate() {
        if i == 0 {
            let mut spans = Vec::with_capacity(
                1 + line.spans.len(),
            );
            spans.push(prompt_span());
            spans.extend(line.spans);
            result.push(Line::from(spans));
        } else {
            result.push(line);
        }
    }
    result
}

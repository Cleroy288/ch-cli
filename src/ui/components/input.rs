use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Clear, Paragraph, Wrap},
    Frame,
};

use super::input_prompt::{
    build_bottom_separator, build_input_lines,
    build_separator,
};
use super::input_suggest;
use crate::app::App;
use crate::domain::constants::{
    MAX_INPUT_FRAC, MIN_INPUT_HEIGHT, PROMPT_WIDTH,
};
use crate::domain::cursor_grid;
use crate::ui::styles::colors;

/// Input area — top sep | wrapped text | bottom sep.
pub fn render_input(
    frame: &mut Frame,
    area: Rect,
    app: &App,
) {
    frame.render_widget(Clear, area);
    let text_h = area.height.saturating_sub(2);
    let rows = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(text_h),
        Constraint::Length(1),
    ]);
    let [top, mid, bot] = rows.areas(area);
    let sep = build_separator(top.width);
    frame.render_widget(Paragraph::new(sep), top);
    let lines = build_input_lines(app);
    let text = Text::from(lines);
    let body = Paragraph::new(text)
        .wrap(Wrap { trim: false });
    frame.render_widget(body, mid);
    let hint = input_suggest::format_suggestions(
        app.agent_suggestions(),
    );
    render_bottom_sep(frame, bot, hint);
}

/// Bottom separator with optional suggestion hint.
fn render_bottom_sep(
    frame: &mut Frame,
    area: Rect,
    hint: Option<String>,
) {
    match hint {
        Some(text) => {
            let line = Line::from(Span::styled(
                text,
                Style::default().fg(colors::DIM_TEXT),
            ));
            frame.render_widget(
                Paragraph::new(line),
                area,
            );
        }
        None => {
            let bsep =
                build_bottom_separator(area.width);
            frame.render_widget(
                Paragraph::new(bsep),
                area,
            );
        }
    }
}

/// Place the blinking cursor at the wrapped position.
pub fn set_cursor(
    frame: &mut Frame,
    area: Rect,
    input: &str,
    cursor_pos: usize,
) {
    if let Some((cx, cy)) =
        compute_cursor_xy(area, input, cursor_pos)
    {
        frame.set_cursor_position((cx, cy));
    }
}

/// Compute terminal (x, y) for the cursor, or None
/// if it falls outside the visible input area.
fn compute_cursor_xy(
    area: Rect,
    input: &str,
    cursor_pos: usize,
) -> Option<(u16, u16)> {
    let first = first_row_cols(area.width);
    let full = area.width as usize;
    if first == 0 || full == 0 {
        return None;
    }
    let (row, col) = cursor_grid::cursor_to_visual(
        input, cursor_pos, first, full,
    );
    let x_off = if row == 0 {
        PROMPT_WIDTH + col as u16
    } else {
        col as u16
    };
    let cx = area.x.saturating_add(x_off);
    let row_u16 =
        u16::try_from(row).unwrap_or(u16::MAX);
    // +1 to skip top separator row
    let cy = area
        .y
        .saturating_add(1)
        .saturating_add(row_u16);
    let max_y =
        area.y + area.height.saturating_sub(1);
    if cx < area.x + area.width && cy < max_y {
        Some((cx, cy))
    } else {
        None
    }
}

/// Dynamic height for the input area.
pub fn compute_input_height(
    input: &str,
    area_width: u16,
    area_height: u16,
) -> u16 {
    let first = first_row_cols(area_width);
    let full = area_width as usize;
    let wrapped = if full == 0 || input.is_empty() {
        1
    } else {
        cursor_grid::total_visual_rows(
            input, first, full,
        ) as u16
    };
    // top sep (1) + text lines + bottom sep (1)
    let raw = 1 + wrapped + 1;
    let cap = area_height / MAX_INPUT_FRAC;
    raw.max(MIN_INPUT_HEIGHT)
        .min(cap.max(MIN_INPUT_HEIGHT))
}

/// Columns available on the first row (after prompt).
fn first_row_cols(area_width: u16) -> usize {
    area_width.saturating_sub(PROMPT_WIDTH) as usize
}

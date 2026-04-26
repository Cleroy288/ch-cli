/// Pre-wrap engine for ratatui Lines.
///
/// Splits lines so each fits within a given terminal
/// width. Enables scroll-by-slice instead of
/// Paragraph::scroll().
use ratatui::text::{Line, Span};
use unicode_width::UnicodeWidthChar;

/// Pre-wrap lines so each fits within `width` columns.
///
/// Flat-maps each line through `wrap_line`.
pub fn pre_wrap(
    lines: Vec<Line<'static>>,
    width: u16,
) -> Vec<Line<'static>> {
    lines
        .into_iter()
        .flat_map(|l| wrap_line(l, width))
        .collect()
}

/// Wrap a single line if it exceeds width.
fn wrap_line(
    line: Line<'static>,
    width: u16,
) -> Vec<Line<'static>> {
    if width == 0 {
        return vec![line];
    }
    let w = width as usize;
    let total: usize = line_width(&line);
    if total <= w {
        return vec![line];
    }
    split_line(line, width)
}

/// Compute the display width of a line in columns.
fn line_width(line: &Line<'_>) -> usize {
    line.spans
        .iter()
        .flat_map(|s| s.content.chars())
        .filter_map(|c| c.width())
        .sum()
}

/// Split a line that exceeds width into multiple rows.
///
/// Iterates spans char-by-char using unicode width.
/// Preserves style on split spans.
fn split_line(
    line: Line<'static>,
    width: u16,
) -> Vec<Line<'static>> {
    let w = width as usize;
    // row_spans: spans accumulated for current row
    let mut rows: Vec<Line<'static>> = Vec::new();
    let mut row_spans: Vec<Span<'static>> = Vec::new();
    let mut row_w: usize = 0;
    // buf: chars for current span fragment
    let mut buf = String::new();

    for span in line.spans {
        let style = span.style;
        for ch in span.content.chars() {
            let cw = ch.width().unwrap_or(0);
            if cw > 0 && row_w + cw > w {
                flush_buf(&mut buf, style, &mut row_spans);
                rows.push(Line::from(
                    row_spans.drain(..).collect::<Vec<_>>(),
                ));
                row_w = 0;
            }
            buf.push(ch);
            row_w += cw;
        }
        flush_buf(&mut buf, style, &mut row_spans);
    }
    if !row_spans.is_empty() {
        rows.push(Line::from(
            row_spans.drain(..).collect::<Vec<_>>(),
        ));
    }
    if rows.is_empty() {
        rows.push(Line::default());
    }
    rows
}

/// Flush char buffer into a span and push it.
fn flush_buf(
    buf: &mut String,
    style: ratatui::style::Style,
    spans: &mut Vec<Span<'static>>,
) {
    if buf.is_empty() {
        return;
    }
    let text = std::mem::take(buf);
    spans.push(Span::styled(text, style));
}

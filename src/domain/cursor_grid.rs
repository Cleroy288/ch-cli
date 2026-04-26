/// Row/col from a 1D byte offset in multi-line text.
///
/// Only logical line 0 has `first_cols` columns
/// (prompt eats space). All other lines use full_width
/// on every visual row.

use super::cursor_grid_helpers::{
    char_count_in_range, line_byte_start,
    offset_in_line, visual_rows_for,
};

/// Logical lines split on `\n`.
pub fn logical_lines(text: &str) -> Vec<&str> {
    if text.is_empty() {
        return vec![""];
    }
    text.split('\n').collect()
}

/// Which logical line the byte offset falls on.
pub fn logical_line_at(
    text: &str,
    byte_offset: usize,
) -> usize {
    text[..byte_offset.min(text.len())]
        .matches('\n')
        .count()
}

/// Convert a 1D byte offset to (visual_row, visual_col).
///
/// Only line 0 uses `first_cols` for its first row.
/// All other lines use `full_width` on every row.
pub fn cursor_to_visual(
    text: &str,
    byte_offset: usize,
    first_cols: usize,
    full_width: usize,
) -> (usize, usize) {
    if full_width == 0 {
        return (0, 0);
    }
    let mut vis_row = 0;
    for (li, line) in logical_lines(text)
        .iter()
        .enumerate()
    {
        let start = line_byte_start(text, li);
        let cols = line_first_cols(
            li, first_cols, full_width,
        );
        let len = line.chars().count();
        let cur = char_count_in_range(
            text,
            start,
            byte_offset.min(text.len()),
        );
        if byte_offset <= start + line.len() {
            let (r, c) = offset_in_line(
                cur, cols, full_width,
            );
            return (vis_row + r, c);
        }
        vis_row += visual_rows_for(
            len, cols, full_width,
        );
    }
    (vis_row, 0)
}

/// Total visual rows for the entire text.
pub fn total_visual_rows(
    text: &str,
    first_cols: usize,
    full_width: usize,
) -> usize {
    if full_width == 0 {
        return 1;
    }
    logical_lines(text)
        .iter()
        .enumerate()
        .map(|(li, line)| {
            let cols = line_first_cols(
                li, first_cols, full_width,
            );
            visual_rows_for(
                line.chars().count(),
                cols,
                full_width,
            )
        })
        .sum()
}

/// First-row cols for a given logical line.
///
/// Line 0 has prompt → reduced width.
/// All other lines use full_width.
fn line_first_cols(
    line_idx: usize,
    first_cols: usize,
    full_width: usize,
) -> usize {
    if line_idx == 0 { first_cols } else { full_width }
}

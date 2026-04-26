/// Byte offset of the start of logical line `n`.
pub(crate) fn line_byte_start(text: &str, n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    text.match_indices('\n')
        .nth(n - 1)
        .map(|(i, _)| i + 1)
        .unwrap_or(text.len())
}

/// Char count between two byte offsets.
pub(crate) fn char_count_in_range(
    text: &str,
    from: usize,
    to: usize,
) -> usize {
    if to <= from || from >= text.len() {
        return 0;
    }
    let end = to.min(text.len());
    text[from..end].chars().count()
}

/// (row, col) for a char offset within one line.
pub(crate) fn offset_in_line(
    char_offset: usize,
    first_cols: usize,
    full_width: usize,
) -> (usize, usize) {
    if first_cols == 0 || char_offset <= first_cols {
        return (0, char_offset);
    }
    let past = char_offset - first_cols;
    (1 + past / full_width, past % full_width)
}

/// Visual rows needed for one logical line.
pub(crate) fn visual_rows_for(
    char_len: usize,
    first_cols: usize,
    full_width: usize,
) -> usize {
    if first_cols == 0 || char_len <= first_cols {
        return 1;
    }
    let past = char_len - first_cols;
    1 + past.div_ceil(full_width)
}

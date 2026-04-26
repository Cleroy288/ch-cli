/// Multi-line input navigation.
///
/// Shift+Enter inserts `\n`. Up/Down navigate between
/// logical lines when cursor is not on first/last line.

use crate::app::App;
use crate::domain::cursor_grid;

impl App {
    /// Insert a newline at the cursor position.
    pub(crate) fn insert_newline(&mut self) {
        let pos = self.cursor_position.get();
        self.input.insert(pos, '\n');
        self.cursor_position.move_right(
            &self.input,
        );
    }

    /// True if cursor is on the first logical line.
    pub(crate) fn cursor_on_first_line(
        &self,
    ) -> bool {
        cursor_grid::logical_line_at(
            &self.input,
            self.cursor_position.get(),
        ) == 0
    }

    /// True if cursor is on the last logical line.
    pub(crate) fn cursor_on_last_line(
        &self,
    ) -> bool {
        let lines =
            cursor_grid::logical_lines(&self.input);
        let current = cursor_grid::logical_line_at(
            &self.input,
            self.cursor_position.get(),
        );
        current >= lines.len().saturating_sub(1)
    }

    /// Move cursor up one logical line.
    pub(crate) fn cursor_move_up(&mut self) {
        self.move_to_line(-1);
    }

    /// Move cursor down one logical line.
    pub(crate) fn cursor_move_down(&mut self) {
        self.move_to_line(1);
    }

    fn move_to_line(&mut self, delta: isize) {
        let pos = self.cursor_position.get();
        let line = cursor_grid::logical_line_at(
            &self.input, pos,
        ) as isize;
        let total = cursor_grid::logical_lines(
            &self.input,
        )
        .len() as isize;
        let target = line + delta;
        if target < 0 || target >= total {
            return;
        }
        let target = target as usize;
        let col =
            col_in_current_line(&self.input, pos);
        let start = line_start(&self.input, target);
        let len = line_char_len(&self.input, target);
        let byte = byte_at_char(
            &self.input,
            start,
            col.min(len),
        );
        self.cursor_position.set(byte);
    }
}

/// Char column of cursor within its logical line.
fn col_in_current_line(
    text: &str,
    byte_pos: usize,
) -> usize {
    let start = text[..byte_pos.min(text.len())]
        .rfind('\n')
        .map(|i| i + 1)
        .unwrap_or(0);
    text[start..byte_pos.min(text.len())]
        .chars()
        .count()
}

/// Byte offset of the start of logical line `n`.
fn line_start(text: &str, n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    text.match_indices('\n')
        .nth(n - 1)
        .map(|(i, _)| i + 1)
        .unwrap_or(text.len())
}

/// Char length of logical line `n`.
fn line_char_len(text: &str, n: usize) -> usize {
    cursor_grid::logical_lines(text)
        .get(n)
        .map(|l| l.chars().count())
        .unwrap_or(0)
}

/// Byte offset at `char_offset` chars past `start`.
fn byte_at_char(
    text: &str,
    start: usize,
    char_offset: usize,
) -> usize {
    text[start..]
        .char_indices()
        .nth(char_offset)
        .map(|(i, _)| start + i)
        .unwrap_or_else(|| {
            // End of line (or text)
            text[start..]
                .find('\n')
                .map(|i| start + i)
                .unwrap_or(text.len())
        })
}

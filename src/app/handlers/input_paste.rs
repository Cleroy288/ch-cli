use crate::app::App;

/// A block of pasted content stored for expansion
#[derive(Debug, Clone)]
pub struct PasteBlock {
    pub start: usize,
    /// End position in the input string
    pub end: usize,
    /// The actual pasted content
    pub content: String,
}

impl App {
    /// Handle a bracketed paste event
    pub fn handle_paste(&mut self, data: &str) {
        let line_count = count_lines(data);
        let label = format_label(line_count);
        let start = self.cursor_position.get();
        self.insert_text_at_cursor(&label);
        let end = self.cursor_position.get();
        self.paste_blocks.push(PasteBlock {
            start,
            end,
            content: data.to_string(),
        });
    }

    /// Expand paste placeholders into real content.
    ///
    /// Called before sending a message. Returns the
    /// expanded input with placeholders replaced.
    pub fn expand_paste_blocks(&self) -> String {
        if self.paste_blocks.is_empty() {
            return self.input.clone();
        }
        build_expanded(&self.input, &self.paste_blocks)
    }

}

fn count_lines(data: &str) -> usize {
    let count = data.lines().count();
    count.max(1)
}

fn format_label(line_count: usize) -> String {
    if line_count == 1 {
        "[1 line pasted]".to_string()
    } else {
        format!("[{line_count} lines pasted]")
    }
}

fn build_expanded(
    input: &str,
    blocks: &[PasteBlock],
) -> String {
    let mut sorted: Vec<&PasteBlock> =
        blocks.iter().collect();
    sorted.sort_by_key(|b| b.start);
    let mut result = String::new();
    let mut last = 0;
    for block in &sorted {
        if block.start < last {
            continue;
        }
        if is_safe_slice(input, last, block.start) {
            result.push_str(
                &input[last..block.start],
            );
        }
        result.push_str(&block.content);
        last = block.end;
    }
    if is_safe_slice(input, last, input.len()) {
        result.push_str(&input[last..]);
    }
    result
}

/// Check both bounds are valid char boundaries
fn is_safe_slice(
    s: &str,
    start: usize,
    end: usize,
) -> bool {
    start <= end
        && end <= s.len()
        && s.is_char_boundary(start)
        && s.is_char_boundary(end)
}

use crate::app::App;

impl App {
    /// Up arrow — browse to older history entry.
    pub(crate) fn history_prev(&mut self) {
        let count = self.prompt_history.len();
        if count == 0 {
            return;
        }
        let next = match self.history_index {
            None => {
                self.saved_input =
                    self.input.clone();
                0
            }
            Some(i) if i + 1 < count => i + 1,
            _ => return,
        };
        self.history_index = Some(next);
        self.load_history_entry(next);
    }

    /// Down arrow — browse to newer entry or
    /// restore saved input.
    pub(crate) fn history_next(&mut self) {
        let Some(idx) = self.history_index else {
            return;
        };
        if idx == 0 {
            self.history_index = None;
            self.input =
                std::mem::take(&mut self.saved_input);
            self.cursor_position.jump_to_end(
                self.input.len(),
            );
            return;
        }
        let next = idx - 1;
        self.history_index = Some(next);
        self.load_history_entry(next);
    }

    /// Replace input with prompt_history entry.
    fn load_history_entry(&mut self, idx: usize) {
        let text = self
            .prompt_history
            .get_from_end(idx)
            .unwrap_or("")
            .to_owned();
        self.input = text;
        self.cursor_position.jump_to_end(
            self.input.len(),
        );
        self.file_references.clear();
        self.symbol_selectors.clear();
        self.tool_references.clear();
        self.paste_blocks.clear();
    }
}

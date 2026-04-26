use super::cursor::CursorPosition;

impl CursorPosition {
    pub fn move_left(&mut self, input: &str) {
        let pos = self.get().min(input.len());
        if pos > 0 {
            let prev = input[..pos]
                .char_indices()
                .next_back()
                .map(|(idx, _)| idx)
                .unwrap_or(0);
            self.set(prev);
        }
    }

    pub fn move_right(&mut self, input: &str) {
        let pos = self.get().min(input.len());
        if pos < input.len() {
            let next = input[pos..]
                .char_indices()
                .nth(1)
                .map(|(idx, _)| pos + idx)
                .unwrap_or(input.len());
            self.set(next);
        }
    }

    pub fn jump_to_start(&mut self) {
        self.set(0);
    }

    pub fn jump_to_end(&mut self, max: usize) {
        self.set(max);
    }

    /// Jump to start of the current logical line.
    pub fn jump_to_line_start(
        &mut self,
        input: &str,
    ) {
        let pos = self.get().min(input.len());
        let start = input[..pos]
            .rfind('\n')
            .map(|i| i + 1)
            .unwrap_or(0);
        self.set(start);
    }

    /// Jump to end of the current logical line.
    pub fn jump_to_line_end(
        &mut self,
        input: &str,
    ) {
        let pos = self.get().min(input.len());
        let end = input[pos..]
            .find('\n')
            .map(|i| pos + i)
            .unwrap_or(input.len());
        self.set(end);
    }
}

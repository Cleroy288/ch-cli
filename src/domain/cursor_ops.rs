use super::cursor::CursorPosition;

/// Cursor movement operations
impl CursorPosition {
    /// Move cursor left
    ///
    /// Decreases position, ensuring it doesn't go below zero.
    pub fn move_left(&mut self) {
        if self.get() > 0 {
            self.set(self.get() - 1);
        }
    }

    /// Move cursor right
    ///
    /// Increases position, ensuring it doesn't exceed max.
    pub fn move_right(&mut self, max: usize) {
        if self.get() < max {
            self.set(self.get() + 1);
        }
    }

    /// Jump to the start (position 0)
    pub fn jump_to_start(&mut self) {
        self.set(0);
    }

    /// Jump to the end (position = max)
    pub fn jump_to_end(&mut self, max: usize) {
        self.set(max);
    }
}

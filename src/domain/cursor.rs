/// NewType for cursor position to make illegal states unrepresentable.
/// Wraps a usize to ensure type safety and provide domain-specific methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorPosition(usize);

impl CursorPosition {
    /// Create a new cursor position at the start
    pub fn new() -> Self {
        Self(0)
    }

    /// Create a cursor position from a raw value
    pub fn from_raw(pos: usize) -> Self {
        Self(pos)
    }

    /// Get the raw position value
    pub fn get(&self) -> usize {
        self.0
    }

    /// Move cursor left (decrease position), ensuring it doesn't go below zero
    pub fn move_left(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }

    /// Move cursor right (increase position), ensuring it doesn't exceed max
    pub fn move_right(&mut self, max: usize) {
        if self.0 < max {
            self.0 += 1;
        }
    }

    /// Jump to the start (position 0)
    pub fn jump_to_start(&mut self) {
        self.0 = 0;
    }

    /// Jump to the end (position = max)
    pub fn jump_to_end(&mut self, max: usize) {
        self.0 = max;
    }

    /// Set to a specific position
    pub fn set(&mut self, pos: usize) {
        self.0 = pos;
    }
}

impl Default for CursorPosition {
    fn default() -> Self {
        Self::new()
    }
}

impl From<usize> for CursorPosition {
    fn from(pos: usize) -> Self {
        Self(pos)
    }
}

impl From<CursorPosition> for usize {
    fn from(cursor: CursorPosition) -> Self {
        cursor.0
    }
}

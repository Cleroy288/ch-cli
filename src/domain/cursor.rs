/// NewType for cursor position
///
/// Makes illegal states unrepresentable and provides
/// domain-specific methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorPosition(pub(crate) usize);

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

    /// Set to a specific position
    pub fn set(&mut self, pos: usize) {
        self.0 = pos;
    }
}

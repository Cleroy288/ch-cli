#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorPosition(pub(crate) usize);

impl CursorPosition {
    pub fn get(&self) -> usize {
        self.0
    }

    pub fn set(&mut self, pos: usize) {
        self.0 = pos;
    }
}

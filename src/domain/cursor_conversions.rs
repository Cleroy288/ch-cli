use super::CursorPosition;

impl Default for CursorPosition {
	fn default() -> Self {
		Self(0)
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

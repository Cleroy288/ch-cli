//! From/Default implementations for CursorPosition.

use super::CursorPosition;

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

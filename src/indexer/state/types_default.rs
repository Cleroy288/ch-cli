//! Default implementation for IndexState.

use std::path::PathBuf;

use super::types::IndexState;

impl Default for IndexState {
	fn default() -> Self {
		Self::new(PathBuf::new())
	}
}

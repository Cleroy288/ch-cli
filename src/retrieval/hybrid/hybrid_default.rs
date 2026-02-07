//! HybridSearch Default Implementation
//!
//! Default trait implementation for HybridSearch.

use crate::retrieval::hybrid::HybridSearch;

impl Default for HybridSearch {
	fn default() -> Self {
		Self::new().expect("failed to create hybrid search")
	}
}

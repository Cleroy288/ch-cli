//! Memory service — interaction storage and
//! search use cases.

mod default;

pub use default::DefaultMemoryService;

use std::path::Path;

use crate::domain::errors::memory::MemoryResult;
use crate::domain::memory::Interaction;
use crate::indexer::memory::types::{
	MemoryHit, MemoryStats,
};

/// Service trait for memory operations
pub trait MemoryService {
	/// Store a new interaction
	fn add(
		&self,
		root: &Path,
		interaction: &Interaction,
	) -> MemoryResult<()>;

	/// Load recent interactions
	fn show_recent(
		&self,
		root: &Path,
		limit: usize,
	) -> MemoryResult<Vec<Interaction>>;

	/// Search interactions by keyword
	fn search(
		&self,
		root: &Path,
		query: &str,
		limit: usize,
	) -> MemoryResult<Vec<MemoryHit>>;

	/// Compute memory store statistics
	fn stats(
		&self,
		root: &Path,
	) -> MemoryResult<MemoryStats>;
}

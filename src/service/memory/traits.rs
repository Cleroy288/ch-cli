use std::path::Path;

use crate::domain::errors::memory::MemoryResult;
use crate::domain::memory::Interaction;
use crate::indexer::memory::types::{
	MemoryHit, MemoryStats,
};

/// Interaction storage and keyword search
pub trait MemoryService {
	fn add(
		&self,
		root: &Path,
		interaction: &Interaction,
	) -> MemoryResult<()>;

	fn show_recent(
		&self,
		root: &Path,
		limit: usize,
	) -> MemoryResult<Vec<Interaction>>;

	fn search(
		&self,
		root: &Path,
		query: &str,
		limit: usize,
	) -> MemoryResult<Vec<MemoryHit>>;

	fn stats(
		&self,
		root: &Path,
	) -> MemoryResult<MemoryStats>;
}

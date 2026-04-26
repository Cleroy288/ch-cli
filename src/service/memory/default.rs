use std::path::Path;

use crate::domain::errors::memory::MemoryResult;
use crate::domain::memory::Interaction;
use crate::indexer::memory::{
	self, MemorySearchIndex,
};
use crate::indexer::memory::types::{
	MemoryHit, MemoryStats,
};

use super::MemoryService;

/// Memory service backed by JSONL + Tantivy
#[derive(Default)]
pub struct DefaultMemoryService;

impl MemoryService for DefaultMemoryService {
	fn add(
		&self,
		root: &Path,
		interaction: &Interaction,
	) -> MemoryResult<()> {
		memory::append(root, interaction)?;
		index_if_available(root, interaction);
		Ok(())
	}

	fn show_recent(
		&self,
		root: &Path,
		limit: usize,
	) -> MemoryResult<Vec<Interaction>> {
		memory::load_recent(root, limit)
	}

	fn search(
		&self,
		root: &Path,
		query: &str,
		limit: usize,
	) -> MemoryResult<Vec<MemoryHit>> {
		let idx =
			MemorySearchIndex::open_or_create(root)?;
		idx.search(query, limit)
	}

	fn stats(
		&self,
		root: &Path,
	) -> MemoryResult<MemoryStats> {
		memory::compute_stats(root)
	}
}

/// Best-effort index update (non-fatal on error)
#[allow(clippy::print_stderr)]
fn index_if_available(
	root: &Path,
	interaction: &Interaction,
) {
	let idx = match
		MemorySearchIndex::open_or_create(root)
	{
		Ok(idx) => idx,
		Err(err) => {
			eprintln!(
				"[memory] index open failed: {err}"
			);
			return;
		}
	};
	if let Err(err) =
		idx.index_interaction(interaction)
	{
		eprintln!(
			"[memory] index write failed: {err}"
		);
	}
}

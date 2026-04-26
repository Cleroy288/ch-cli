use std::path::Path;

use tantivy::{Index, IndexReader, ReloadPolicy};

use crate::domain::errors::memory::{
	MemoryError, MemoryResult,
};

use super::search_schema::{
	MemoryFields, build_memory_schema,
};

/// Tantivy-backed search index for memory interactions
pub struct MemorySearchIndex {
	pub(crate) index: Index,
	pub(crate) fields: MemoryFields,
}

impl MemorySearchIndex {
	/// Open or create a persistent index
	pub fn open_or_create(
		root: &Path,
	) -> MemoryResult<Self> {
		let dir =
			super::paths::tantivy_dir(root);
		std::fs::create_dir_all(&dir)?;
		let schema = build_memory_schema();
		let index = open_index(&dir, &schema)?;
		let fields =
			MemoryFields::from_schema(&schema)?;
		Ok(Self { index, fields })
	}

	/// Create an in-memory index (for tests)
	pub fn in_memory() -> MemoryResult<Self> {
		let schema = build_memory_schema();
		let index =
			Index::create_in_ram(schema.clone());
		let fields =
			MemoryFields::from_schema(&schema)?;
		Ok(Self { index, fields })
	}

	pub(crate) fn reader(
		&self,
	) -> MemoryResult<IndexReader> {
		self.index
			.reader_builder()
			.reload_policy(
				ReloadPolicy::OnCommitWithDelay,
			)
			.try_into()
			.map_err(map_tantivy)
	}
}

/// Open existing or create new Tantivy index
fn open_index(
	dir: &Path,
	schema: &tantivy::schema::Schema,
) -> MemoryResult<Index> {
	// Try open first, fall back to create
	match Index::open_in_dir(dir) {
		Ok(idx) => Ok(idx),
		Err(_) => Index::create_in_dir(
			dir,
			schema.clone(),
		)
		.map_err(map_tantivy),
	}
}

pub(crate) fn map_tantivy(
	err: impl std::fmt::Display,
) -> MemoryError {
	MemoryError::Search(err.to_string())
}

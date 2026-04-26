use std::path::{Path, PathBuf};

use tantivy::{Index, IndexReader, IndexWriter, ReloadPolicy};

use crate::indexer::search::error::SearchResult;
use crate::indexer::search::schema::{build_schema, SchemaFields};

/// Tantivy-backed search index for code symbols
pub struct SearchIndex {
	pub(crate) index: Index,
	pub(crate) fields: SchemaFields,
	pub(crate) index_path: Option<PathBuf>,
}

impl SearchIndex {
	pub fn in_memory() -> SearchResult<Self> {
		let schema = build_schema();
		let index = Index::create_in_ram(schema.clone());
		let fields = SchemaFields::from_schema(&schema)?;

		Ok(Self {
			index,
			fields,
			index_path: None,
		})
	}

	/// Open or create a persistent index at the given path
	pub fn open_or_create<P: AsRef<Path>>(
		path: P,
	) -> SearchResult<Self> {
		let path = path.as_ref();
		std::fs::create_dir_all(path)?;

		let schema = build_schema();
		let index = if path.join("meta.json").exists() {
			Index::open_in_dir(path)?
		} else {
			Index::create_in_dir(path, schema.clone())?
		};
		let fields = SchemaFields::from_schema(&schema)?;

		Ok(Self {
			index,
			fields,
			index_path: Some(path.to_path_buf()),
		})
	}

	pub fn writer(&self, heap_size: usize) -> SearchResult<IndexWriter> {
		Ok(self.index.writer(heap_size)?)
	}

	pub fn reader(&self) -> SearchResult<IndexReader> {
		Ok(self.index
			.reader_builder()
			.reload_policy(ReloadPolicy::OnCommitWithDelay)
			.try_into()?)
	}

	pub fn path(&self) -> Option<&Path> {
		self.index_path.as_deref()
	}
}

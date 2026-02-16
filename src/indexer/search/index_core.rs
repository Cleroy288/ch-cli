//! Core index operations: creation, opening, writer and reader access.

use std::path::{Path, PathBuf};

use tantivy::{Index, IndexReader, IndexWriter, ReloadPolicy};

use crate::indexer::search::error::SearchResult;
use crate::indexer::search::schema::{build_schema, SchemaFields};

/// The main search index for symbols
pub struct SearchIndex {
	pub(crate) index: Index,         // tantivy index instance
	pub(crate) fields: SchemaFields, // field handles
	pub(crate) index_path: Option<PathBuf>, // path for persistent index
}

impl SearchIndex {
	/// Create a new in-memory search index
	pub fn in_memory() -> SearchResult<Self> {
		let schema = build_schema(); // build the schema
		let index = Index::create_in_ram(schema.clone()); // create in-memory index
		let fields = SchemaFields::from_schema(&schema)?; // extract field handles

		Ok(Self {
			index,
			fields,
			index_path: None,
		})
	}

	/// Create or open a persistent search index at the given path
	pub fn open_or_create<P: AsRef<Path>>(path: P) -> SearchResult<Self> {
		let path = path.as_ref(); // path reference
		std::fs::create_dir_all(path)?;

		let schema = build_schema(); // build the schema
		let index = if path.join("meta.json").exists() {
			Index::open_in_dir(path)?
		} else {
			Index::create_in_dir(path, schema.clone())?
		};

		let fields = SchemaFields::from_schema(&schema)?; // extract field handles

		Ok(Self {
			index,
			fields,
			index_path: Some(path.to_path_buf()),
		})
	}

	/// Get an index writer for adding documents
	pub fn writer(&self, heap_size: usize) -> SearchResult<IndexWriter> {
		Ok(self.index.writer(heap_size)?)
	}

	/// Get an index reader for searching
	pub fn reader(&self) -> SearchResult<IndexReader> {
		Ok(self.index
			.reader_builder()
			.reload_policy(ReloadPolicy::OnCommitWithDelay)
			.try_into()?)
	}

	/// Get the index path (if persistent)
	pub fn path(&self) -> Option<&Path> {
		self.index_path.as_deref()
	}
}

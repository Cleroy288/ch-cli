//! Vector Store — Persistence and Query
//!
//! Persist operation and read-only accessors for the HNSW
//! vector store. See vector_store_load.rs for loading.

use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use instant_distance::Search;

use crate::retrieval::hybrid::vector_store::{
	SearchResult, VectorPoint, VectorStore,
};
use crate::retrieval::{RetrievalError, RetrievalResult};

impl VectorStore {
	/// Search for nearest neighbors
	pub fn search(
		&self,
		query: &[f32],
		k: usize,
	) -> Vec<SearchResult> {
		let index = match &self.index {
			Some(idx) => idx,
			None => return Vec::new(),
		};

		// create query point
		let query_point = VectorPoint {
			id: u64::MAX,
			vector: query.to_vec(),
			file_path: PathBuf::new(),
			line: 0,
			symbol_name: String::new(),
			symbol_kind: String::new(),
		};

		// search the HNSW index
		let mut search = Search::default();
		let results: Vec<_> = index
			.search(&query_point, &mut search)
			.take(k)
			.collect();

		results
			.into_iter()
			.map(|item| SearchResult {
				point: item.point.clone(),
				distance: item.distance,
			})
			.collect()
	}

	/// Get number of points
	pub fn len(&self) -> usize {
		self.points.len()
	}

	/// Check if empty
	pub fn is_empty(&self) -> bool {
		self.points.is_empty()
	}

	/// Check if index is built
	pub fn is_indexed(&self) -> bool {
		self.index.is_some()
	}
}

impl VectorStore {
	/// Persist to disk
	pub fn persist(&self) -> RetrievalResult<()> {
		let path = match &self.store_path {
			Some(p) => p,
			None => return Ok(()),
		};

		// serialize points
		let file = File::create(path)?;
		let writer = BufWriter::new(file);
		serde_json::to_writer(writer, &self.points)
			.map_err(|e| {
				RetrievalError::Io(std::io::Error::other(
					e.to_string(),
				))
			})?;

		Ok(())
	}
}

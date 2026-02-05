//! HNSW Vector Store
//!
//! Provides efficient approximate nearest neighbor search using
//! the HNSW (Hierarchical Navigable Small World) algorithm.

use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use instant_distance::{Builder, HnswMap, Search};
use serde::{Deserialize, Serialize};

use crate::retrieval::{RetrievalError, RetrievalResult};

/// A point in the vector space with associated metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VectorPoint {
	/// unique identifier
	pub id: u64,
	/// embedding vector
	pub vector: Vec<f32>,
	/// file path
	pub file_path: PathBuf,
	/// line number
	pub line: usize,
	/// symbol name
	pub symbol_name: String,
	/// symbol kind (function, struct, etc.)
	pub symbol_kind: String,
}

impl instant_distance::Point for VectorPoint {
	fn distance(&self, other: &Self) -> f32 {
		// cosine distance = 1 - cosine_similarity
		let dot: f32 = self
			.vector
			.iter()
			.zip(other.vector.iter())
			.map(|(a, b)| a * b)
			.sum();
		let norm_a: f32 = self.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
		let norm_b: f32 = other.vector.iter().map(|x| x * x).sum::<f32>().sqrt();

		if norm_a < 1e-10 || norm_b < 1e-10 {
			return 1.0;
		}

		1.0 - (dot / (norm_a * norm_b))
	}
}

/// Search result with distance
#[derive(Debug, Clone)]
pub struct SearchResult {
	/// the matched point
	pub point: VectorPoint,
	/// distance (lower is better)
	pub distance: f32,
}

/// HNSW-based vector store for semantic search
pub struct VectorStore {
	/// the HNSW index
	index: Option<HnswMap<VectorPoint, u64>>,
	/// all points for building index
	points: Vec<VectorPoint>,
	/// path for persistence
	store_path: Option<PathBuf>,
	/// next ID to assign
	next_id: u64,
}

impl VectorStore {
	/// Create a new empty vector store
	pub fn new() -> Self {
		Self {
			index: None,
			points: Vec::new(),
			store_path: None,
			next_id: 0,
		}
	}

	/// Create a vector store with persistence
	pub fn with_path(path: impl AsRef<Path>) -> RetrievalResult<Self> {
		let store_path = path.as_ref().to_path_buf();

		// try to load existing store
		if store_path.exists() {
			return Self::load(&store_path);
		}

		// create parent directory
		if let Some(parent) = store_path.parent() {
			fs::create_dir_all(parent)?;
		}

		Ok(Self {
			index: None,
			points: Vec::new(),
			store_path: Some(store_path),
			next_id: 0,
		})
	}

	/// Add a point to the store (index needs rebuild after)
	pub fn insert(&mut self, point: VectorPoint) {
		self.points.push(point);
		self.index = None; // invalidate index
	}

	/// Add multiple points
	pub fn insert_batch(&mut self, points: Vec<VectorPoint>) {
		self.points.extend(points);
		self.index = None;
	}

	/// Build the HNSW index
	pub fn build_index(&mut self) -> RetrievalResult<()> {
		if self.points.is_empty() {
			self.index = None;
			return Ok(());
		}

		// create values (IDs)
		let values: Vec<u64> = self.points.iter().map(|p| p.id).collect();

		// build index
		let hnsw = Builder::default().build(self.points.clone(), values);
		self.index = Some(hnsw);
		self.next_id = self.points.len() as u64;

		Ok(())
	}

	/// Search for nearest neighbors
	pub fn search(&self, query: &[f32], k: usize) -> Vec<SearchResult> {
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

		// search
		let mut search = Search::default();
		let results: Vec<_> = index.search(&query_point, &mut search).take(k).collect();

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

	/// Clear all points
	pub fn clear(&mut self) {
		self.points.clear();
		self.index = None;
		self.next_id = 0;
	}

	/// Generate next unique ID
	pub fn next_id(&mut self) -> u64 {
		let id = self.next_id;
		self.next_id += 1;
		id
	}

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
			.map_err(|e| RetrievalError::Io(std::io::Error::other(e.to_string())))?;

		Ok(())
	}

	/// Load from disk
	pub fn load(path: &Path) -> RetrievalResult<Self> {
		let file = File::open(path)?;
		let reader = BufReader::new(file);
		let points: Vec<VectorPoint> = serde_json::from_reader(reader)
			.map_err(|e| RetrievalError::Io(std::io::Error::other(e.to_string())))?;

		let next_id = points.iter().map(|p| p.id).max().unwrap_or(0) + 1;

		let mut store = Self {
			index: None,
			points,
			store_path: Some(path.to_path_buf()),
			next_id,
		};

		store.build_index()?;
		Ok(store)
	}
}

impl Default for VectorStore {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn create_point(id: u64, vector: Vec<f32>) -> VectorPoint {
		VectorPoint {
			id,
			vector,
			file_path: PathBuf::from("test.rs"),
			line: 1,
			symbol_name: format!("symbol_{}", id),
			symbol_kind: "function".to_string(),
		}
	}

	#[test]
	fn test_insert_and_search() {
		let mut store = VectorStore::new();

		// insert points
		store.insert(create_point(0, vec![1.0, 0.0, 0.0]));
		store.insert(create_point(1, vec![0.0, 1.0, 0.0]));
		store.insert(create_point(2, vec![0.0, 0.0, 1.0]));

		// build index
		store.build_index().unwrap();

		// search
		let results = store.search(&[1.0, 0.0, 0.0], 2);
		assert_eq!(results.len(), 2);
		assert_eq!(results[0].point.id, 0);
	}

	#[test]
	fn test_empty_store() {
		let store = VectorStore::new();
		let results = store.search(&[1.0, 0.0, 0.0], 5);
		assert!(results.is_empty());
	}
}

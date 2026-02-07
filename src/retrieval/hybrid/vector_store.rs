//! HNSW Vector Store
//!
//! Provides efficient approximate nearest neighbor search using
//! the HNSW (Hierarchical Navigable Small World) algorithm.

use std::fs;
use std::path::{Path, PathBuf};

use instant_distance::HnswMap;
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
		let norm_a: f32 =
			self.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
		let norm_b: f32 =
			other.vector.iter().map(|x| x * x).sum::<f32>().sqrt();

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
	pub(crate) index: Option<HnswMap<VectorPoint, u64>>,
	/// all points for building index
	pub(crate) points: Vec<VectorPoint>,
	/// path for persistence
	pub(crate) store_path: Option<PathBuf>,
	/// next ID to assign
	pub(crate) next_id: u64,
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
	pub fn with_path(
		path: impl AsRef<Path>,
	) -> RetrievalResult<Self> {
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
}

impl Default for VectorStore {
	fn default() -> Self {
		Self::new()
	}
}


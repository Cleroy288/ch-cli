//! HNSW Vector Store
//!
//! Provides efficient approximate nearest neighbor search using
//! hnsw_rs with cosine distance.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::retrieval::RetrievalResult;

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

/// Lightweight metadata from a VectorPoint (no vector).
/// Avoids cloning the 768-dim Vec<f32> on every search hit.
#[derive(Debug, Clone)]
pub struct PointMeta {
	/// unique identifier (index into VectorStore.points)
	pub id: u64,
	/// file path
	pub file_path: PathBuf,
	/// line number
	pub line: usize,
	/// symbol name
	pub symbol_name: String,
	/// symbol kind (function, struct, etc.)
	pub symbol_kind: String,
}

impl PointMeta {
	/// Build metadata from a VectorPoint reference.
	/// Copies only the lightweight fields, skipping the
	/// embedding vector entirely.
	pub fn from_point(point: &VectorPoint) -> Self {
		Self {
			id: point.id,
			file_path: point.file_path.clone(),
			line: point.line,
			symbol_name: point.symbol_name.clone(),
			symbol_kind: point.symbol_kind.clone(),
		}
	}
}

/// Search result with distance (vector-free)
#[derive(Debug, Clone)]
pub struct SearchResult {
	/// matched point metadata (no embedding vector)
	pub point: PointMeta,
	/// distance (lower is better)
	pub distance: f32,
}

/// HNSW-based vector store for semantic search
pub struct VectorStore {
	/// all points (metadata + vectors)
	pub(crate) points: Vec<VectorPoint>,
	/// whether index is built
	pub(crate) indexed: bool,
	/// path for persistence
	pub(crate) store_path: Option<PathBuf>,
	/// next ID to assign
	pub(crate) next_id: u64,
	/// precomputed sigma (k-th NN distance) for LS
	pub(crate) sigma_values: Vec<f32>,
	/// mean vector for anisotropy correction
	pub(crate) mean_vector: Option<Vec<f32>>,
}

impl VectorStore {
	/// Create a new empty vector store
	pub fn new() -> Self {
		Self {
			points: Vec::new(),
			indexed: false,
			store_path: None,
			next_id: 0,
			sigma_values: Vec::new(),
			mean_vector: None,
		}
	}

	/// Create a vector store with persistence.
	/// Tries `.bin` path first, then legacy `.json` fallback.
	pub fn with_path(
		path: impl AsRef<Path>,
	) -> RetrievalResult<Self> {
		let store_path = path.as_ref().to_path_buf();

		// try to load existing store (bin or json)
		if let Some(found) =
			resolve_store_path(&store_path)
		{
			return Self::load(&found);
		}

		// create parent directory
		if let Some(parent) = store_path.parent() {
			std::fs::create_dir_all(parent)?;
		}

		Ok(Self {
			points: Vec::new(),
			indexed: false,
			store_path: Some(store_path),
			next_id: 0,
			sigma_values: Vec::new(),
			mean_vector: None,
		})
	}
}

/// Resolve which store file to load. Returns the path
/// that actually exists: first the given path, then
/// the alternate extension (.bin <-> .json) for migration.
fn resolve_store_path(path: &Path) -> Option<PathBuf> {
	if path.exists() {
		return Some(path.to_path_buf());
	}

	// Try alternate extension for migration
	let alt = match path.extension().and_then(|ext| ext.to_str()) {
		Some("bin") => path.with_extension("json"),
		Some("json") => path.with_extension("bin"),
		_ => return None,
	};

	if alt.exists() {
		return Some(alt);
	}
	None
}

impl Default for VectorStore {
	fn default() -> Self {
		Self::new()
	}
}

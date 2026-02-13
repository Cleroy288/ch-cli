//! Embedding format versioning.
//!
//! Tracks the embedding text format version so stale
//! vector caches are invalidated when the format changes.

use std::path::Path;

/// Current embedding format version.
/// Bump when `symbol_to_embedding_text()` or serialization
/// format changes. v7: bincode + HNSW persistence.
pub const EMBEDDING_FORMAT_VERSION: u32 = 7;

/// Version file name inside `.rustean-index/`
const VERSION_FILE: &str = "embedding_version";

/// Check if cached vectors match current format version
pub fn is_cache_current(index_dir: &Path) -> bool {
	let path = index_dir.join(VERSION_FILE);
	match std::fs::read_to_string(&path) {
		Ok(content) => {
			let version = content.trim().parse::<u32>();
			version == Ok(EMBEDDING_FORMAT_VERSION)
		}
		Err(_) => false,
	}
}

/// Write current version to `.rustean-index/embedding_version`
pub fn write_version(index_dir: &Path) {
	let path = index_dir.join(VERSION_FILE);
	let _ = std::fs::write(
		&path,
		EMBEDDING_FORMAT_VERSION.to_string(),
	);
}

/// Delete cached vector files and index state to
/// force full re-index + re-embedding.
/// Also removes Tantivy and HNSW dumps.
pub fn invalidate_vectors(index_dir: &Path) {
	invalidate_vector_files(index_dir);
	invalidate_enriched_files(index_dir);
	let _ = std::fs::remove_file(
		index_dir.join("state.json"),
	);
	let _ = std::fs::remove_dir_all(
		index_dir.join("tantivy"),
	);
}

/// Remove primary vector files (JSON + bincode + HNSW)
fn invalidate_vector_files(index_dir: &Path) {
	// Legacy JSON
	let _ = std::fs::remove_file(
		index_dir.join("vectors.json"),
	);
	let _ = std::fs::remove_file(
		index_dir.join("vectors_mean.json"),
	);
	let _ = std::fs::remove_file(
		index_dir.join("vectors_sigma.json"),
	);
	// Bincode
	let _ = std::fs::remove_file(
		index_dir.join("vectors.bin"),
	);
	let _ = std::fs::remove_file(
		index_dir.join("vectors_mean.bin"),
	);
	let _ = std::fs::remove_file(
		index_dir.join("vectors_sigma.bin"),
	);
	// HNSW dump directory
	let _ = std::fs::remove_dir_all(
		index_dir.join("hnsw"),
	);
}

/// Remove enriched vector files (JSON + bincode + HNSW)
fn invalidate_enriched_files(index_dir: &Path) {
	let _ = std::fs::remove_file(
		index_dir.join("enriched_vectors.json"),
	);
	let _ = std::fs::remove_file(
		index_dir.join("enriched_vectors_mean.json"),
	);
	let _ = std::fs::remove_file(
		index_dir.join("enriched_vectors.bin"),
	);
	let _ = std::fs::remove_file(
		index_dir.join("enriched_vectors_mean.bin"),
	);
	let _ = std::fs::remove_file(
		index_dir.join("enriched_vectors_sigma.bin"),
	);
}

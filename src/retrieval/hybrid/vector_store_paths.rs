//! Vector Store — Path Helpers
//!
//! Derives sibling file paths for mean vectors, sigma
//! values, HNSW dumps, and legacy JSON files from the
//! main vector store path.

use std::path::{Path, PathBuf};

/// Derive mean vector path from vectors path.
/// e.g. `/path/vectors.bin` -> `/path/vectors_mean.bin`
pub(crate) fn mean_vector_path(
	vectors_path: &Path,
) -> PathBuf {
	sibling_path(vectors_path, "mean")
}

/// Derive sigma values path from vectors path.
/// e.g. `/path/vectors.bin` -> `/path/vectors_sigma.bin`
pub(crate) fn sigma_values_path(
	vectors_path: &Path,
) -> PathBuf {
	sibling_path(vectors_path, "sigma")
}

/// Derive HNSW dump directory from vectors path.
/// e.g. `/path/vectors.bin` -> `/path/hnsw/`
pub(crate) fn hnsw_dump_dir(
	vectors_path: &Path,
) -> PathBuf {
	let parent = vectors_path
		.parent()
		.unwrap_or(Path::new("."));
	parent.join("hnsw")
}

/// Basename used for HNSW file dump (hnsw_rs writes
/// two files: `{basename}.hnsw.data` and
/// `{basename}.hnsw.graph`)
pub(crate) const HNSW_DUMP_BASENAME: &str =
	"hnsw_graph";

/// Build a sibling file path with given suffix.
/// e.g. (`/path/vectors.bin`, "mean") ->
///      `/path/vectors_mean.bin`
fn sibling_path(
	base: &Path,
	suffix: &str,
) -> PathBuf {
	let stem = base
		.file_stem()
		.and_then(|seg| seg.to_str())
		.unwrap_or("vectors");
	let parent =
		base.parent().unwrap_or(Path::new("."));
	parent.join(format!("{}_{}.bin", stem, suffix))
}

/// Build a legacy JSON sibling path for migration.
/// e.g. (`/path/vectors.bin`, "mean") ->
///      `/path/vectors_mean.json`
pub(crate) fn legacy_json_sibling(
	base: &Path,
	suffix: &str,
) -> PathBuf {
	let stem = base
		.file_stem()
		.and_then(|seg| seg.to_str())
		.unwrap_or("vectors");
	let parent =
		base.parent().unwrap_or(Path::new("."));
	parent.join(format!("{}_{}.json", stem, suffix))
}

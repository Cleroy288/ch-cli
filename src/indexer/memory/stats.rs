//! Stats computation and caching for memory store.

use std::fs;
use std::path::Path;

use crate::domain::errors::memory::{
	MemoryError, MemoryResult,
};

use super::paths;
use super::store_helpers;
use super::types::MemoryStats;

/// Compute fresh stats from the session files
pub fn compute_stats(
	root: &Path,
) -> MemoryResult<MemoryStats> {
	let dir = paths::sessions_dir(root);
	let files =
		store_helpers::sorted_session_files(&dir);
	let sessions = files.len();
	let mut total = 0_usize;
	let mut size_bytes = 0_u64;
	let mut last_updated = 0_u64;

	for path in &files {
		let items = store_helpers::read_lines(path);
		total += items.len();
		update_last_ts(&items, &mut last_updated);
		size_bytes += file_size(path);
	}

	let stats = MemoryStats {
		total,
		sessions,
		size_bytes,
		last_updated,
	};
	save_stats(root, &stats)?;
	Ok(stats)
}

/// Load cached stats from meta file
pub fn load_cached(
	root: &Path,
) -> MemoryResult<MemoryStats> {
	let path = paths::meta_file(root);
	let content = fs::read_to_string(&path)?;
	serde_json::from_str(&content).map_err(
		|err| MemoryError::Serialize(
			err.to_string(),
		),
	)
}

/// Save stats to meta file
pub fn save_stats(
	root: &Path,
	stats: &MemoryStats,
) -> MemoryResult<()> {
	store_helpers::ensure_dirs(root)?;
	let path = paths::meta_file(root);
	let json =
		serde_json::to_string_pretty(stats)
			.map_err(|err| {
				MemoryError::Serialize(
					err.to_string(),
				)
			})?;
	fs::write(path, json)?;
	Ok(())
}

/// Update last_updated from interaction timestamps
fn update_last_ts(
	items: &[crate::domain::memory::Interaction],
	last: &mut u64,
) {
	for item in items {
		if item.timestamp > *last {
			*last = item.timestamp;
		}
	}
}

/// Get file size in bytes, 0 on error
fn file_size(path: &Path) -> u64 {
	fs::metadata(path)
		.map(|meta| meta.len())
		.unwrap_or(0)
}

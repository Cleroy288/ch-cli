//! JSONL persistence for memory interactions.
//!
//! Each session gets its own `.jsonl` file.
//! One interaction per line, append-only.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use crate::domain::errors::memory::{
	MemoryError, MemoryResult,
};
use crate::domain::memory::Interaction;

use super::paths;
use super::store_helpers;

/// Append an interaction to its session file
pub fn append(
	root: &Path,
	interaction: &Interaction,
) -> MemoryResult<()> {
	store_helpers::ensure_dirs(root)?;
	let file_path = paths::session_file(
		root,
		&interaction.session_id,
	);
	let json = serde_json::to_string(interaction)
		.map_err(|err| {
			MemoryError::Serialize(err.to_string())
		})?;
	let mut file = OpenOptions::new()
		.create(true)
		.append(true)
		.open(file_path)?;
	writeln!(file, "{json}")?;
	Ok(())
}

/// Load all interactions for a session
pub fn load_session(
	root: &Path,
	session_id: &str,
) -> MemoryResult<Vec<Interaction>> {
	let file_path =
		paths::session_file(root, session_id);
	if !file_path.exists() {
		return Err(MemoryError::SessionNotFound(
			session_id.to_string(),
		));
	}
	Ok(store_helpers::read_lines(&file_path))
}

/// Load recent interactions across all sessions
pub fn load_recent(
	root: &Path,
	limit: usize,
) -> MemoryResult<Vec<Interaction>> {
	let dir = paths::sessions_dir(root);
	let files =
		store_helpers::sorted_session_files(&dir);
	let mut all = Vec::new();
	for path in files {
		let items = store_helpers::read_lines(&path);
		all.extend(items);
		if all.len() >= limit {
			break;
		}
	}
	all.truncate(limit);
	Ok(all)
}

/// List all session IDs, newest first
pub fn list_sessions(
	root: &Path,
) -> MemoryResult<Vec<String>> {
	let dir = paths::sessions_dir(root);
	let files =
		store_helpers::sorted_session_files(&dir);
	let ids: Vec<String> = files
		.iter()
		.filter_map(|path| {
			store_helpers::session_id_from_path(path)
		})
		.collect();
	Ok(ids)
}

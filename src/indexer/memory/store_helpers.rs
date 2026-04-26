use std::fs;
use std::path::{Path, PathBuf};

use crate::domain::errors::memory::MemoryResult;
use crate::domain::memory::Interaction;

use super::paths;
use super::store_filters::{
	file_modified_time, is_jsonl_file,
};

/// Ensure all memory directories exist
pub fn ensure_dirs(
	root: &Path,
) -> MemoryResult<()> {
	fs::create_dir_all(
		paths::sessions_dir(root),
	)?;
	Ok(())
}

pub fn parse_line(
	line: &str,
) -> Option<Interaction> {
	let trimmed = line.trim();
	if trimmed.is_empty() {
		return None;
	}
	serde_json::from_str(trimmed).ok()
}

pub fn read_lines(
	path: &Path,
) -> Vec<Interaction> {
	let Ok(content) = fs::read_to_string(path)
	else {
		return Vec::new();
	};
	content
		.lines()
		.filter_map(parse_line)
		.collect()
}

/// List session files sorted by mod time
pub fn sorted_session_files(
	dir: &Path,
) -> Vec<PathBuf> {
	let Ok(entries) = fs::read_dir(dir) else {
		return Vec::new();
	};
	let mut files: Vec<PathBuf> = entries
		.filter_map(|entry| entry.ok())
		.map(|entry| entry.path())
		.filter(|path| is_jsonl_file(path))
		.collect();
	files.sort_by_key(|path| {
		file_modified_time(path)
	});
	files.reverse(); // newest first
	files
}

/// Extract session ID from a JSONL file path
pub fn session_id_from_path(
	path: &Path,
) -> Option<String> {
	path.file_stem()
		.and_then(|stem| stem.to_str())
		.map(String::from)
}

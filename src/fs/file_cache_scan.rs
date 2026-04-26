use std::path::Path;

use super::entry::FsEntry;

/// Dirs to skip in picker scan (internal/build)
const SKIP_DIRS: &[&str] = &[
	".git", ".hg", ".svn",
	"target", "node_modules", ".cache",
];

/// Max recursion depth for picker scan
const MAX_SCAN_DEPTH: usize = 10;

/// Scan project root, skipping internal dirs
pub(crate) fn scan_root() -> Vec<FsEntry> {
	let mut entries = Vec::new();
	scan_filtered(Path::new("."), &mut entries, 0);
	entries.sort_by(sort_dirs_first);
	entries
}

/// Recursive scan skipping SKIP_DIRS
fn scan_filtered(
	dir: &Path,
	entries: &mut Vec<FsEntry>,
	depth: usize,
) {
	if depth > MAX_SCAN_DEPTH {
		return;
	}
	let Ok(read_dir) = std::fs::read_dir(dir) else {
		return;
	};
	for item in read_dir.flatten() {
		process_item(&item, entries, depth);
	}
}

/// Process one dir entry, skip internal dirs
fn process_item(
	item: &std::fs::DirEntry,
	entries: &mut Vec<FsEntry>,
	depth: usize,
) {
	let path = item.path();
	let Ok(meta) = item.metadata() else {
		return;
	};
	if meta.is_dir() {
		let name = file_name_str(&path);
		entries.push(FsEntry::new(path.clone(), true));
		if !SKIP_DIRS.contains(&name) {
			scan_filtered(&path, entries, depth + 1);
		}
	} else if meta.is_file() {
		entries.push(FsEntry::new(path, false));
	}
}

/// Extract filename as &str for skip check
fn file_name_str(path: &Path) -> &str {
	path.file_name()
		.and_then(|n| n.to_str())
		.unwrap_or("")
}

/// Sort: directories first, then alphabetical
fn sort_dirs_first(
	lhs: &FsEntry,
	rhs: &FsEntry,
) -> std::cmp::Ordering {
	match (lhs.is_dir, rhs.is_dir) {
		(true, false) => std::cmp::Ordering::Less,
		(false, true) => std::cmp::Ordering::Greater,
		_ => lhs.name.to_lowercase().cmp(
			&rhs.name.to_lowercase(),
		),
	}
}

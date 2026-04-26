use std::path::{Path, PathBuf};
use std::process::Command;

const SKIP_DIRS: &[&str] = &[
	"node_modules", "target", ".cache",
	"vendor", "dist", "build",
];
const MAX_DEPTH: u32 = 3;

/// Find git repos: CWD (if repo) + children.
pub fn discover_git_repos(
	root: &Path,
) -> Vec<PathBuf> {
	let mut repos = Vec::new();
	if let Some(top) = git_toplevel(root) {
		if paths_equal(&top, root) {
			repos.push(top);
		}
	}
	scan_children(root, 0, &mut repos);
	repos
}

fn git_toplevel(dir: &Path) -> Option<PathBuf> {
	let out = Command::new("git")
		.arg("rev-parse")
		.arg("--show-toplevel")
		.current_dir(dir)
		.output()
		.ok()?;
	if !out.status.success() {
		return None;
	}
	let s = String::from_utf8_lossy(&out.stdout);
	let trimmed = s.trim();
	if trimmed.is_empty() {
		return None;
	}
	Some(PathBuf::from(trimmed))
}

fn paths_equal(a: &Path, b: &Path) -> bool {
	a.canonicalize().ok()
		== b.canonicalize().ok()
}

fn scan_children(
	dir: &Path,
	depth: u32,
	repos: &mut Vec<PathBuf>,
) {
	if depth > MAX_DEPTH {
		return;
	}
	let Ok(entries) = std::fs::read_dir(dir) else {
		return;
	};
	for entry in entries.flatten() {
		visit_entry(&entry, depth, repos);
	}
}

fn visit_entry(
	entry: &std::fs::DirEntry,
	depth: u32,
	repos: &mut Vec<PathBuf>,
) {
	let path = entry.path();
	if !path.is_dir() {
		return;
	}
	let skip = path
		.file_name()
		.and_then(|n| n.to_str())
		.is_some_and(|n| {
			n.starts_with('.')
				|| SKIP_DIRS.contains(&n)
		});
	if skip {
		return;
	}
	if path.join(".git").exists() {
		repos.push(path);
	} else {
		scan_children(&path, depth + 1, repos);
	}
}

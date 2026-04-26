use std::fs;
use std::path::Path;

use crate::domain::repo_info::RepoEntry;

use super::git_remote;

/// Directories to skip during scanning
const SKIP_NAMES: &[&str] = &[
	".git", "node_modules", "target", ".cache",
];

/// Bundled context for recursive scan
struct ScanCtx<'a> {
	root: &'a Path,
	max_depth: u32,
	out: Vec<RepoEntry>,
}

/// Scan subdirectories for git repositories
pub fn discover_repos(
	root: &Path,
	max_depth: u32,
) -> Vec<RepoEntry> {
	let mut ctx = ScanCtx {
		root, max_depth, out: Vec::new(),
	};
	scan_children(&mut ctx, root, 0);
	ctx.out
}

/// Scan children of a directory for repos
fn scan_children(
	ctx: &mut ScanCtx,
	dir: &Path,
	depth: u32,
) {
	if depth > ctx.max_depth {
		return;
	}
	let Ok(entries) = fs::read_dir(dir) else {
		return;
	};
	for entry in entries.flatten() {
		visit_entry(ctx, &entry, depth);
	}
}

/// Visit a single entry: check if repo or recurse
fn visit_entry(
	ctx: &mut ScanCtx,
	entry: &fs::DirEntry,
	depth: u32,
) {
	let path = entry.path();
	let skip = path
		.file_name()
		.and_then(|n| n.to_str())
		.is_some_and(|n| SKIP_NAMES.contains(&n));
	if !path.is_dir() || skip {
		return;
	}
	if let Some(repo) =
		try_parse_repo(ctx.root, &path)
	{
		ctx.out.push(repo);
	} else {
		scan_children(ctx, &path, depth + 1);
	}
}

/// Try to parse a directory as a git repo
fn try_parse_repo(
	root: &Path,
	dir: &Path,
) -> Option<RepoEntry> {
	if !dir.join(".git").exists() {
		return None;
	}
	let info = git_remote::detect_repo_info(dir)?;
	let folder = dir
		.file_name()?
		.to_str()?
		.to_string();
	let rel = dir.strip_prefix(root)
		.ok()?
		.to_string_lossy()
		.to_string();
	let path = format!("./{rel}");
	Some(RepoEntry {
		folder,
		path,
		host: info.host,
		workspace: info.workspace,
		repo_slug: info.repo_slug,
	})
}

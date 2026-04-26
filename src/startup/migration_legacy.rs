use std::fs;
use std::path::Path;

/// Copy a subdirectory from src to dst via rename
pub(crate) fn copy_subdir(
	src: &Path,
	dst: &Path,
	name: &str,
) {
	let from = src.join(name);
	let to = dst.join(name);
	if from.exists() && !to.exists() {
		let _ = fs::rename(&from, &to);
	}
}

/// Copy config.json from local to centralized
pub(crate) fn copy_config_file(
	src: &Path,
	dst: &Path,
) {
	let from = src.join("config.json");
	let to = dst.join("config.json");
	if from.exists() && !to.exists() {
		let _ = fs::rename(&from, &to);
	}
}

pub(crate) fn move_old_dirs(
	root: &Path,
	data: &Path,
) {
	let old_idx = root.join(super::migration::OLD_INDEX_DIR);
	if old_idx.exists() {
		let _ =
			fs::rename(&old_idx, data.join("index"));
	}
	let old_mem =
		root.join(super::migration::OLD_MEMORY_DIR);
	if old_mem.exists() {
		let _ = fs::rename(
			&old_mem, data.join("memory"),
		);
	}
	migrate_claude_files(data);
}

pub(crate) fn move_old_creds(
	root: &Path,
	data: &Path,
) {
	let old =
		root.join(super::migration::OLD_CREDS_FILE);
	if old.exists() {
		let _ = fs::rename(
			&old, data.join("credentials.json"),
		);
	}
}

pub(crate) fn migrate_claude_files(data: &Path) {
	let index = data.join("index");
	for name in
		["claude_model.json", "claude_session.json"]
	{
		let src = index.join(name);
		let dst = data.join(name);
		if src.exists() && !dst.exists() {
			let _ = fs::rename(&src, &dst);
		}
	}
}

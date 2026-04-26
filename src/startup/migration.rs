use std::fs;
use std::path::Path;

use crate::domain::data_paths;

use super::migration_legacy::{
	copy_config_file, copy_subdir,
	migrate_claude_files, move_old_creds,
	move_old_dirs,
};

pub(crate) const OLD_INDEX_DIR: &str =
	".rustean-index";
pub(crate) const OLD_MEMORY_DIR: &str =
	".rustean-memory";
pub(crate) const OLD_CREDS_FILE: &str =
	".rustean-credentials.json";

/// Run all migration steps in order:
/// 1. Legacy scattered files → .rustean-data/
/// 2. .rustean-data/ → ~/.config/rustean/projects/
pub fn migrate_data_layout(root: &Path) {
	migrate_legacy_to_local(root);
	migrate_local_to_centralized(root);
}

/// Step 1: old scattered layout → .rustean-data/
fn migrate_legacy_to_local(root: &Path) {
	let data = root.join(data_paths::LEGACY_DATA_DIR);
	if data.exists() {
		migrate_claude_files(&data);
		return;
	}
	if !has_old_layout(root) {
		return;
	}
	let _ = fs::create_dir_all(&data);
	move_old_dirs(root, &data);
	move_old_creds(root, &data);
}

/// Step 2: .rustean-data/ → centralized dir
fn migrate_local_to_centralized(root: &Path) {
	let local = root.join(data_paths::LEGACY_DATA_DIR);
	if !local.exists() {
		return;
	}
	let target = data_paths::data_dir(root);
	let _ = fs::create_dir_all(&target);
	copy_subdir(&local, &target, "index");
	copy_subdir(&local, &target, "memory");
	copy_config_file(&local, &target);
	let _ = fs::remove_dir_all(&local);
}

fn has_old_layout(root: &Path) -> bool {
	root.join(OLD_INDEX_DIR).exists()
		|| root.join(OLD_MEMORY_DIR).exists()
		|| root.join(OLD_CREDS_FILE).exists()
}

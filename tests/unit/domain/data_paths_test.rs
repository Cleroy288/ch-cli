//! Tests for centralized data path definitions.

use std::path::Path;

use rustean::domain::data_paths;
use rustean::domain::data_paths_dirs;

/// project_hash returns 8-char hex string
#[test]
fn project_hash_length() {
	let hash =
		data_paths::project_hash(Path::new("/tmp"));
	assert_eq!(hash.len(), 8);
	assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
}

/// Same path produces same hash
#[test]
fn project_hash_deterministic() {
	let root = Path::new("/tmp/test-project");
	let h1 = data_paths::project_hash(root);
	let h2 = data_paths::project_hash(root);
	assert_eq!(h1, h2);
}

/// data_dir contains projects/{hash}
#[test]
fn data_dir_contains_hash() {
	let root = Path::new("/tmp/proj");
	let hash = data_paths::project_hash(root);
	let dir = data_paths::data_dir(root);
	let dir_str = dir.to_string_lossy();
	assert!(dir_str.contains("projects"));
	assert!(dir_str.contains(&hash));
}

/// index_dir is inside data_dir
#[test]
fn index_dir_inside_data() {
	let root = Path::new("/tmp/proj");
	let dir = data_paths_dirs::index_dir(root);
	assert!(dir.starts_with(
		data_paths::data_dir(root),
	));
	assert!(dir.ends_with("index"));
}

/// memory_dir is inside data_dir
#[test]
fn memory_dir_inside_data() {
	let root = Path::new("/tmp/proj");
	let dir = data_paths_dirs::memory_dir(root);
	assert!(dir.starts_with(
		data_paths::data_dir(root),
	));
	assert!(dir.ends_with("memory"));
}

/// config_file is inside data_dir
#[test]
fn config_file_inside_data() {
	let root = Path::new("/tmp/proj");
	let file = data_paths_dirs::config_file(root);
	assert!(file.starts_with(
		data_paths::data_dir(root),
	));
	assert!(file.ends_with("config.json"));
}

/// global_config_file ends with config.json
#[test]
fn global_config_ends_with_config() {
	let file = data_paths_dirs::global_config_file();
	assert!(file.ends_with("config.json"));
}

/// manifest_file ends with projects.json
#[test]
fn manifest_ends_with_projects() {
	let file = data_paths_dirs::manifest_file();
	assert!(file.ends_with("projects.json"));
}

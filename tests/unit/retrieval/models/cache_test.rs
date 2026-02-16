use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use tempfile::tempdir;

use rustean::retrieval::models::cache::{
	CachedModel, ModelCache,
};

fn create_test_model(
	model_id: &str,
	cache_dir: &Path,
) -> CachedModel {
	let timestamp = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_secs();

	CachedModel {
		model_id: model_id.to_string(),
		weights_path: cache_dir
			.join(format!("{}.safetensors", model_id)),
		tokenizer_path: cache_dir.join(format!(
			"{}.tokenizer.json",
			model_id
		)),
		config_path: cache_dir.join(format!(
			"{}.config.json",
			model_id
		)),
		downloaded_at: timestamp,
		size_bytes: 1024 * 1024,
	}
}

#[test]
fn test_new_creates_directory() {
	let temp_dir = tempdir().unwrap();
	let cache_path = temp_dir.path().join("cache");
	let cache = ModelCache::new(&cache_path).unwrap();
	assert!(cache_path.exists());
	assert_eq!(cache.cache_dir(), cache_path);
}

#[test]
fn test_cache_dir_returns_path() {
	let temp_dir = tempdir().unwrap();
	let cache =
		ModelCache::new(temp_dir.path()).unwrap();
	assert_eq!(cache.cache_dir(), temp_dir.path());
}

#[test]
fn test_is_cached_nonexistent() {
	let temp_dir = tempdir().unwrap();
	let cache =
		ModelCache::new(temp_dir.path()).unwrap();
	assert!(!cache.is_cached("nonexistent-model"));
}

#[test]
fn test_is_cached_missing_files() {
	let temp_dir = tempdir().unwrap();
	let mut cache =
		ModelCache::new(temp_dir.path()).unwrap();
	let model =
		create_test_model("test-model", temp_dir.path());
	cache.add(model).unwrap();
	assert!(!cache.is_cached("test-model"));
}

#[test]
fn test_is_cached_all_files_exist() {
	let temp_dir = tempdir().unwrap();
	let mut cache =
		ModelCache::new(temp_dir.path()).unwrap();
	let model =
		create_test_model("test-model", temp_dir.path());
	fs::write(&model.weights_path, b"weights").unwrap();
	fs::write(&model.tokenizer_path, b"tok").unwrap();
	fs::write(&model.config_path, b"cfg").unwrap();
	cache.add(model).unwrap();
	assert!(cache.is_cached("test-model"));
}

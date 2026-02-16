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
fn test_get_nonexistent() {
	let temp_dir = tempdir().unwrap();
	let cache =
		ModelCache::new(temp_dir.path()).unwrap();
	assert!(cache.get("nonexistent").is_none());
}

#[test]
fn test_get_existing() {
	let temp_dir = tempdir().unwrap();
	let mut cache =
		ModelCache::new(temp_dir.path()).unwrap();
	let model =
		create_test_model("test-model", temp_dir.path());
	let id = model.model_id.clone();
	cache.add(model).unwrap();
	let cached = cache.get(&id).unwrap();
	assert_eq!(cached.model_id, id);
	assert_eq!(cached.size_bytes, 1024 * 1024);
}

#[test]
fn test_add_model() {
	let temp_dir = tempdir().unwrap();
	let mut cache =
		ModelCache::new(temp_dir.path()).unwrap();
	let model =
		create_test_model("test-model", temp_dir.path());
	cache.add(model).unwrap();
	assert!(cache.get("test-model").is_some());
}

#[test]
fn test_add_persists_metadata() {
	let temp_dir = tempdir().unwrap();
	let mut cache =
		ModelCache::new(temp_dir.path()).unwrap();
	let model =
		create_test_model("test-model", temp_dir.path());
	cache.add(model).unwrap();
	let cache2 =
		ModelCache::new(temp_dir.path()).unwrap();
	assert!(cache2.get("test-model").is_some());
}

#[test]
fn test_remove_model() {
	let temp_dir = tempdir().unwrap();
	let mut cache =
		ModelCache::new(temp_dir.path()).unwrap();
	let model =
		create_test_model("test-model", temp_dir.path());
	fs::write(&model.weights_path, b"w").unwrap();
	fs::write(&model.tokenizer_path, b"t").unwrap();
	fs::write(&model.config_path, b"c").unwrap();
	cache.add(model.clone()).unwrap();
	cache.remove("test-model").unwrap();
	assert!(cache.get("test-model").is_none());
	assert!(!model.weights_path.exists());
	assert!(!model.tokenizer_path.exists());
	assert!(!model.config_path.exists());
}

#[test]
fn test_remove_persists_metadata() {
	let temp_dir = tempdir().unwrap();
	let mut cache =
		ModelCache::new(temp_dir.path()).unwrap();
	let model =
		create_test_model("test-model", temp_dir.path());
	cache.add(model).unwrap();
	cache.remove("test-model").unwrap();
	let cache2 =
		ModelCache::new(temp_dir.path()).unwrap();
	assert!(cache2.get("test-model").is_none());
}

#[test]
fn test_total_size_empty() {
	let temp_dir = tempdir().unwrap();
	let cache =
		ModelCache::new(temp_dir.path()).unwrap();
	assert_eq!(cache.total_size(), 0);
}

#[test]
fn test_total_size_multiple_models() {
	let temp_dir = tempdir().unwrap();
	let mut cache =
		ModelCache::new(temp_dir.path()).unwrap();
	let mut m1 =
		create_test_model("model-1", temp_dir.path());
	m1.size_bytes = 1000;
	cache.add(m1).unwrap();
	let mut m2 =
		create_test_model("model-2", temp_dir.path());
	m2.size_bytes = 2000;
	cache.add(m2).unwrap();
	let mut m3 =
		create_test_model("model-3", temp_dir.path());
	m3.size_bytes = 3000;
	cache.add(m3).unwrap();
	assert_eq!(cache.total_size(), 6000);
}

#[test]
fn test_list_empty() {
	let temp_dir = tempdir().unwrap();
	let cache =
		ModelCache::new(temp_dir.path()).unwrap();
	assert_eq!(cache.list().len(), 0);
}

#[test]
fn test_list_multiple_models() {
	let temp_dir = tempdir().unwrap();
	let mut cache =
		ModelCache::new(temp_dir.path()).unwrap();
	cache
		.add(create_test_model(
			"model-1",
			temp_dir.path(),
		))
		.unwrap();
	cache
		.add(create_test_model(
			"model-2",
			temp_dir.path(),
		))
		.unwrap();
	cache
		.add(create_test_model(
			"model-3",
			temp_dir.path(),
		))
		.unwrap();
	let list = cache.list();
	assert_eq!(list.len(), 3);
	let ids: Vec<&str> = list
		.iter()
		.map(|m| m.model_id.as_str())
		.collect();
	assert!(ids.contains(&"model-1"));
	assert!(ids.contains(&"model-2"));
	assert!(ids.contains(&"model-3"));
}

//! Model Weight Caching
//!
//! Provides caching for downloaded model weights to avoid re-downloading.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::{ModelError, ModelResult};

/// Metadata about a cached model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedModel {
	/// model identifier
	pub model_id: String,
	/// path to weights file
	pub weights_path: PathBuf,
	/// path to tokenizer file
	pub tokenizer_path: PathBuf,
	/// path to config file
	pub config_path: PathBuf,
	/// download timestamp
	pub downloaded_at: u64,
	/// file size in bytes
	pub size_bytes: u64,
}

/// Cache for model weights
#[derive(Debug)]
pub struct ModelCache {
	/// base directory for cache
	cache_dir: PathBuf,
	/// cached model metadata
	models: HashMap<String, CachedModel>,
}

impl ModelCache {
	/// Create a new model cache at the specified directory
	pub fn new(cache_dir: impl AsRef<Path>) -> ModelResult<Self> {
		let cache_dir = cache_dir.as_ref().to_path_buf();
		fs::create_dir_all(&cache_dir)?;

		let mut cache = Self {
			cache_dir,
			models: HashMap::new(),
		};
		cache.load_metadata()?;
		Ok(cache)
	}

	/// Get the cache directory path
	pub fn cache_dir(&self) -> &Path {
		&self.cache_dir
	}

	/// Check if a model is cached
	pub fn is_cached(&self, model_id: &str) -> bool {
		if let Some(cached) = self.models.get(model_id) {
			// verify files still exist
			cached.weights_path.exists()
				&& cached.tokenizer_path.exists()
				&& cached.config_path.exists()
		} else {
			false
		}
	}

	/// Get cached model info
	pub fn get(&self, model_id: &str) -> Option<&CachedModel> {
		self.models.get(model_id)
	}

	/// Add a model to the cache
	pub fn add(&mut self, model: CachedModel) -> ModelResult<()> {
		self.models.insert(model.model_id.clone(), model);
		self.save_metadata()
	}

	/// Remove a model from the cache
	pub fn remove(&mut self, model_id: &str) -> ModelResult<()> {
		if let Some(cached) = self.models.remove(model_id) {
			// delete files
			let _ = fs::remove_file(&cached.weights_path);
			let _ = fs::remove_file(&cached.tokenizer_path);
			let _ = fs::remove_file(&cached.config_path);
		}
		self.save_metadata()
	}

	/// Get total cache size in bytes
	pub fn total_size(&self) -> u64 {
		self.models.values().map(|m| m.size_bytes).sum()
	}

	/// List all cached models
	pub fn list(&self) -> Vec<&CachedModel> {
		self.models.values().collect()
	}

	/// Load metadata from disk
	fn load_metadata(&mut self) -> ModelResult<()> {
		let metadata_path = self.cache_dir.join("cache.json");
		if metadata_path.exists() {
			let content = fs::read_to_string(&metadata_path)?;
			self.models = serde_json::from_str(&content)
				.map_err(|e| ModelError::Hub(format!("cache metadata: {}", e)))?;
		}
		Ok(())
	}

	/// Save metadata to disk
	fn save_metadata(&self) -> ModelResult<()> {
		let metadata_path = self.cache_dir.join("cache.json");
		let content = serde_json::to_string_pretty(&self.models)
			.map_err(|e| ModelError::Hub(format!("serialize cache: {}", e)))?;
		fs::write(&metadata_path, content)?;
		Ok(())
	}
}

impl Default for ModelCache {
	fn default() -> Self {
		let cache_dir = directories::ProjectDirs::from("com", "ch-cli", "ch-cli")
			.map(|d| d.cache_dir().join("models"))
			.unwrap_or_else(|| PathBuf::from(".ch-cli/models"));

		Self::new(cache_dir).unwrap_or_else(|_| Self {
			cache_dir: PathBuf::from(".ch-cli/models"),
			models: HashMap::new(),
		})
	}
}

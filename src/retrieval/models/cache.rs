//! Model Weight Cache - Core
//!
//! Provides caching for downloaded model weights to avoid
//! re-downloading. Stores metadata in cache.json.

use std::collections::HashMap;
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
	pub(crate) cache_dir: PathBuf,
	/// cached model metadata
	pub(crate) models: HashMap<String, CachedModel>,
}

impl ModelCache {
	/// Create a new model cache at the specified directory
	pub fn new(
		cache_dir: impl AsRef<Path>,
	) -> ModelResult<Self> {
		let cache_dir = cache_dir.as_ref().to_path_buf();
		std::fs::create_dir_all(&cache_dir)?;

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

	/// Check if a model is cached (files must exist)
	pub fn is_cached(&self, model_id: &str) -> bool {
		if let Some(cached) = self.models.get(model_id) {
			cached.weights_path.exists()
				&& cached.tokenizer_path.exists()
				&& cached.config_path.exists()
		} else {
			false
		}
	}

	/// Get cached model info
	pub fn get(
		&self,
		model_id: &str,
	) -> Option<&CachedModel> {
		self.models.get(model_id)
	}

	/// Add a model to the cache
	pub fn add(
		&mut self,
		model: CachedModel,
	) -> ModelResult<()> {
		self.models
			.insert(model.model_id.clone(), model);
		self.save_metadata()
	}
}

/// Internal cache operations.
impl ModelCache {
	/// Load metadata from disk
	fn load_metadata(&mut self) -> ModelResult<()> {
		let path = self.cache_dir.join("cache.json");
		if path.exists() {
			let content =
				std::fs::read_to_string(&path)?;
			self.models = serde_json::from_str(&content)
				.map_err(|err| {
					ModelError::Hub(format!(
						"cache metadata: {}",
						err
					))
				})?;
		}
		Ok(())
	}

	/// Save metadata to disk
	pub(crate) fn save_metadata(
		&self,
	) -> ModelResult<()> {
		let path = self.cache_dir.join("cache.json");
		let content =
			serde_json::to_string_pretty(&self.models)
				.map_err(|err| {
					ModelError::Hub(format!(
						"serialize cache: {}",
						err
					))
				})?;
		std::fs::write(&path, content)?;
		Ok(())
	}

	/// Get mutable reference to models map
	pub(crate) fn models_mut(
		&mut self,
	) -> &mut HashMap<String, CachedModel> {
		&mut self.models
	}

	/// Get reference to models map
	pub(crate) fn models(
		&self,
	) -> &HashMap<String, CachedModel> {
		&self.models
	}
}


//! Model Cache Operations
//!
//! Provides remove, list, and size management operations
//! for the model weight cache.

use super::cache::{CachedModel, ModelCache};
use super::ModelResult;

impl ModelCache {
	/// Remove a model from the cache and delete files
	pub fn remove(
		&mut self,
		model_id: &str,
	) -> ModelResult<()> {
		if let Some(cached) =
			self.models_mut().remove(model_id)
		{
			let _ =
				std::fs::remove_file(&cached.weights_path);
			let _ = std::fs::remove_file(
				&cached.tokenizer_path,
			);
			let _ =
				std::fs::remove_file(&cached.config_path);
		}
		self.save_metadata()
	}

	/// Get total cache size in bytes
	pub fn total_size(&self) -> u64 {
		self.models()
			.values()
			.map(|model| model.size_bytes)
			.sum()
	}

	/// List all cached models
	pub fn list(&self) -> Vec<&CachedModel> {
		self.models().values().collect()
	}
}


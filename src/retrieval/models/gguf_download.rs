//! GGUF Model Download Helper
//!
//! Downloads a single GGUF quantized model file and its tokenizer
//! from HuggingFace Hub. Unlike safetensors downloads, GGUF files
//! embed their config in file headers (no config.json needed).

use std::path::PathBuf;

use hf_hub::api::sync::Api;

use super::{ModelError, ModelResult};

/// Information about a downloaded GGUF model
#[derive(Debug, Clone)]
pub struct GgufModelInfo {
	/// model identifier (e.g. "microsoft/Phi-3-mini-4k-instruct-gguf")
	pub model_id: String,
	/// path to the GGUF weights file
	pub gguf_path: PathBuf,
	/// path to the tokenizer file
	pub tokenizer_path: PathBuf,
}

/// Download GGUF model + tokenizer from HuggingFace
pub fn download_gguf_model(
	model_id: &str,
	gguf_filename: &str,
) -> ModelResult<GgufModelInfo> {
	let api = Api::new()
		.map_err(|err| ModelError::Hub(err.to_string()))?;
	let repo = api.model(model_id.to_string());

	// download tokenizer from base repo (strip -gguf/-GGUF)
	let base_id = model_id
		.trim_end_matches("-gguf")
		.trim_end_matches("-GGUF");
	let base_repo = api.model(base_id.to_string());
	let tokenizer_path = base_repo
		.get("tokenizer.json")
		.map_err(|err| {
			let msg = format!("tokenizer: {}", err);
			ModelError::FileNotFound(msg)
		})?;

	// download the GGUF file
	let gguf_path = repo
		.get(gguf_filename)
		.map_err(|err| {
			let msg =
				format!("gguf {}: {}", gguf_filename, err);
			ModelError::FileNotFound(msg)
		})?;

	Ok(GgufModelInfo {
		model_id: model_id.to_string(),
		gguf_path,
		tokenizer_path,
	})
}

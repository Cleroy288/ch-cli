//! Phi-3 LLM for Query Expansion
//!
//! Loads and runs the Phi-3-mini model for text generation.
//! Used to interpret natural language queries into structured search specs.

use std::path::Path;

use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::generation::LogitsProcessor;
use candle_transformers::models::phi3::{Config as Phi3Config, Model as Phi3Model_};
use tokenizers::Tokenizer;

use crate::retrieval::models::{download_model, get_device, load_tokenizer};
use crate::retrieval::models::{ModelError, ModelResult};

/// Default Phi-3 model for query expansion
pub const DEFAULT_LLM: &str = "microsoft/Phi-3-mini-4k-instruct";

/// Maximum tokens to generate
pub const MAX_NEW_TOKENS: usize = 256;

/// Phi-3 Model for text generation
pub struct Phi3Model {
	/// the transformer model
	model: Phi3Model_,
	/// tokenizer for input/output processing
	tokenizer: Tokenizer,
	/// device (CPU/GPU)
	device: Device,
	/// end of sequence token ID
	eos_token_id: u32,
}

impl Phi3Model {
	/// Load the default Phi-3 model
	pub fn new() -> ModelResult<Self> {
		Self::from_model_id(DEFAULT_LLM)
	}

	/// Load a specific model by ID
	pub fn from_model_id(model_id: &str) -> ModelResult<Self> {
		let device = get_device();
		let model_info = download_model(model_id)?;

		Self::from_paths(
			&model_info.weights_paths,
			&model_info.tokenizer_path,
			&model_info.config_path,
			device,
		)
	}

	/// Load from local paths
	/// Supports sharded models with multiple weight files
	pub fn from_paths(
		weights_paths: &[std::path::PathBuf],
		tokenizer_path: &Path,
		config_path: &Path,
		device: Device,
	) -> ModelResult<Self> {
		let config_str = std::fs::read_to_string(config_path)?;
		let config: Phi3Config =
			serde_json::from_str(&config_str).map_err(|e| {
				ModelError::WeightLoad(
					format!("config: {}", e),
				)
			})?;

		let vb = unsafe {
			VarBuilder::from_mmaped_safetensors(
				weights_paths,
				DType::F32,
				&device,
			)?
		};

		let model = Phi3Model_::new(&config, vb)?;

		let tokenizer =
			load_tokenizer(&tokenizer_path.to_path_buf())?;

		let eos_token_id = tokenizer
			.token_to_id("<|end|>")
			.or_else(|| tokenizer.token_to_id("</s>"))
			.or_else(|| tokenizer.token_to_id("<|endoftext|>"))
			.unwrap_or(2);

		Ok(Self {
			model,
			tokenizer,
			device,
			eos_token_id,
		})
	}

	/// Generate text given a prompt
	pub fn generate(
		&mut self,
		prompt: &str,
		max_tokens: usize,
	) -> ModelResult<String> {
		let tokens = self
			.tokenizer
			.encode(prompt, true)
			.map_err(|e| {
				ModelError::Tokenizer(e.to_string())
			})?;

		let mut token_ids: Vec<u32> = tokens.get_ids().to_vec();
		let prompt_len = token_ids.len();

		let mut logits_processor =
			LogitsProcessor::new(42, Some(0.7), Some(0.9));

		for _ in 0..max_tokens {
			let input_len = token_ids.len();
			let input =
				Tensor::new(&token_ids[..], &self.device)?
					.unsqueeze(0)?;

			let logits =
				self.model.forward(&input, input_len - 1)?;

			let logits = logits.squeeze(0)?;
			let next_token = logits_processor.sample(&logits)?;

			if next_token == self.eos_token_id {
				break;
			}

			token_ids.push(next_token);
		}

		let generated_ids = &token_ids[prompt_len..];
		let output = self
			.tokenizer
			.decode(generated_ids, true)
			.map_err(|e| {
				ModelError::Tokenizer(e.to_string())
			})?;

		Ok(output)
	}

	/// Generate with the query expansion prompt
	pub fn expand_query(
		&mut self,
		query: &str,
	) -> ModelResult<String> {
		let prompt = super::QUERY_EXPANSION_PROMPT
			.replace("{query}", query);
		self.generate(&prompt, MAX_NEW_TOKENS)
	}
}


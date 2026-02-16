//! Phi-3 LLM for Query Expansion (GGUF Quantized)
//!
//! Loads and runs the Phi-3-mini model in GGUF Q4
//! format for text generation. Uses ~2.2 GB RAM
//! instead of ~7.6 GB (F32).

use std::io::BufReader;

use candle_core::quantized::gguf_file;
use candle_core::{Device, Tensor};
use candle_transformers::generation::LogitsProcessor;
use candle_transformers::models::quantized_phi3::ModelWeights;
use tokenizers::Tokenizer;

use crate::retrieval::models::{
	download_gguf_model, get_device, load_tokenizer,
};
use crate::retrieval::models::{ModelError, ModelResult};

/// Default GGUF repo for query expansion
pub const DEFAULT_LLM: &str =
	"microsoft/Phi-3-mini-4k-instruct-gguf";

/// Default GGUF filename inside the repo
pub const DEFAULT_GGUF_FILE: &str =
	"Phi-3-mini-4k-instruct-q4.gguf";

/// Maximum tokens to generate
pub const MAX_NEW_TOKENS: usize = 256;

/// First token + output buffer from initial pass
type FirstPassResult = (u32, Vec<u32>);

/// Mutable state for autoregressive token loop
struct TokenGenState<'gen> {
	/// current token to feed next
	next: &'gen mut u32,
	/// accumulated output token IDs
	out: &'gen mut Vec<u32>,
	/// logits sampling processor
	logits_proc: &'gen mut LogitsProcessor,
}

/// Phi-3 Model for text generation (GGUF quantized)
pub struct Phi3Model {
	/// quantized transformer weights with KV cache
	model: ModelWeights,
	/// tokenizer for input/output processing
	tokenizer: Tokenizer,
	/// device (CPU/GPU)
	device: Device,
	/// end of sequence token ID
	eos_token_id: u32,
}

impl Phi3Model {
	/// Load the default Phi-3 GGUF model
	pub fn new() -> ModelResult<Self> {
		Self::from_model_id(DEFAULT_LLM)
	}

	/// Load a specific GGUF model by repo ID
	pub fn from_model_id(
		model_id: &str,
	) -> ModelResult<Self> {
		let device = get_device();
		let info = download_gguf_model(
			model_id, DEFAULT_GGUF_FILE,
		)?;
		Self::from_gguf_path(
			&info.gguf_path,
			&info.tokenizer_path,
			device,
		)
	}

	/// Load from local GGUF + tokenizer paths
	pub fn from_gguf_path(
		gguf_path: &std::path::Path,
		tokenizer_path: &std::path::Path,
		device: Device,
	) -> ModelResult<Self> {
		let file = std::fs::File::open(gguf_path)?;
		let mut reader = BufReader::new(file);

		let content =
			gguf_file::Content::read(&mut reader)
				.map_err(|err| {
					let msg =
						format!("gguf read: {}", err);
					ModelError::WeightLoad(msg)
				})?;

		let model = ModelWeights::from_gguf(
			false, content, &mut reader, &device,
		)?;

		let tokenizer =
			load_tokenizer(&tokenizer_path.to_path_buf())?;
		let eos_token_id =
			resolve_eos_token(&tokenizer);

		Ok(Self {
			model, tokenizer, device, eos_token_id,
		})
	}
}

/// Text generation methods.
impl Phi3Model {
	/// Generate text given a prompt
	pub fn generate(
		&mut self,
		prompt: &str,
		max_tokens: usize,
	) -> ModelResult<String> {
		let prompt_ids = self.encode_prompt(prompt)?;
		let prompt_len = prompt_ids.len();
		let mut logits_proc =
			LogitsProcessor::new(42, Some(0.7), Some(0.9));

		let (mut next, mut out) = self
			.first_pass(&prompt_ids, &mut logits_proc)?;

		if next == self.eos_token_id {
			return Ok(String::new());
		}
		out.push(next);

		let mut state = TokenGenState {
			next: &mut next,
			out: &mut out,
			logits_proc: &mut logits_proc,
		};
		self.token_loop(
			max_tokens, prompt_len, &mut state,
		)?;

		self.decode_output(&out)
	}

	/// Encode a prompt string into token IDs
	fn encode_prompt(
		&self,
		prompt: &str,
	) -> ModelResult<Vec<u32>> {
		let tokens = self
			.tokenizer
			.encode(prompt, true)
			.map_err(|err| {
				ModelError::Tokenizer(err.to_string())
			})?;
		Ok(tokens.get_ids().to_vec())
	}

	/// Decode output token IDs into a string
	fn decode_output(
		&self,
		out: &[u32],
	) -> ModelResult<String> {
		self.tokenizer
			.decode(out, true)
			.map_err(|err| {
				ModelError::Tokenizer(err.to_string())
			})
	}

	/// Feed entire prompt and get first token
	fn first_pass(
		&mut self,
		prompt_ids: &[u32],
		logits_proc: &mut LogitsProcessor,
	) -> ModelResult<FirstPassResult> {
		let input =
			Tensor::new(prompt_ids, &self.device)?
				.unsqueeze(0)?;
		let logits =
			self.model.forward(&input, 0)?.squeeze(0)?;
		let next = logits_proc.sample(&logits)?;
		Ok((next, Vec::new()))
	}
}

/// Token loop and query expansion.
impl Phi3Model {
	/// Autoregressive token generation loop
	fn token_loop(
		&mut self,
		max_tokens: usize,
		prompt_len: usize,
		state: &mut TokenGenState<'_>,
	) -> ModelResult<()> {
		for _ in 1..max_tokens {
			let pos =
				prompt_len + state.out.len() - 1;
			let input =
				Tensor::new(
					&[*state.next], &self.device,
				)?
				.unsqueeze(0)?;
			let logits = self
				.model.forward(&input, pos)?
				.squeeze(0)?;
			*state.next =
				state.logits_proc.sample(&logits)?;
			if *state.next == self.eos_token_id {
				break;
			}
			state.out.push(*state.next);
		}
		Ok(())
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

/// Resolve EOS token from tokenizer vocabulary
fn resolve_eos_token(tokenizer: &Tokenizer) -> u32 {
	tokenizer
		.token_to_id("<|end|>")
		.or_else(|| tokenizer.token_to_id("</s>"))
		.or_else(|| {
			tokenizer.token_to_id("<|endoftext|>")
		})
		.unwrap_or(2)
}

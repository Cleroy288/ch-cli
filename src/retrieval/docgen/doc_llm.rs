//! Documentation LLM — Qwen2.5-0.5B (GGUF Q8)
//!
//! Lightweight model (~500 MB) optimized for fast
//! doc generation. ~6x faster than Phi-3 3.8B.

use std::io::BufReader;

use candle_core::quantized::gguf_file;
use candle_core::{Device, Tensor};
use candle_transformers::generation::LogitsProcessor;
use candle_transformers::models::quantized_qwen2;
use tokenizers::Tokenizer;

use crate::retrieval::models::{
	download_gguf_model, get_device, load_tokenizer,
};
use crate::retrieval::models::{ModelError, ModelResult};

/// GGUF repo for doc generation model
const DEFAULT_DOC_LLM: &str =
	"Qwen/Qwen2.5-0.5B-Instruct-GGUF";

/// GGUF filename inside the repo
const DEFAULT_DOC_GGUF: &str =
	"qwen2.5-0.5b-instruct-q8_0.gguf";

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

/// Qwen2.5-0.5B model for documentation generation
pub struct DocLlm {
	/// quantized transformer weights
	model: quantized_qwen2::ModelWeights,
	/// tokenizer for input/output processing
	tokenizer: Tokenizer,
	/// device (CPU/GPU)
	device: Device,
	/// end-of-sequence token ID
	eos_token_id: u32,
}

impl DocLlm {
	/// Load the default Qwen2.5-0.5B model
	pub fn new() -> ModelResult<Self> {
		let device = get_device();
		let info = download_gguf_model(
			DEFAULT_DOC_LLM, DEFAULT_DOC_GGUF,
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
					let msg = format!("gguf: {}", err);
					ModelError::WeightLoad(msg)
				})?;

		let model = quantized_qwen2::ModelWeights::from_gguf(
			content, &mut reader, &device,
		)?;

		let tokenizer =
			load_tokenizer(&tokenizer_path.to_path_buf())?;
		let eos_token_id =
			resolve_qwen_eos(&tokenizer);

		Ok(Self {
			model, tokenizer, device, eos_token_id,
		})
	}

	/// Generate text given a prompt
	pub fn generate(
		&mut self,
		prompt: &str,
		max_tokens: usize,
	) -> ModelResult<String> {
		let encoded = self.tokenizer
			.encode(prompt, true)
			.map_err(|err| {
				ModelError::Tokenizer(err.to_string())
			})?;
		let ids: Vec<u32> = encoded.get_ids().to_vec();
		let prompt_len = ids.len();
		let mut logits_proc =
			LogitsProcessor::new(42, Some(0.7), Some(0.9));

		let (mut next, mut out) =
			self.first_pass(&ids, &mut logits_proc)?;

		if next == self.eos_token_id {
			return Ok(String::new());
		}
		out.push(next);

		let mut state = TokenGenState {
			next: &mut next,
			out: &mut out,
			logits_proc: &mut logits_proc,
		};
		self.token_loop(max_tokens, prompt_len, &mut state)?;

		self.tokenizer.decode(&out, true).map_err(
			|err| ModelError::Tokenizer(err.to_string()),
		)
	}

	/// Feed the entire prompt and get first token
	fn first_pass(
		&mut self,
		ids: &[u32],
		logits_proc: &mut LogitsProcessor,
	) -> ModelResult<FirstPassResult> {
		let input =
			Tensor::new(ids, &self.device)?
				.unsqueeze(0)?;
		let logits =
			self.model.forward(&input, 0)?.squeeze(0)?;
		let next = logits_proc.sample(&logits)?;
		Ok((next, Vec::new()))
	}

	/// Autoregressive token generation loop
	fn token_loop(
		&mut self,
		max_tokens: usize,
		prompt_len: usize,
		state: &mut TokenGenState<'_>,
	) -> ModelResult<()> {
		for _ in 1..max_tokens {
			let pos = prompt_len + state.out.len() - 1;
			let inp =
				Tensor::new(&[*state.next], &self.device)?
					.unsqueeze(0)?;
			let logits = self
				.model.forward(&inp, pos)?
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
}

/// Resolve EOS token for Qwen2.5 models
fn resolve_qwen_eos(tokenizer: &Tokenizer) -> u32 {
	tokenizer
		.token_to_id("<|im_end|>")
		.or_else(|| {
			tokenizer.token_to_id("<|endoftext|>")
		})
		.or_else(|| tokenizer.token_to_id("</s>"))
		.unwrap_or(151645)
}

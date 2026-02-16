//! JinaBERT v2 Code Model (QK Post-Norm variant)
//!
//! Top-level model, embeddings, and encoder. The layer
//! internals (attention, MLP) are in jina_code_layers.rs.

use candle_core::{DType, Device, Result, Tensor};
use candle_nn::{
	layer_norm, Embedding, LayerNorm, Module, VarBuilder,
};

use super::jina_code_layers::{
	JinaAttnOutput, JinaGegluMlp, JinaSelfAttention,
};

/// Model configuration parsed from config.json
pub struct JinaCodeConfig {
	/// number of transformer layers
	pub num_layers: usize,
	/// number of attention heads
	pub num_heads: usize,
	/// hidden dimension
	pub hidden: usize,
	/// MLP intermediate dimension
	pub intermediate: usize,
	/// vocabulary size
	pub vocab_size: usize,
}

/// JinaBERT v2 Code model (top-level)
pub struct JinaCodeBert {
	/// token + type embeddings with LayerNorm
	embeddings: JinaEmbeddings,
	/// transformer encoder layers
	encoder: JinaEncoder,
	/// compute device
	pub device: Device,
}

impl JinaCodeBert {
	/// Load model from VarBuilder and config
	pub fn new(
		var_builder: VarBuilder,
		config: &JinaCodeConfig,
	) -> Result<Self> {
		let embeddings = JinaEmbeddings::new(
			var_builder.pp("embeddings"),
			config.vocab_size,
			config.hidden,
		)?;
		let encoder = JinaEncoder::new(
			var_builder.pp("encoder"), config,
		)?;
		Ok(Self {
			embeddings,
			encoder,
			device: var_builder.device().clone(),
		})
	}
}

impl Module for JinaCodeBert {
	fn forward(
		&self,
		input_ids: &Tensor,
	) -> Result<Tensor> {
		let emb = self.embeddings.forward(input_ids)?;
		self.encoder.forward(&emb)
	}
}

/// Token embeddings + LayerNorm
struct JinaEmbeddings {
	/// word embedding table
	word: Embedding,
	/// token type embedding table
	token_type: Embedding,
	/// layer normalization
	norm: LayerNorm,
}

impl JinaEmbeddings {
	fn new(
		var_builder: VarBuilder,
		vocab_size: usize,
		hidden: usize,
	) -> Result<Self> {
		let word = Embedding::new(
			var_builder.get(
				(vocab_size, hidden),
				"word_embeddings.weight",
			)?,
			hidden,
		);
		let token_type = Embedding::new(
			var_builder.get(
				(2, hidden),
				"token_type_embeddings.weight",
			)?,
			hidden,
		);
		let norm = layer_norm(
			hidden, 1e-12,
			var_builder.pp("LayerNorm"),
		)?;
		Ok(Self { word, token_type, norm })
	}

	fn forward(&self, ids: &Tensor) -> Result<Tensor> {
		let (batch, seq) = ids.dims2()?;
		let word_emb = self.word.forward(ids)?;
		let zeros = Tensor::zeros(
			seq, DType::U32, ids.device(),
		)?
		.broadcast_left(batch)?;
		let type_emb =
			self.token_type.forward(&zeros)?;
		self.norm.forward(&(&word_emb + type_emb)?)
	}
}

/// Transformer encoder with dynamic ALiBi bias
struct JinaEncoder {
	/// ALiBi slopes per head [1, H, 1, 1]
	alibi_slopes: Tensor,
	/// encoder layers
	layers: Vec<JinaLayer>,
}

impl JinaEncoder {
	fn new(
		var_builder: VarBuilder,
		config: &JinaCodeConfig,
	) -> Result<Self> {
		let layers = (0..config.num_layers)
			.map(|idx| {
				JinaLayer::new(
					var_builder.pp(format!("layer.{idx}")),
					config.num_heads,
					config.hidden,
					config.intermediate,
				)
			})
			.collect::<Result<Vec<_>>>()?;
		let alibi_slopes =
			super::jina_code_alibi::build_alibi_slopes(
				config.num_heads,
				var_builder.device(),
			)?;
		Ok(Self { alibi_slopes, layers })
	}

	fn forward(
		&self,
		input: &Tensor,
	) -> Result<Tensor> {
		let seq_len = input.dim(1)?;
		let bias =
			super::jina_code_alibi::compute_alibi_bias(
				&self.alibi_slopes,
				seq_len,
				input.device(),
			)?;
		// match bias dtype to hidden states (F16 on GPU)
		let bias = bias.to_dtype(input.dtype())?;
		let mut hidden = input.clone();
		for layer in &self.layers {
			hidden = layer.forward(&hidden, &bias)?;
		}
		Ok(hidden)
	}
}

/// Single transformer layer with QK post-norm
struct JinaLayer {
	/// self-attention with QK normalization
	attn: JinaSelfAttention,
	/// attention output dense + LayerNorm
	attn_output: JinaAttnOutput,
	/// post-attention layer norm
	norm1: LayerNorm,
	/// GEGLU MLP
	mlp: JinaGegluMlp,
	/// post-MLP layer norm
	norm2: LayerNorm,
}

impl JinaLayer {
	fn new(
		var_builder: VarBuilder,
		num_heads: usize,
		hidden: usize,
		intermediate: usize,
	) -> Result<Self> {
		let attn = JinaSelfAttention::new(
			var_builder.pp("attention.self"),
			num_heads,
			hidden,
		)?;
		let attn_output = JinaAttnOutput::new(
			var_builder.pp("attention.output"),
			hidden,
		)?;
		let norm1 = layer_norm(
			hidden, 1e-12,
			var_builder.pp("layer_norm_1"),
		)?;
		let mlp = JinaGegluMlp::new(
			var_builder.pp("mlp"),
			hidden,
			intermediate,
		)?;
		let norm2 = layer_norm(
			hidden, 1e-12,
			var_builder.pp("layer_norm_2"),
		)?;
		Ok(Self {
			attn, attn_output, norm1, mlp, norm2,
		})
	}

	fn forward(
		&self,
		input: &Tensor,
		bias: &Tensor,
	) -> Result<Tensor> {
		let attn_out =
			self.attn.forward(input, bias)?;
		let attn_out =
			self.attn_output.forward(&attn_out, input)?;
		let normed1 =
			self.norm1.forward(&attn_out)?;
		let mlp_out = self.mlp.forward(&normed1)?;
		self.norm2.forward(&mlp_out)
	}
}

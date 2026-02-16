//! JinaBERT v2 Code — Layer Components
//!
//! Self-attention with QK post-norm, attention output,
//! and GEGLU MLP matching the jina-bert-v2-qk-post-norm
//! weight layout.

use candle_core::{Module, Result, Tensor};
use candle_nn::{
	layer_norm, LayerNorm, Linear, VarBuilder,
};

/// Self-attention with QK post-normalization
pub struct JinaSelfAttention {
	/// query projection
	query: Linear,
	/// key projection
	key: Linear,
	/// value projection
	value: Linear,
	/// LayerNorm on Q after projection
	norm_q: LayerNorm,
	/// LayerNorm on K after projection
	norm_k: LayerNorm,
	/// number of attention heads
	num_heads: usize,
	/// dimension per head
	head_dim: usize,
}

impl JinaSelfAttention {
	/// Load from VarBuilder
	pub fn new(
		var_builder: VarBuilder,
		num_heads: usize,
		hidden: usize,
	) -> Result<Self> {
		let head_dim = hidden / num_heads;
		let (query, key, value) = load_qkv_projections(
			&var_builder, hidden,
		)?;
		let norm_q = layer_norm(
			hidden, 1e-12,
			var_builder.pp("layer_norm_q"),
		)?;
		let norm_k = layer_norm(
			hidden, 1e-12,
			var_builder.pp("layer_norm_k"),
		)?;
		Ok(Self {
			query,
			key,
			value,
			norm_q,
			norm_k,
			num_heads,
			head_dim,
		})
	}

	/// Forward pass with ALiBi bias
	pub fn forward(
		&self,
		input: &Tensor,
		bias: &Tensor,
	) -> Result<Tensor> {
		let qry = self.norm_q.forward(
			&self.query.forward(input)?,
		)?;
		let keys = self.norm_k.forward(
			&self.key.forward(input)?,
		)?;
		let vals = self.value.forward(input)?;

		let qry = reshape_heads(
			&qry, self.num_heads, self.head_dim,
		)?;
		let keys = reshape_heads(
			&keys, self.num_heads, self.head_dim,
		)?;
		let vals = reshape_heads(
			&vals, self.num_heads, self.head_dim,
		)?;

		let qkv = QkvTensors {
			qry: &qry,
			keys: &keys,
			vals: &vals,
		};
		compute_attention(
			&qkv, bias, self.head_dim,
		)
	}
}

/// Bundled QKV tensors for attention computation
struct QkvTensors<'attn> {
	/// query tensor
	qry: &'attn Tensor,
	/// key tensor
	keys: &'attn Tensor,
	/// value tensor
	vals: &'attn Tensor,
}

/// Compute scaled dot-product attention with bias
fn compute_attention<'attn>(
	qkv: &QkvTensors<'attn>,
	bias: &Tensor,
	head_dim: usize,
) -> Result<Tensor> {
	let scale = (head_dim as f64).sqrt();
	let scores =
		(qkv.qry.matmul(&qkv.keys.t()?)? / scale)?
			.broadcast_add(bias)?;
	let probs =
		candle_nn::ops::softmax_last_dim(&scores)?;
	let ctx = probs.matmul(qkv.vals)?;
	ctx.transpose(1, 2)?
		.contiguous()?
		.flatten_from(candle_core::D::Minus2)
}

/// Attention output: dense + residual + LayerNorm
pub struct JinaAttnOutput {
	/// dense projection
	dense: Linear,
	/// layer normalization
	norm: LayerNorm,
}

impl JinaAttnOutput {
	/// Load from VarBuilder
	pub fn new(
		var_builder: VarBuilder,
		hidden: usize,
	) -> Result<Self> {
		let dense = load_linear(
			hidden, hidden, var_builder.pp("dense"),
		)?;
		let norm = layer_norm(
			hidden, 1e-12,
			var_builder.pp("LayerNorm"),
		)?;
		Ok(Self { dense, norm })
	}

	/// Forward: project + residual + normalize
	pub fn forward(
		&self,
		input: &Tensor,
		residual: &Tensor,
	) -> Result<Tensor> {
		let out = self.dense.forward(input)?;
		self.norm.forward(&(out + residual)?)
	}
}

/// GEGLU MLP: up_gated_layer -> GELU gate -> down_layer
pub struct JinaGegluMlp {
	/// up-projection w/ gating (hidden -> 2*intermediate)
	up_gated: Linear,
	/// down-projection (intermediate -> hidden)
	down: Linear,
	/// intermediate dimension (half of up_gated output)
	intermediate: usize,
}

impl JinaGegluMlp {
	/// Load from VarBuilder
	pub fn new(
		var_builder: VarBuilder,
		hidden: usize,
		intermediate: usize,
	) -> Result<Self> {
		let up_proj = load_linear_no_bias(
			hidden,
			intermediate * 2,
			var_builder.pp("up_gated_layer"),
		)?;
		let down = load_linear(
			intermediate, hidden,
			var_builder.pp("down_layer"),
		)?;
		Ok(Self {
			up_gated: up_proj,
			down,
			intermediate,
		})
	}

	/// Forward: GEGLU activation + residual
	pub fn forward(
		&self,
		input: &Tensor,
	) -> Result<Tensor> {
		let proj = self.up_gated.forward(input)?;
		let gated =
			proj.narrow(candle_core::D::Minus1, 0, self.intermediate)?;
		let linear_part = proj.narrow(
			candle_core::D::Minus1,
			self.intermediate,
			self.intermediate,
		)?;
		let activated = (gated.gelu()? * linear_part)?;
		let out = self.down.forward(&activated)?;
		out + input // residual connection
	}
}

/// Bundled Q, K, V linear projection layers
type QkvProjections = (Linear, Linear, Linear);

/// Load Q, K, V linear projections from VarBuilder
fn load_qkv_projections(
	var_builder: &VarBuilder,
	hidden: usize,
) -> Result<QkvProjections> {
	let query = load_linear(
		hidden, hidden, var_builder.pp("query"),
	)?;
	let key = load_linear(
		hidden, hidden, var_builder.pp("key"),
	)?;
	let value = load_linear(
		hidden, hidden, var_builder.pp("value"),
	)?;
	Ok((query, key, value))
}

/// Reshape for multi-head attention
fn reshape_heads(
	tensor: &Tensor,
	num_heads: usize,
	head_dim: usize,
) -> Result<Tensor> {
	let mut dims = tensor.dims().to_vec();
	dims.pop();
	dims.push(num_heads);
	dims.push(head_dim);
	tensor.reshape(dims)?.transpose(1, 2)?.contiguous()
}

/// Load Linear layer (weight + bias)
fn load_linear(
	in_dim: usize,
	out_dim: usize,
	var_builder: VarBuilder,
) -> Result<Linear> {
	let weight =
		var_builder.get((out_dim, in_dim), "weight")?;
	let bias = var_builder.get(out_dim, "bias")?;
	Ok(Linear::new(weight, Some(bias)))
}

/// Load Linear layer (weight only, no bias)
fn load_linear_no_bias(
	in_dim: usize,
	out_dim: usize,
	var_builder: VarBuilder,
) -> Result<Linear> {
	let weight =
		var_builder.get((out_dim, in_dim), "weight")?;
	Ok(Linear::new(weight, None))
}

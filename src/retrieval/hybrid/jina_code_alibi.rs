//! ALiBi (Attention with Linear Biases)
//!
//! Computes per-head linear position biases for JinaBERT.
//! Slopes are pre-computed; the full bias matrix is built
//! dynamically at each forward pass for the actual seq_len.

use candle_core::{Device, Result, Tensor};

/// Pre-compute ALiBi slopes for all heads
///
/// Returns tensor of shape [1, num_heads, 1, 1] on the
/// given device. Use with `compute_alibi_bias()` at
/// forward time.
pub fn build_alibi_slopes(
	num_heads: usize,
	device: &Device,
) -> Result<Tensor> {
	let slopes = compute_slopes(num_heads);
	Tensor::new(&slopes[..], device)?
		.reshape((1, num_heads, 1, 1))
}

/// Build ALiBi bias matrix for a given sequence length
///
/// Multiplies pre-computed slopes [1, H, 1, 1] by distance
/// matrix [1, 1, S, S] -> result [1, H, S, S] via broadcast.
pub fn compute_alibi_bias(
	slopes: &Tensor,
	seq_len: usize,
	device: &Device,
) -> Result<Tensor> {
	let distances = build_distances(seq_len);
	let dist_tensor = Tensor::from_vec(
		distances,
		(1, 1, seq_len, seq_len),
		device,
	)?;
	slopes.broadcast_mul(&dist_tensor)
}

/// Compute ALiBi slopes per head (non-power-of-2 safe)
fn compute_slopes(num_heads: usize) -> Vec<f32> {
	let log2_n = (num_heads as f64).log2();
	let is_pow2 =
		(log2_n - log2_n.floor()).abs() < 1e-9;
	if is_pow2 {
		return slopes_pow2(num_heads);
	}
	// Non-power-of-2: combine two slope sets
	let closest = 1usize << (log2_n.floor() as u32);
	let mut slopes = slopes_pow2(closest);
	let extra = slopes_pow2(closest * 2);
	let remaining = num_heads - closest;
	for i in 0..remaining {
		slopes.push(extra[i * 2]);
	}
	slopes
}

/// Slopes for power-of-2 head counts
fn slopes_pow2(n: usize) -> Vec<f32> {
	let base = 2.0_f64.powf(-8.0 / n as f64);
	(0..n)
		.map(|i| base.powi((i + 1) as i32) as f32)
		.collect()
}

/// Build distance matrix: d[i][j] = -(|i - j|)
fn build_distances(n: usize) -> Vec<f32> {
	let mut out = Vec::with_capacity(n * n);
	for i in 0..n {
		for j in 0..n {
			out.push(-((i as f32 - j as f32).abs()));
		}
	}
	out
}

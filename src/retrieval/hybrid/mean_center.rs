//! Mean centering for embedding anisotropy fix.
//!
//! Subtracts the global mean vector from all embeddings,
//! then re-normalizes to unit length. This spreads vectors
//! across the full sphere, restoring cosine discriminability.
//! Reference: arXiv:2103.15316

use super::vector_store::VectorPoint;

/// Compute mean vector across all points.
/// Returns empty vec if input is empty.
pub fn compute_mean(
	points: &[VectorPoint],
) -> Vec<f32> {
	if points.is_empty() {
		return Vec::new();
	}
	let dim = points[0].vector.len();
	let count = points.len() as f32;
	let mut mean = vec![0.0_f32; dim];

	for point in points {
		for (idx, &val) in
			point.vector.iter().enumerate()
		{
			mean[idx] += val;
		}
	}
	for component in &mut mean {
		*component /= count;
	}
	mean
}

/// Subtract mean from all vectors in-place, then
/// L2-renormalize each to unit length.
pub fn apply_centering(
	points: &mut [VectorPoint],
	mean: &[f32],
) {
	for point in points.iter_mut() {
		subtract_and_renorm(&mut point.vector, mean);
	}
}

/// Center a single query vector: subtract mean,
/// L2-renormalize. Returns a new Vec (no mutation).
pub fn center_query(
	query: &[f32],
	mean: &[f32],
) -> Vec<f32> {
	let mut centered: Vec<f32> = query
		.iter()
		.zip(mean)
		.map(|(qval, mval)| qval - mval)
		.collect();
	l2_normalize(&mut centered);
	centered
}

/// Subtract mean in-place and renormalize
fn subtract_and_renorm(
	vec: &mut [f32],
	mean: &[f32],
) {
	for (val, mval) in vec.iter_mut().zip(mean) {
		*val -= mval;
	}
	l2_normalize(vec);
}

/// L2-normalize a vector in-place.
/// If norm is near zero, leaves vector unchanged.
fn l2_normalize(vec: &mut [f32]) {
	let norm: f32 = vec
		.iter()
		.map(|val| val * val)
		.sum::<f32>()
		.sqrt();
	if norm > f32::EPSILON {
		for val in vec.iter_mut() {
			*val /= norm;
		}
	}
}

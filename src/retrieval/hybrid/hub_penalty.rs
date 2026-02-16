//! Hub function penalty — data-driven only.
//!
//! Uses ref_count from the semantic call graph to penalize
//! utility functions called by many places. No hardcoded
//! name or path lists — Local Scaling handles embedding
//! space hubness at the vector store level.

/// Threshold: above this, function is a heavy hub
const HIGH_REF_THRESHOLD: usize = 20;

/// Threshold: above this, function is a moderate hub
const MID_REF_THRESHOLD: usize = 10;

/// Penalty multiplier for heavy hubs (>20 refs)
const HIGH_REF_PENALTY: f32 = 0.5;

/// Penalty multiplier for moderate hubs (>10 refs)
const MID_REF_PENALTY: f32 = 0.7;

/// Penalty for high reference count (hub functions).
/// Functions called by many places are likely utilities.
/// Returns multiplier in [HIGH_REF_PENALTY, 1.0].
pub fn ref_count_penalty(ref_count: usize) -> f32 {
	if ref_count > HIGH_REF_THRESHOLD {
		return HIGH_REF_PENALTY;
	}
	if ref_count > MID_REF_THRESHOLD {
		return MID_REF_PENALTY;
	}
	1.0
}

//! Tiered expansion configuration

/// Configuration for tiered expansion
#[derive(Debug, Clone)]
pub struct TieredConfig {
	/// minimum confidence for fast-path (0.0 - 1.0)
	pub confidence_threshold: f32,
	/// minimum importance for validated symbols
	pub importance_threshold: f32,
	/// minimum existence ratio for fast-path
	pub existence_threshold: f32,
}

impl Default for TieredConfig {
	fn default() -> Self {
		Self {
			confidence_threshold: 0.7,
			importance_threshold: 0.5,
			existence_threshold: 0.5,
		}
	}
}

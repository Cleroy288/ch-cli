use crate::indexer::symbols::Symbol;

/// A search result with score and symbol information
#[derive(Debug, Clone)]
pub struct SearchHit {
	/// The matched symbol
	pub symbol: Symbol,
	/// Relevance score (higher is better)
	pub score: f32,
}

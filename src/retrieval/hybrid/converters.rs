//! Converter functions for hybrid search
//!
//! Contains functions to convert search hits to ranked items
//! and parse symbol kinds from strings.

use crate::indexer::{SearchHit, SymbolKind};
use crate::retrieval::hybrid::fusion::RankedItem;
use crate::retrieval::hybrid::vector_store::SearchResult as VectorSearchResult;

/// Convert keyword hits to ranked items
pub fn to_keyword_ranked(
	hits: Vec<SearchHit>,
) -> Vec<RankedItem<SearchHit>> {
	hits.into_iter()
		.enumerate()
		.map(|(i, hit)| RankedItem {
			item: hit.clone(),
			rank: i + 1,
			score: hit.score,
		})
		.collect()
}

/// Convert semantic hits to ranked items
pub fn to_semantic_ranked(
	hits: Vec<VectorSearchResult>,
) -> Vec<RankedItem<VectorSearchResult>> {
	hits.into_iter()
		.enumerate()
		.map(|(i, result)| RankedItem {
			item: result.clone(),
			rank: i + 1,
			score: result.distance,
		})
		.collect()
}

/// Parse symbol kind from string representation
pub fn parse_symbol_kind(kind: &str) -> SymbolKind {
	match kind.to_lowercase().as_str() {
		"fn" | "function" => SymbolKind::Function,
		"struct" => SymbolKind::Struct,
		"enum" => SymbolKind::Enum,
		"trait" => SymbolKind::Trait,
		"impl" => SymbolKind::Impl,
		"method" => SymbolKind::Method,
		"const" | "constant" => SymbolKind::Constant,
		"static" => SymbolKind::Static,
		"type" => SymbolKind::TypeAlias,
		"mod" | "module" => SymbolKind::Module,
		"macro" => SymbolKind::Macro,
		"field" => SymbolKind::Field,
		"variant" => SymbolKind::EnumVariant,
		_ => SymbolKind::Function,
	}
}


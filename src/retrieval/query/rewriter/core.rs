//! QueryRewriter core implementation

use crate::retrieval::RetrievalResult;

use super::decompose::decompose_query;
use super::extract::extract_symbols;
use super::mapping::map_concepts;
use super::prf::{expand_iterative, PrfFeedback};

/// A rewritten query variant
#[derive(Debug, Clone)]
pub struct RewrittenQuery {
	/// the rewritten query text
	pub text: String,
	/// type of rewrite applied
	pub rewrite_type: RewriteType,
	/// confidence in this rewrite (0.0 - 1.0)
	pub confidence: f32,
}

/// Type of query rewriting applied
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewriteType {
	/// Original query unchanged
	Original,
	/// Extracted likely symbol names
	SymbolExtraction,
	/// Query decomposed into sub-queries
	Decomposition,
	/// Conceptual terms mapped to technical terms
	ConceptMapping,
	/// Expanded via pseudo-relevance feedback
	PseudoRelevanceFeedback,
}

/// Query rewriter for improving retrieval
#[derive(Default)]
pub struct QueryRewriter;

impl QueryRewriter {
	/// Create a new query rewriter
	pub fn new() -> Self {
		Self
	}

	/// Rewrite a conceptual query into variants.
	///
	/// When `feedback` is provided, adds a PRF expansion
	/// variant using terms from top keyword results.
	pub fn rewrite(
		&self,
		query: &str,
		feedback: Option<&PrfFeedback>,
	) -> RetrievalResult<Vec<RewrittenQuery>> {
		let mut variants = Vec::new();

		variants.push(RewrittenQuery {
			text: query.to_string(),
			rewrite_type: RewriteType::Original,
			confidence: 1.0,
		});

		add_symbol_variant(query, &mut variants);
		add_concept_variant(query, &mut variants);
		add_decompose_variants(query, &mut variants);

		if let Some(prf_data) = feedback {
			add_prf_variant(query, prf_data, &mut variants);
		}

		Ok(variants)
	}
}

/// Add PRF expansion variant if available.
/// Uses iterative expansion capped at MAX_PRF_ITERATIONS.
fn add_prf_variant(
	query: &str,
	prf_data: &PrfFeedback,
	variants: &mut Vec<RewrittenQuery>,
) {
	if let Some(exp) =
		expand_iterative(query, prf_data)
	{
		variants.push(RewrittenQuery {
			text: exp.expanded_text,
			rewrite_type: RewriteType::PseudoRelevanceFeedback,
			confidence: exp.confidence,
		});
	}
}

/// Add symbol extraction variant if symbols found
fn add_symbol_variant(
	query: &str,
	variants: &mut Vec<RewrittenQuery>,
) {
	let symbols = extract_symbols(query);
	if !symbols.is_empty() {
		variants.push(RewrittenQuery {
			text: symbols.join(" "),
			rewrite_type: RewriteType::SymbolExtraction,
			confidence: 0.9,
		});
	}
}

/// Add concept mapping variant if different
fn add_concept_variant(
	query: &str,
	variants: &mut Vec<RewrittenQuery>,
) {
	let mapped = map_concepts(query);
	if mapped != query {
		variants.push(RewrittenQuery {
			text: mapped,
			rewrite_type: RewriteType::ConceptMapping,
			confidence: 0.85,
		});
	}
}

/// Add decomposed sub-query variants
fn add_decompose_variants(
	query: &str,
	variants: &mut Vec<RewrittenQuery>,
) {
	let decomposed = decompose_query(query);
	for sub_query in decomposed {
		if sub_query != query {
			variants.push(RewrittenQuery {
				text: sub_query,
				rewrite_type: RewriteType::Decomposition,
				confidence: 0.8,
			});
		}
	}
}

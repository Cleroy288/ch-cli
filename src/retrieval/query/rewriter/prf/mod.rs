//! Pseudo-Relevance Feedback (PRF) for query expansion.
//!
//! Enriches semantic queries with terms extracted from
//! top BM25 keyword results. The caller provides feedback
//! symbols; this module extracts and ranks the terms.

mod iterative;
mod prf_helpers;

use std::collections::HashMap;

use prf_helpers::{count_path_terms, count_symbol_terms};

// Re-export split_to_words for testing
#[doc(hidden)]
pub use prf_helpers::split_to_words;

pub use iterative::expand_iterative;
pub use iterative::MAX_PRF_ITERATIONS;

/// Max terms to extract from PRF feedback
const MAX_PRF_TERMS: usize = 5;

/// Min confidence for a PRF term
const MIN_PRF_CONFIDENCE: f32 = 0.3;

/// Common stop words filtered from PRF terms
const STOP_WORDS: &[&str] = &[
	"the", "a", "an", "in", "of", "to", "for",
	"and", "or", "is", "are", "was", "were",
	"with", "from", "by", "on", "at", "as",
	"it", "its", "this", "that", "not", "but",
	"src", "lib", "mod", "test", "tests",
];

/// Feedback from keyword search results
#[derive(Debug, Clone)]
pub struct PrfFeedback {
	/// Symbol names from top keyword results
	pub symbol_names: Vec<String>,
	/// File paths from top keyword results
	pub file_paths: Vec<String>,
}

/// A PRF-expanded query
#[derive(Debug, Clone)]
pub struct PrfExpansion {
	/// Original query + feedback terms
	pub expanded_text: String,
	/// The terms that were added
	pub added_terms: Vec<String>,
	/// Confidence in the expansion
	pub confidence: f32,
}

/// Expand a query using pseudo-relevance feedback.
///
/// Extracts discriminative terms from feedback symbols
/// and file paths, scores by frequency, and appends
/// the best terms to the original query.
/// Returns None if no useful terms are found.
pub fn expand_with_feedback(
	query: &str,
	feedback: &PrfFeedback,
) -> Option<PrfExpansion> {
	let scored = extract_feedback_terms(feedback);
	let filtered = filter_query_terms(scored, query);

	if filtered.is_empty() {
		return None;
	}

	let terms: Vec<String> = filtered
		.iter()
		.map(|(term, _)| term.clone())
		.collect();
	let avg_conf: f32 = filtered
		.iter()
		.map(|(_, score)| score)
		.sum::<f32>()
		/ filtered.len() as f32;
	let expanded =
		format!("{} {}", query, terms.join(" "));

	Some(PrfExpansion {
		expanded_text: expanded,
		added_terms: terms,
		confidence: avg_conf,
	})
}

/// Extract and score terms from feedback.
pub fn extract_feedback_terms(
	feedback: &PrfFeedback,
) -> Vec<(String, f32)> {
	let total = feedback.symbol_names.len()
		+ feedback.file_paths.len();
	if total == 0 {
		return Vec::new();
	}

	let mut counts: HashMap<String, usize> =
		HashMap::new();
	count_symbol_terms(feedback, &mut counts);
	count_path_terms(feedback, &mut counts);

	score_and_sort(counts, total as f32)
}

/// Score terms by frequency and sort descending
fn score_and_sort(
	counts: HashMap<String, usize>,
	denom: f32,
) -> Vec<(String, f32)> {
	let mut scored: Vec<(String, f32)> = counts
		.into_iter()
		.map(|(term, cnt)| {
			(term, cnt as f32 / denom)
		})
		.filter(|(_, score)| {
			*score >= MIN_PRF_CONFIDENCE
		})
		.collect();

	scored.sort_by(|lhs, rhs| {
		rhs.1
			.partial_cmp(&lhs.1)
			.unwrap_or(std::cmp::Ordering::Equal)
	});
	scored
}

/// Filter out terms already in the query.
fn filter_query_terms(
	terms: Vec<(String, f32)>,
	query: &str,
) -> Vec<(String, f32)> {
	let lower_query = query.to_lowercase();
	let query_words: Vec<&str> =
		lower_query.split_whitespace().collect();

	terms
		.into_iter()
		.filter(|(term, _)| {
			!query_words.contains(&term.as_str())
		})
		.filter(|(term, _)| {
			!STOP_WORDS.contains(&term.as_str())
		})
		.take(MAX_PRF_TERMS)
		.collect()
}

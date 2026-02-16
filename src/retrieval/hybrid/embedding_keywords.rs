//! Body keyword extraction for embedding enrichment.
//!
//! Extracts discriminative tokens from function bodies
//! using simplified TF-IDF scoring. High-scoring tokens
//! are unique to a function and improve embedding quality.

use std::collections::HashMap;
use std::collections::HashSet;

/// Max keywords to extract per symbol
const MAX_KEYWORDS: usize = 5;

/// Min token length to consider
const MIN_TOKEN_LEN: usize = 3;

/// Rust keywords and common tokens to skip
const RUST_STOP_WORDS: &[&str] = &[
	"fn", "let", "mut", "pub", "self", "use", "mod",
	"crate", "super", "impl", "struct", "enum", "trait",
	"where", "for", "in", "if", "else", "match", "return",
	"true", "false", "none", "some", "ok", "err",
	"string", "vec", "option", "result", "box",
	"usize", "u32", "u64", "i32", "i64", "f32", "f64",
	"bool", "str", "ref", "as", "type", "const",
];

/// Pre-computed document frequencies across all symbols.
/// Each token maps to the number of symbol bodies that
/// contain it at least once.
pub struct CorpusStats {
	/// token -> number of symbols containing it
	pub doc_freq: HashMap<String, usize>,
	/// total number of symbols in the corpus
	pub total_docs: usize,
}

/// Build corpus-wide document frequency stats.
///
/// For each body, tokenizes and collects unique tokens,
/// then counts how many bodies each token appears in.
pub fn build_corpus_stats(
	bodies: &[&str],
) -> CorpusStats {
	let mut doc_freq: HashMap<String, usize> =
		HashMap::new();

	for body in bodies {
		let tokens = tokenize(body);
		let unique: HashSet<String> =
			tokens.into_iter().collect();
		for token in unique {
			*doc_freq.entry(token).or_insert(0) += 1;
		}
	}

	CorpusStats {
		doc_freq,
		total_docs: bodies.len(),
	}
}

/// Extract the most discriminative keywords from a body.
///
/// Computes TF-IDF for each unique token in the body:
/// `tf * ln(total_docs / df)`. Returns the top
/// MAX_KEYWORDS tokens sorted by descending score.
pub fn extract_keywords(
	body: &str,
	stats: &CorpusStats,
) -> Vec<String> {
	let tokens = tokenize(body);
	let term_freq = compute_term_freq(&tokens);
	let mut scored: Vec<(String, f64)> = term_freq
		.into_iter()
		.filter_map(|(token, count)| {
			let doc_freq = stats.doc_freq.get(&token)?;
			let total = stats.total_docs.max(1) as f64;
			let idf = (total / *doc_freq as f64).ln();
			Some((token, count as f64 * idf))
		})
		.collect();

	scored.sort_by(|left, right| {
		right.1.partial_cmp(&left.1)
			.unwrap_or(std::cmp::Ordering::Equal)
	});
	scored
		.into_iter()
		.take(MAX_KEYWORDS)
		.map(|(tok, _)| tok)
		.collect()
}

/// Tokenize text into lowercase words, filtering out
/// Rust keywords, short tokens, and punctuation.
fn tokenize(text: &str) -> Vec<String> {
	let stop: HashSet<&str> =
		RUST_STOP_WORDS.iter().copied().collect();

	text.split(|chr: char| {
		!chr.is_alphanumeric() && chr != '_'
	})
	.map(|word| word.to_lowercase())
	.filter(|word| {
		word.len() >= MIN_TOKEN_LEN
			&& !stop.contains(word.as_str())
	})
	.collect()
}

/// Join extracted keywords into a single string
/// separated by spaces, ready for embedding text.
pub fn format_keywords(keywords: &[String]) -> String {
	keywords.join(" ")
}

/// Count term frequency for each unique token.
fn compute_term_freq(
	tokens: &[String],
) -> HashMap<String, usize> {
	let mut freq: HashMap<String, usize> = HashMap::new();
	for token in tokens {
		*freq.entry(token.clone()).or_insert(0) += 1;
	}
	freq
}

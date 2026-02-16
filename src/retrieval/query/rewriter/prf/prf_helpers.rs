//! PRF helper functions for term extraction.
//!
//! Splits symbol names and file paths into words
//! and counts term frequencies across feedback items.

use std::collections::HashMap;

use super::PrfFeedback;

/// Min word length to keep after splitting
const MIN_WORD_LEN: usize = 3;

/// Count terms extracted from symbol names
pub fn count_symbol_terms(
	feedback: &PrfFeedback,
	counts: &mut HashMap<String, usize>,
) {
	for sym in &feedback.symbol_names {
		for word in split_to_words(sym) {
			*counts.entry(word).or_insert(0) += 1;
		}
	}
}

/// Count terms extracted from file path segments
pub fn count_path_terms(
	feedback: &PrfFeedback,
	counts: &mut HashMap<String, usize>,
) {
	for path in &feedback.file_paths {
		let segments: Vec<&str> = path
			.split(['/', '\\'])
			.collect();
		for seg in segments {
			let stem =
				seg.split('.').next().unwrap_or(seg);
			for word in split_to_words(stem) {
				*counts.entry(word).or_insert(0) += 1;
			}
		}
	}
}

/// Split a name into lowercase words.
///
/// Handles both camelCase ("VectorStore" ->
/// ["vector", "store"]) and snake_case
/// ("vector_store" -> ["vector", "store"]).
pub fn split_to_words(name: &str) -> Vec<String> {
	let underscored = name.replace('_', " ");
	let mut words = Vec::new();
	let mut current = String::new();

	for chr in underscored.chars() {
		if chr.is_uppercase() && !current.is_empty() {
			push_word(&current, &mut words);
			current.clear();
		}
		if chr.is_alphanumeric() {
			current.push(
				chr.to_lowercase().next().unwrap_or(chr),
			);
		} else if !current.is_empty() {
			push_word(&current, &mut words);
			current.clear();
		}
	}
	push_word(&current, &mut words);
	words
}

/// Push word to list if it meets minimum length
fn push_word(word: &str, words: &mut Vec<String>) {
	if word.len() >= MIN_WORD_LEN {
		words.push(word.to_string());
	}
}

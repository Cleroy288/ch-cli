//! Triple Indexing — Text Building Helpers
//!
//! Converts symbols to embedding texts with NL labels,
//! caller context, LLM doc, and TF-IDF body keywords.

use std::collections::HashMap;

use crate::indexer::symbols::Symbol;
use crate::retrieval::docgen::DocStore;
use crate::retrieval::hybrid::embedding_keywords::{
	build_corpus_stats, extract_keywords, format_keywords,
	CorpusStats,
};
use crate::retrieval::hybrid::embedding_text::{
	symbol_to_embedding_text,
};

/// Build embedding texts for all symbols
pub(crate) fn build_embedding_texts(
	symbols: &[Symbol],
	doc_store: Option<&DocStore>,
	callers: &HashMap<String, String>,
) -> Vec<String> {
	let bodies: Vec<&str> = symbols
		.iter()
		.filter_map(|sym| sym.content.as_deref())
		.collect();
	let corpus = build_corpus_stats(&bodies);

	symbols
		.iter()
		.map(|sym| {
			symbol_to_text(
				sym, doc_store, &corpus, callers,
			)
		})
		.collect()
}

/// Convert symbol to text for embedding, enriched with
/// NL labels, file context, caller context, body
/// keywords, and optional LLM doc.
pub(crate) fn symbol_to_text(
	symbol: &Symbol,
	doc_store: Option<&DocStore>,
	corpus: &CorpusStats,
	callers: &HashMap<String, String>,
) -> String {
	let mut text = symbol_to_embedding_text(symbol);

	if let Some(ctx) = callers.get(&symbol.name) {
		text.push_str(". ");
		text.push_str(ctx);
	}

	append_doc_text(&mut text, symbol, doc_store);
	append_body_keywords(&mut text, symbol, corpus);

	text
}

/// Append LLM doc text if available
fn append_doc_text(
	text: &mut String,
	symbol: &Symbol,
	doc_store: Option<&DocStore>,
) {
	if let Some(store) = doc_store {
		if let Some(entry) =
			store.get_by_name(&symbol.name)
		{
			if let Some(ref doc) = entry.llm_doc {
				text.push_str(". ");
				text.push_str(doc);
			}
		}
	}
}

/// Append body keywords (TF-IDF discriminative)
fn append_body_keywords(
	text: &mut String,
	symbol: &Symbol,
	corpus: &CorpusStats,
) {
	if let Some(ref body) = symbol.content {
		let keywords = extract_keywords(body, corpus);
		if !keywords.is_empty() {
			text.push_str(". ");
			text.push_str(&format_keywords(&keywords));
		}
	}
}

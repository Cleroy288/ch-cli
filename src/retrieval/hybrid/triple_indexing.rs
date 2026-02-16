//! Triple Hybrid Search — Indexing
//!
//! Symbol indexing and embedding generation for the triple
//! hybrid search pipelines (code, doc, notes). Doc symbols
//! use keyword-only search (no GPU embedding) since keyword
//! matching is sufficient for natural language content.

use crate::indexer::symbols::{ContentType, Symbol};
use crate::indexer::triple_search::TripleIndexStats;
use crate::retrieval::docgen::DocStore;
use crate::retrieval::hybrid::triple::{
	TripleHybridSearch, TripleHybridStats,
};
use crate::retrieval::hybrid::triple_indexing_text::{
	build_embedding_texts,
};
use crate::retrieval::hybrid::vector_store::VectorPoint;
use crate::retrieval::{RetrievalError, RetrievalResult};

/// Symbol eligible for GPU embedding (non-doc only)
pub(crate) struct NonDocEntry {
	/// index in the original symbols array
	pub original_idx: usize,
	/// content type (Code or Notes, never Doc)
	pub content_type: ContentType,
}

impl TripleHybridSearch {
	/// Index symbols (without doc enrichment)
	pub fn index_symbols(
		&mut self,
		symbols: &[Symbol],
	) -> RetrievalResult<TripleHybridStats> {
		self.index_symbols_with_docs(symbols, None)
	}

	/// Index symbols with optional LLM doc enrichment.
	/// Doc symbols get keyword-only indexing (no GPU
	/// embedding).
	pub fn index_symbols_with_docs(
		&mut self,
		symbols: &[Symbol],
		doc_store: Option<&DocStore>,
	) -> RetrievalResult<TripleHybridStats> {
		self.symbols = symbols.to_vec();
		let kw_stats =
			self.index_keyword_search(symbols)?;

		let entries = collect_non_doc(symbols);
		let syms: Vec<Symbol> = entries
			.iter()
			.map(|nde| symbols[nde.original_idx].clone())
			.collect();
		let texts = build_embedding_texts(
			&syms, doc_store, &self.caller_contexts,
		);
		let embeddings =
			self.daemon_client.embed(texts)?;

		route_non_doc_embeddings(
			&mut self.vector_store,
			symbols, &entries, &embeddings,
		);
		self.try_index_enriched(symbols, doc_store)?;
		self.vector_store.build_indexes()?;

		build_stats(&kw_stats, self)
	}

	/// Index symbols into the keyword search index
	fn index_keyword_search(
		&mut self,
		symbols: &[Symbol],
	) -> RetrievalResult<TripleIndexStats> {
		self.keyword_index
			.index_symbols(symbols)
			.map_err(|err| {
				RetrievalError::Embedding(
					err.to_string(),
				)
			})
	}

	/// Try to populate enriched store with LLM docs
	#[allow(clippy::print_stderr)]
	fn try_index_enriched(
		&mut self,
		symbols: &[Symbol],
		doc_store: Option<&DocStore>,
	) -> RetrievalResult<()> {
		if let Some(store) = doc_store {
			let count =
				self.index_enriched(symbols, store)?;
			if count > 0 {
				eprintln!(
					"[pipeline] Enriched {} symbols \
					 with LLM docs",
					count
				);
			}
		}
		Ok(())
	}
}

/// Collect indices of non-doc symbols for GPU embedding.
/// Doc symbols use keyword-only search pipeline.
fn collect_non_doc(
	symbols: &[Symbol],
) -> Vec<NonDocEntry> {
	symbols
		.iter()
		.enumerate()
		.filter_map(|(idx, sym)| {
			let ctype = ContentType::from_path(
				&sym.location.file,
			);
			if ctype == ContentType::Doc {
				return None;
			}
			Some(NonDocEntry {
				original_idx: idx,
				content_type: ctype,
			})
		})
		.collect()
}

/// Route embeddings to code/notes vector stores
fn route_non_doc_embeddings(
	store: &mut super::triple_vector_store::TripleVectorStore,
	all_symbols: &[Symbol],
	entries: &[NonDocEntry],
	embeddings: &[Vec<f32>],
) {
	for (entry, embedding) in
		entries.iter().zip(embeddings.iter())
	{
		let sym = &all_symbols[entry.original_idx];
		let point = VectorPoint {
			id: entry.original_idx as u64,
			vector: embedding.clone(),
			file_path: sym.location.file.clone(),
			line: sym.location.line,
			symbol_name: sym.name.clone(),
			symbol_kind: sym.kind.to_string(),
		};
		store.insert(point, entry.content_type);
	}
}

/// Build triple hybrid stats from keyword + vector stats
fn build_stats(
	kw_stats: &TripleIndexStats,
	search: &TripleHybridSearch,
) -> RetrievalResult<TripleHybridStats> {
	let vec_stats = search.vector_store.stats();

	Ok(TripleHybridStats {
		code_keyword_count: kw_stats.code_count,
		code_vector_count: vec_stats.code_count,
		doc_keyword_count: kw_stats.doc_count,
		doc_vector_count: vec_stats.doc_count,
		notes_keyword_count: kw_stats.notes_count,
		notes_vector_count: vec_stats.notes_count,
	})
}

//! Triple Hybrid Search — Enriched Store Indexing
//!
//! Generates separate NL-focused embeddings from LLM docs
//! and inserts them into the enriched vector store for
//! conceptual query matching.

use crate::indexer::symbols::Symbol;
use crate::retrieval::docgen::DocStore;
use crate::retrieval::hybrid::triple::TripleHybridSearch;
use crate::retrieval::hybrid::vector_store::VectorPoint;
use crate::retrieval::RetrievalResult;

impl TripleHybridSearch {
	/// Populate enriched store with LLM doc embeddings
	///
	/// For each symbol with an llm_doc, embeds "name + doc"
	/// into a separate vector store for NL-focused search.
	pub(crate) fn index_enriched(
		&mut self,
		symbols: &[Symbol],
		doc_store: &DocStore,
	) -> RetrievalResult<usize> {
		let pairs = collect_enriched_pairs(
			symbols, doc_store,
		);
		if pairs.is_empty() {
			return Ok(0);
		}

		let texts: Vec<String> = pairs
			.iter()
			.map(|(_, text)| text.clone())
			.collect();
		let embeddings =
			self.daemon_client.embed(texts)?;

		self.insert_enriched_embeddings(
			symbols, &pairs, &embeddings,
		);
		Ok(pairs.len())
	}

	/// Insert enriched embeddings into the vector store
	fn insert_enriched_embeddings(
		&mut self,
		symbols: &[Symbol],
		pairs: &[(usize, String)],
		embeddings: &[Vec<f32>],
	) {
		for ((idx, _), embedding) in
			pairs.iter().zip(embeddings.iter())
		{
			let symbol = &symbols[*idx];
			let point = VectorPoint {
				id: *idx as u64,
				vector: embedding.clone(),
				file_path: symbol.location.file.clone(),
				line: symbol.location.line,
				symbol_name: symbol.name.clone(),
				symbol_kind: symbol.kind.to_string(),
			};
			self.vector_store.insert_enriched(point);
		}
	}
}

/// Collect (index, text) pairs for symbols with LLM docs
fn collect_enriched_pairs(
	symbols: &[Symbol],
	doc_store: &DocStore,
) -> Vec<(usize, String)> {
	symbols
		.iter()
		.enumerate()
		.filter_map(|(idx, sym)| {
			let entry =
				doc_store.get_by_name(&sym.name)?;
			let llm_doc = entry.llm_doc.as_ref()?;
			let text =
				format!("{} {}", sym.name, llm_doc);
			Some((idx, text))
		})
		.collect()
}

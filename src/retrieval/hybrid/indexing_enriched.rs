//! Enriched NL Indexing
//!
//! Indexes NL-focused embeddings from LLM-generated docs
//! into a separate VectorStore for conceptual query matching.

use crate::indexer::Symbol;
use crate::retrieval::daemon::DaemonClient;
use crate::retrieval::docgen::DocStore;
use crate::retrieval::hybrid::indexing::embed_in_batches;
use crate::retrieval::hybrid::vector_store::{
	VectorPoint, VectorStore,
};
use crate::retrieval::RetrievalResult;

/// Index enriched NL embeddings into a VectorStore.
/// Returns count of indexed symbols.
pub fn index_enriched(
	store: &mut VectorStore,
	daemon_client: &DaemonClient,
	symbols: &[Symbol],
	doc_store: &DocStore,
) -> RetrievalResult<usize> {
	let pairs =
		collect_enriched_pairs(symbols, doc_store);
	if pairs.is_empty() {
		return Ok(0);
	}

	let texts: Vec<String> =
		pairs
			.iter()
			.map(|(_, text)| text.clone())
			.collect();
	let embeddings =
		embed_in_batches(daemon_client, &texts)?;

	store_enriched_embeddings(
		store, symbols, &pairs, &embeddings,
	);
	store.build_index()?;
	Ok(pairs.len())
}

/// Collect (index, text) pairs for symbols with LLM docs.
/// Text format: "{name} {llm_doc}"
fn collect_enriched_pairs(
	symbols: &[Symbol],
	doc_store: &DocStore,
) -> Vec<(usize, String)> {
	symbols
		.iter()
		.enumerate()
		.filter_map(|(i, sym)| {
			let entry =
				doc_store.get_by_name(&sym.name)?;
			let llm_doc = entry.llm_doc.as_ref()?;
			let text =
				format!("{} {}", sym.name, llm_doc);
			Some((i, text))
		})
		.collect()
}

/// Store enriched embeddings as VectorPoints
fn store_enriched_embeddings(
	store: &mut VectorStore,
	symbols: &[Symbol],
	pairs: &[(usize, String)],
	embeddings: &[Vec<f32>],
) {
	for ((idx, _), emb) in
		pairs.iter().zip(embeddings.iter())
	{
		let sym = &symbols[*idx];
		let point = VectorPoint {
			id: *idx as u64,
			vector: emb.clone(),
			file_path: sym.location.file.clone(),
			line: sym.location.line,
			symbol_name: sym.name.clone(),
			symbol_kind: sym.kind.to_string(),
		};
		store.insert(point);
	}
}

//! Indexing functions for hybrid search
//!
//! Contains index_symbols and persist functions
//! for storing symbols in both keyword and semantic indices.

use crate::indexer::{SearchIndex, Symbol};
use crate::retrieval::daemon::DaemonClient;
use crate::retrieval::hybrid::vector_store::{VectorPoint, VectorStore};
use crate::retrieval::{RetrievalError, RetrievalResult};

/// Index symbols for both keyword and semantic search
pub fn index_symbols(
	keyword_index: &mut SearchIndex,
	vector_store: &mut VectorStore,
	daemon_client: &DaemonClient,
	symbols: &[Symbol],
) -> RetrievalResult<()> {
	// index in Tantivy
	keyword_index
		.index_symbols(symbols)
		.map_err(|e| RetrievalError::Embedding(e.to_string()))?;

	// generate embeddings via daemon
	let texts: Vec<String> = symbols
		.iter()
		.map(|s| {
			format!(
				"{} {} {}",
				s.kind,
				s.name,
				s.signature.as_deref().unwrap_or("")
			)
		})
		.collect();

	let embeddings = daemon_client.embed(texts)?;

	// add to vector store
	for (i, (symbol, embedding)) in symbols
		.iter()
		.zip(embeddings.iter())
		.enumerate()
	{
		let point = VectorPoint {
			id: i as u64,
			vector: embedding.clone(),
			file_path: symbol.location.file.clone(),
			line: symbol.location.line,
			symbol_name: symbol.name.clone(),
			symbol_kind: symbol.kind.to_string(),
		};
		vector_store.insert(point);
	}

	// build HNSW index
	vector_store.build_index()?;

	Ok(())
}

/// Persist vector store to disk
pub fn persist(vector_store: &VectorStore) -> RetrievalResult<()> {
	vector_store.persist()
}

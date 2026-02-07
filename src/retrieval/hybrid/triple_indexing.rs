//! Triple Hybrid Search — Indexing
//!
//! Symbol indexing and embedding generation for the triple
//! hybrid search pipelines (code, doc, notes).

use crate::indexer::symbols::{ContentType, Symbol};
use crate::retrieval::hybrid::triple::{
	TripleHybridSearch, TripleHybridStats,
};
use crate::retrieval::hybrid::vector_store::VectorPoint;
use crate::retrieval::{RetrievalError, RetrievalResult};

impl TripleHybridSearch {
	/// Index symbols into all three pipelines
	pub fn index_symbols(
		&mut self,
		symbols: &[Symbol],
	) -> RetrievalResult<TripleHybridStats> {
		// Store symbols for later lookup
		self.symbols = symbols.to_vec();

		// Index into keyword index (routes by content type)
		let keyword_stats = self
			.keyword_index
			.index_symbols(symbols)
			.map_err(|e| {
				RetrievalError::Embedding(e.to_string())
			})?;

		// Generate embeddings and index into vector store
		let texts: Vec<String> = symbols
			.iter()
			.map(|s| self.symbol_to_text(s))
			.collect();

		let embeddings =
			self.daemon_client.embed(texts)?;

		// Route embeddings to appropriate vector store
		self.route_embeddings(symbols, &embeddings);

		// Build HNSW indexes
		self.vector_store.build_indexes()?;

		let vector_stats = self.vector_store.stats();

		Ok(TripleHybridStats {
			code_keyword_count: keyword_stats.code_count,
			code_vector_count: vector_stats.code_count,
			doc_keyword_count: keyword_stats.doc_count,
			doc_vector_count: vector_stats.doc_count,
			notes_keyword_count: keyword_stats.notes_count,
			notes_vector_count: vector_stats.notes_count,
		})
	}

	/// Route embeddings to appropriate vector stores
	fn route_embeddings(
		&mut self,
		symbols: &[Symbol],
		embeddings: &[Vec<f32>],
	) {
		for (i, (symbol, embedding)) in
			symbols.iter().zip(embeddings.iter()).enumerate()
		{
			let content_type =
				ContentType::from_path(&symbol.location.file);
			let point = VectorPoint {
				id: i as u64,
				vector: embedding.clone(),
				file_path: symbol.location.file.clone(),
				line: symbol.location.line,
				symbol_name: symbol.name.clone(),
				symbol_kind: symbol.kind.to_string(),
			};
			self.vector_store.insert(point, content_type);
		}
	}

	/// Convert symbol to text for embedding
	pub(crate) fn symbol_to_text(
		&self,
		symbol: &Symbol,
	) -> String {
		let mut parts = vec![symbol.name.clone()];

		if let Some(ref sig) = symbol.signature {
			parts.push(sig.clone());
		}

		if let Some(ref doc) = symbol.doc_comment {
			parts.push(doc.clone());
		}

		if let Some(ref content) = symbol.content {
			// For doc chunks, use the content
			parts.push(content.clone());
		}

		parts.join(" ")
	}
}

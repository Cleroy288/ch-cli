//! Indexing functions for hybrid search
//!
//! Contains index_symbols and persist functions
//! for storing symbols in both keyword and semantic indices.

use crate::indexer::{SearchIndex, Symbol, SymbolKind};
use crate::retrieval::daemon::DaemonClient;
use crate::retrieval::hybrid::embedding_text::symbol_to_embedding_text;
use crate::retrieval::hybrid::vector_store::{VectorPoint, VectorStore};
use crate::retrieval::models::{get_device_info, DeviceType};
use crate::retrieval::{RetrievalError, RetrievalResult};

/// Batch size for Metal GPU (low memory pressure)
const METAL_BATCH_SIZE: usize = 8;

/// Batch size for CPU fallback
const CPU_BATCH_SIZE: usize = 16;

/// Batch size for CUDA GPU
const CUDA_BATCH_SIZE: usize = 32;

/// Select embedding batch size based on detected device
fn embed_batch_size() -> usize {
	match get_device_info().device_type {
		DeviceType::Metal => METAL_BATCH_SIZE,
		DeviceType::Cuda => CUDA_BATCH_SIZE,
		DeviceType::Cpu => CPU_BATCH_SIZE,
	}
}

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
		.map_err(|err| {
			RetrievalError::Embedding(err.to_string())
		})?;

	// Only embed code symbols (skip doc chunks)
	let code: Vec<&Symbol> = symbols
		.iter()
		.filter(|sym| {
			sym.kind != SymbolKind::DocumentChunk
		})
		.collect();

	let texts: Vec<String> = code
		.iter()
		.map(|sym| symbol_to_embedding_text(sym))
		.collect();

	// embed in batches to avoid Metal OOM
	let embeddings =
		embed_in_batches(daemon_client, &texts)?;

	store_code_embeddings(
		vector_store, &code, &embeddings,
	);
	vector_store.build_index()?;
	Ok(())
}

/// Embed texts in device-aware batches
pub(crate) fn embed_in_batches(
	client: &DaemonClient,
	texts: &[String],
) -> RetrievalResult<Vec<Vec<f32>>> {
	let batch_sz = embed_batch_size();
	let mut all =
		Vec::with_capacity(texts.len());
	for batch in texts.chunks(batch_sz) {
		let mut result =
			client.embed(batch.to_vec())?;
		all.append(&mut result);
	}
	Ok(all)
}

/// Store code embeddings as VectorPoints
fn store_code_embeddings(
	store: &mut VectorStore,
	symbols: &[&Symbol],
	embeddings: &[Vec<f32>],
) {
	for (idx, (sym, emb)) in symbols
		.iter()
		.zip(embeddings.iter())
		.enumerate()
	{
		let point = VectorPoint {
			id: idx as u64,
			vector: emb.clone(),
			file_path: sym.location.file.clone(),
			line: sym.location.line,
			symbol_name: sym.name.clone(),
			symbol_kind: sym.kind.to_string(),
		};
		store.insert(point);
	}
}

/// Persist vector store to disk
pub fn persist(vector_store: &VectorStore) -> RetrievalResult<()> {
	vector_store.persist()
}

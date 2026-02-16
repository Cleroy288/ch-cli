//! Model loading for the daemon
//!
//! Loads embedding and reranker models at startup.
//! The Phi-3 query expansion model is lazy-loaded
//! on first use to save ~2.2 GB GPU memory.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::atomic::Ordering;

use super::ModelDaemon;
use crate::retrieval::daemon::protocol::{
	deserialize_request, serialize_response,
	DaemonRequest, DaemonResponse,
};
use crate::retrieval::hybrid::embedding::BgeEmbedder;
use crate::retrieval::hybrid::jina_embedding::JinaEmbedder;
use crate::retrieval::rerank::BgeReranker;
use crate::domain::errors::ModelResult;
use crate::retrieval::{RetrievalError, RetrievalResult};

/// Accept and handle a single client during loading
#[allow(clippy::print_stderr)]
fn check_client_during_load(
	daemon: &mut ModelDaemon,
	listener: &UnixListener,
) {
	if let Ok((stream, _)) = listener.accept() {
		let _ = stream.set_nonblocking(false);
		if let Err(err) =
			handle_client_limited(daemon, stream)
		{
			eprintln!(
				"[daemon] client error during loading: {}",
				err
			);
		}
	}
}

/// Load models while accepting ping/status requests
///
/// Loads embedding and reranker eagerly. The Phi-3
/// query expansion model is lazy-loaded on first use
/// to save ~2.2 GB GPU memory at startup.
#[allow(clippy::print_stderr)]
pub fn load_models_with_listener(
	daemon: &mut ModelDaemon,
	listener: &UnixListener,
) -> RetrievalResult<()> {
	eprintln!(
		"[daemon] Using device: {} ({})",
		daemon.device_info.device_type,
		daemon.device_info.device_name
	);
	if let Some(mem) = daemon.device_info.memory_mb {
		eprintln!("[daemon] GPU memory: {} MB", mem);
	}

	check_client_during_load(daemon, listener);
	load_embedding_model(daemon);
	check_client_during_load(daemon, listener);
	load_reranker_model(daemon);

	eprintln!(
		"[daemon] Phi-3 expansion: deferred (lazy)"
	);
	eprintln!(
		"[daemon] Models loaded, ready for requests"
	);
	Ok(())
}

/// Embedder trait alias for readability
type BoxedEmbedder = Box<
	dyn crate::retrieval::hybrid::embedder_trait::Embedder,
>;

/// Create embedder from model ID (auto-detect type)
fn create_embedder(
	model_id: &str,
) -> ModelResult<BoxedEmbedder> {
	if model_id.contains("jina") {
		JinaEmbedder::from_model_id(model_id)
			.map(|emb| Box::new(emb) as BoxedEmbedder)
	} else {
		BgeEmbedder::from_model_id(model_id)
			.map(|emb| Box::new(emb) as BoxedEmbedder)
	}
}

/// Load embedding model (detects Jina vs BGE)
#[allow(clippy::print_stderr)]
fn load_embedding_model(daemon: &mut ModelDaemon) {
	let model_id = &daemon.config.embedding_model;
	eprintln!(
		"[daemon] Loading embedding model: {}",
		model_id
	);
	match create_embedder(model_id) {
		Ok(embedder) => {
			eprintln!(
				"[daemon] Embedding loaded (dim={})",
				embedder.dim()
			);
			daemon.embedder = Some(embedder);
		}
		Err(err) => {
			eprintln!(
				"[daemon] Failed to load embedding: {}",
				err
			);
			eprintln!(
				"[daemon] Degraded mode (no embeddings)"
			);
		}
	}
}

/// Load reranker model
#[allow(clippy::print_stderr)]
fn load_reranker_model(daemon: &mut ModelDaemon) {
	eprintln!(
		"[daemon] Loading reranker model: {}",
		daemon.config.reranker_model
	);
	match BgeReranker::from_model_id(
		&daemon.config.reranker_model,
	) {
		Ok(reranker) => {
			eprintln!("[daemon] Reranker model loaded");
			daemon.reranker = Some(reranker);
		}
		Err(err) => {
			eprintln!(
				"[daemon] Failed to load reranker: {}",
				err
			);
			eprintln!(
				"[daemon] Degraded mode (no reranking)"
			);
		}
	}
}

/// Handle client with limited commands during loading
///
/// Only ping, status, and shutdown are accepted.
fn handle_client_limited(
	daemon: &mut ModelDaemon,
	mut stream: UnixStream,
) -> RetrievalResult<()> {
	let mut reader = BufReader::new(stream.try_clone()?);
	let mut line = String::new();

	if reader.read_line(&mut line)? == 0 {
		return Ok(());
	}

	let request: DaemonRequest =
		deserialize_request(line.trim().as_bytes())
			.map_err(|err| {
				RetrievalError::DaemonCommunication(
					format!("deserialize: {}", err),
				)
			})?;

	let response = dispatch_limited(daemon, request);
	let bytes = serialize_response(&response).map_err(
		|err| {
			RetrievalError::DaemonCommunication(format!(
				"serialize: {}",
				err
			))
		},
	)?;
	stream.write_all(&bytes)?;
	stream.flush()?;
	Ok(())
}

/// Dispatch during model loading (limited commands)
fn dispatch_limited(
	daemon: &mut ModelDaemon,
	request: DaemonRequest,
) -> DaemonResponse {
	match request {
		DaemonRequest::Ping => DaemonResponse::Pong,
		DaemonRequest::Status => {
			super::client_handler::handle_status(daemon)
		}
		DaemonRequest::Shutdown => {
			daemon
				.shutdown
				.store(true, Ordering::SeqCst);
			DaemonResponse::Ok
		}
		_ => DaemonResponse::Error(
			"Daemon is still loading models".to_string(),
		),
	}
}

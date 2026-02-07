//! Model loading for the daemon
//!
//! Loads embedding, reranker, and query expansion
//! models while accepting ping/status requests.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::atomic::Ordering;

use super::ModelDaemon;
use crate::retrieval::daemon::protocol::{
	deserialize_request, serialize_response,
	DaemonRequest, DaemonResponse,
};
use crate::retrieval::hybrid::BgeEmbedder;
use crate::retrieval::query::QueryInterpreter;
use crate::retrieval::rerank::BgeReranker;
use crate::retrieval::{RetrievalError, RetrievalResult};

/// Load models while accepting ping/status requests
pub fn load_models_with_listener(
	daemon: &mut ModelDaemon,
	listener: &UnixListener,
) -> RetrievalResult<()> {
	// helper to handle ping/status during loading
	let check = |d: &mut ModelDaemon, l: &UnixListener| {
		if let Ok((stream, _)) = l.accept() {
			let _ = stream.set_nonblocking(false);
			if let Err(e) =
				handle_client_limited(d, stream)
			{
				eprintln!(
					"[daemon] client error during loading: {}",
					e
				);
			}
		}
	};

	// Log device info
	eprintln!(
		"[daemon] Using device: {} ({})",
		daemon.device_info.device_type,
		daemon.device_info.device_name
	);
	if let Some(mem) = daemon.device_info.memory_mb {
		eprintln!("[daemon] GPU memory: {} MB", mem);
	}

	check(daemon, listener);
	load_embedding_model(daemon);
	check(daemon, listener);
	load_reranker_model(daemon);
	check(daemon, listener);
	load_expansion_model(daemon);

	eprintln!(
		"[daemon] All models loaded, ready for requests"
	);
	Ok(())
}

/// Load embedding model
fn load_embedding_model(daemon: &mut ModelDaemon) {
	eprintln!(
		"[daemon] Loading embedding model: {}",
		daemon.config.embedding_model
	);
	match BgeEmbedder::from_model_id(
		&daemon.config.embedding_model,
	) {
		Ok(embedder) => {
			eprintln!(
				"[daemon] Embedding model loaded (dim={})",
				embedder.dim()
			);
			daemon.embedder = Some(embedder);
		}
		Err(e) => {
			eprintln!(
				"[daemon] Failed to load embedding: {}",
				e
			);
			eprintln!(
				"[daemon] Degraded mode (no embeddings)"
			);
		}
	}
}

/// Load reranker model
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
		Err(e) => {
			eprintln!(
				"[daemon] Failed to load reranker: {}",
				e
			);
			eprintln!(
				"[daemon] Degraded mode (no reranking)"
			);
		}
	}
}

/// Load query expansion model (LLM)
fn load_expansion_model(daemon: &mut ModelDaemon) {
	eprintln!(
		"[daemon] Loading expansion model: {}",
		daemon.config.expansion_model
	);
	match QueryInterpreter::with_model_id(
		&daemon.config.expansion_model,
	) {
		Ok(interp) => {
			eprintln!(
				"[daemon] Query expansion model loaded"
			);
			daemon.interpreter = interp;
		}
		Err(e) => {
			eprintln!(
				"[daemon] Failed to load expansion: {}",
				e
			);
			eprintln!(
				"[daemon] Degraded mode (no expansion)"
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
	let mut line = String::new(); // request line

	if reader.read_line(&mut line)? == 0 {
		return Ok(());
	}

	let request: DaemonRequest =
		deserialize_request(line.trim().as_bytes())
			.map_err(|e| {
				RetrievalError::DaemonCommunication(
					format!("deserialize: {}", e),
				)
			})?;

	let response = match request {
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
	};

	let bytes = serialize_response(&response).map_err(
		|e| {
			RetrievalError::DaemonCommunication(format!(
				"serialize: {}",
				e
			))
		},
	)?;
	stream.write_all(&bytes)?;
	stream.flush()?;

	Ok(())
}

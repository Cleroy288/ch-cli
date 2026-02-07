//! Daemon lifecycle management
//!
//! Handles daemon creation, startup, and the main
//! accept loop for client connections.

use std::fs;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::sync::atomic::Ordering;

use super::ModelDaemon;
use crate::retrieval::{RetrievalConfig, RetrievalResult};

/// Create a new daemon with default config
pub fn new_daemon() -> ModelDaemon {
	let config = RetrievalConfig::default();
	ModelDaemon::new_with_config(
		config.socket_path.clone(),
		config,
	)
}

/// Create a new daemon with custom socket path
pub fn with_socket_path(
	socket_path: impl AsRef<Path>,
) -> ModelDaemon {
	let config = RetrievalConfig::default();
	ModelDaemon::new_with_config(
		socket_path.as_ref().to_path_buf(),
		config,
	)
}

/// Write PID file for the daemon
pub fn write_pid_file(
	daemon: &ModelDaemon,
) -> RetrievalResult<()> {
	super::lifecycle_utils::write_pid_file(daemon)
}

/// Get list of loaded models
pub fn loaded_models(
	daemon: &ModelDaemon,
) -> Vec<String> {
	let mut models = Vec::new(); // model name list
	if daemon.embedder.is_some() {
		models.push(format!(
			"{} (embeddings)",
			daemon.config.embedding_model
		));
	}
	if daemon.reranker.is_some() {
		models.push(format!(
			"{} (reranker)",
			daemon.config.reranker_model
		));
	}
	if daemon.interpreter.has_llm() {
		models.push(format!(
			"{} (query expansion)",
			daemon.config.expansion_model
		));
	}
	models
}

/// Run the daemon main loop (blocking)
pub fn run(
	daemon: &mut ModelDaemon,
) -> RetrievalResult<()> {
	// ensure parent directory exists
	if let Some(parent) = daemon.socket_path.parent() {
		fs::create_dir_all(parent)?;
	}

	// remove stale socket
	let _ = fs::remove_file(&daemon.socket_path);

	// write PID file
	write_pid_file(daemon)?;

	// create listener BEFORE loading models
	let listener =
		UnixListener::bind(&daemon.socket_path)?;
	eprintln!(
		"[daemon] Listening on {:?}",
		daemon.socket_path
	);
	eprintln!(
		"[daemon] Loading models (socket ready)..."
	);

	// non-blocking for model loading phase
	listener.set_nonblocking(true)?;

	// load models while accepting ping/status
	super::model_loader::load_models_with_listener(
		daemon, &listener,
	)?;

	// switch to blocking mode
	listener.set_nonblocking(false)?;

	// accept connections (models loaded)
	run_main_loop(daemon, &listener)?;

	// signal background doc gen to stop
	daemon
		.doc_gen_cancel
		.store(true, Ordering::SeqCst);

	// wait for doc gen thread to finish
	if let Some(handle) = daemon.doc_gen_thread.take() {
		eprintln!(
			"[daemon] Waiting for doc gen thread..."
		);
		let _ = handle.join();
	}

	// delete partial docs for cached projects
	for project_path in daemon.project_cache.keys() {
		let docs =
			project_path.join(".ch-index/docs.json");
		let _ = fs::remove_file(&docs);
	}

	// cleanup socket and pid files
	let _ = fs::remove_file(&daemon.socket_path);
	let _ = fs::remove_file(
		daemon.socket_path.with_extension("pid"),
	);

	Ok(())
}

/// Main accept loop for client connections
fn run_main_loop(
	daemon: &mut ModelDaemon,
	listener: &UnixListener,
) -> RetrievalResult<()> {
	loop {
		// check shutdown before blocking accept
		if daemon.shutdown.load(Ordering::SeqCst) {
			eprintln!("[daemon] Shutting down...");
			break;
		}

		match listener.accept() {
			Ok((stream, _)) => {
				if let Err(e) =
					super::client_handler::handle_client(
						daemon, stream,
					)
				{
					eprintln!(
						"[daemon] client error: {}",
						e
					);
				}
			}
			Err(e) => {
				eprintln!(
					"[daemon] accept error: {}",
					e
				);
			}
		}
	}
	Ok(())
}


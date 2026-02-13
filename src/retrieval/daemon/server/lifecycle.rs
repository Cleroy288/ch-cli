//! Daemon lifecycle management
//!
//! Handles daemon creation, startup, and the main
//! accept loop for client connections.

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
///
/// Shows expansion model as "lazy, not loaded" when
/// it has not been triggered yet.
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
	let expansion_label =
		expansion_status_label(daemon);
	models.push(expansion_label);
	models
}

/// Build status label for the expansion model
fn expansion_status_label(
	daemon: &ModelDaemon,
) -> String {
	let name = &daemon.config.expansion_model;
	if daemon.interpreter.has_llm() {
		format!("{} (query expansion)", name)
	} else {
		format!("{} (query expansion, lazy)", name)
	}
}

/// Prepare socket and bind listener
#[allow(clippy::print_stderr)]
fn bind_listener(
	daemon: &mut ModelDaemon,
) -> RetrievalResult<UnixListener> {
	if let Some(parent) = daemon.socket_path.parent() {
		std::fs::create_dir_all(parent)?;
	}
	let _ = std::fs::remove_file(&daemon.socket_path);
	write_pid_file(daemon)?;
	let listener =
		UnixListener::bind(&daemon.socket_path)?;
	eprintln!(
		"[daemon] Listening on {:?}",
		daemon.socket_path
	);
	eprintln!(
		"[daemon] Loading models (socket ready)..."
	);
	Ok(listener)
}

/// Cleanup on shutdown: join threads, remove files
#[allow(clippy::print_stderr)]
fn cleanup_on_shutdown(daemon: &mut ModelDaemon) {
	daemon
		.doc_gen_cancel
		.store(true, Ordering::SeqCst);
	if let Some(handle) = daemon.doc_gen_thread.take() {
		eprintln!(
			"[daemon] Waiting for doc gen thread..."
		);
		let _ = handle.join();
	}
	for project_path in daemon.project_cache.keys() {
		let docs =
			project_path.join(".rustean-index/docs.json");
		let _ = std::fs::remove_file(&docs);
	}
	let _ = std::fs::remove_file(&daemon.socket_path);
	let _ = std::fs::remove_file(
		daemon.socket_path.with_extension("pid"),
	);
}

/// Run the daemon main loop (blocking)
pub fn run(
	daemon: &mut ModelDaemon,
) -> RetrievalResult<()> {
	let listener = bind_listener(daemon)?;
	listener.set_nonblocking(true)?;
	super::model_loader::load_models_with_listener(
		daemon, &listener,
	)?;
	listener.set_nonblocking(false)?;
	run_main_loop(daemon, &listener)?;
	cleanup_on_shutdown(daemon);
	Ok(())
}

/// Accept and handle a single client connection
#[allow(clippy::print_stderr)]
fn accept_one_client(
	daemon: &mut ModelDaemon,
	listener: &UnixListener,
) {
	match listener.accept() {
		Ok((stream, _)) => {
			if let Err(err) =
				super::client_handler::handle_client(
					daemon, stream,
				)
			{
				eprintln!(
					"[daemon] client error: {}", err
				);
			}
		}
		Err(err) => {
			eprintln!(
				"[daemon] accept error: {}", err
			);
		}
	}
}

/// Main accept loop for client connections
#[allow(clippy::print_stderr)]
fn run_main_loop(
	daemon: &mut ModelDaemon,
	listener: &UnixListener,
) -> RetrievalResult<()> {
	loop {
		if daemon.shutdown.load(Ordering::SeqCst) {
			eprintln!("[daemon] Shutting down...");
			break;
		}
		accept_one_client(daemon, listener);
	}
	Ok(())
}


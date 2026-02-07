//! Client request handling
//!
//! Handles incoming client connections and request dispatch.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::sync::atomic::Ordering;

use super::ModelDaemon;
use crate::retrieval::daemon::protocol::{
	deserialize_request, serialize_response, DaemonRequest, DaemonResponse,
	DaemonStatus, DeviceStatus,
};
use crate::retrieval::{RetrievalError, RetrievalResult};

/// Handle status request - returns daemon status info
pub fn handle_status(daemon: &ModelDaemon) -> DaemonResponse {
	DaemonResponse::Status(DaemonStatus {
		running: true,
		pid: Some(std::process::id()),
		loaded_models: super::lifecycle::loaded_models(daemon),
		device: DeviceStatus {
			device_type: daemon.device_info.device_type.to_string(),
			device_name: daemon.device_info.device_name.clone(),
			memory_mb: daemon.device_info.memory_mb,
		},
		uptime_secs: daemon.start_time.elapsed().as_secs(),
	})
}

/// Handle a single client request - dispatches to handler
pub fn handle_request(
	daemon: &mut ModelDaemon,
	request: DaemonRequest,
) -> DaemonResponse {
	match request {
		DaemonRequest::Ping => DaemonResponse::Pong,
		DaemonRequest::Status => handle_status(daemon),
		DaemonRequest::Shutdown => {
			daemon.shutdown.store(true, Ordering::SeqCst);
			DaemonResponse::Ok
		}
		DaemonRequest::Embed { texts } => handle_embed(daemon, texts),
		DaemonRequest::Rerank { query, documents } => {
			handle_rerank(daemon, &query, documents)
		}
		DaemonRequest::Expand { query } => handle_expand(daemon, &query),
		DaemonRequest::IndexProject {
			project_path, force,
		} => {
			super::project_handlers::handle_index_project(
				daemon, &project_path, force,
			)
		}
		DaemonRequest::SearchProject {
			project_path, query, limit,
		} => {
			super::project_handlers::handle_search_project(
				daemon, &project_path, &query, limit,
			)
		}
		DaemonRequest::ProjectStatus { project_path } => {
			super::project_handlers::handle_project_status(
				daemon, &project_path,
			)
		}
		DaemonRequest::EvictProject { project_path } => {
			super::project_handlers::handle_evict_project(
				daemon, &project_path,
			)
		}
		DaemonRequest::StartDocGen {
			project_path, force,
		} => {
			super::doc_handlers::handle_start_doc_gen(
				daemon, &project_path, force,
			)
		}
		DaemonRequest::DocGenStatus { project_path } => {
			super::doc_handlers::handle_doc_gen_status(
				daemon, &project_path,
			)
		}
		DaemonRequest::GetDoc {
			project_path, symbol_name,
		} => {
			super::doc_handlers::handle_get_doc(
				daemon, &project_path, &symbol_name,
			)
		}
		DaemonRequest::SearchDocs {
			project_path, query, limit,
		} => {
			super::doc_handlers::handle_search_docs(
				daemon, &project_path, &query, limit,
			)
		}
	}
}

/// Handle embed request
fn handle_embed(
	daemon: &ModelDaemon,
	texts: Vec<String>,
) -> DaemonResponse {
	super::request_handlers::handle_embed(daemon, texts)
}

/// Handle rerank request
fn handle_rerank(
	daemon: &ModelDaemon,
	query: &str,
	documents: Vec<String>,
) -> DaemonResponse {
	super::request_handlers::handle_rerank(
		daemon, query, documents,
	)
}

/// Handle expand request (query expansion via LLM)
fn handle_expand(
	daemon: &mut ModelDaemon,
	query: &str,
) -> DaemonResponse {
	super::request_handlers::handle_expand(daemon, query)
}

/// Handle a client connection - reads requests and sends responses
pub fn handle_client(
	daemon: &mut ModelDaemon,
	stream: UnixStream,
) -> RetrievalResult<()> {
	let mut reader = BufReader::new(&stream);
	let mut writer = &stream;

	loop {
		let mut line = String::new(); // incoming request line
		match reader.read_line(&mut line) {
			Ok(0) => break, // client disconnected
			Ok(_) => {
				// parse request
				let raw = line.trim().as_bytes();
				let request = match deserialize_request(raw) {
					Ok(r) => r,
					Err(e) => {
						let msg = format!("parse error: {}", e);
						let resp = DaemonResponse::Error(msg);
						let bytes = serialize_response(&resp).unwrap();
						writer.write_all(&bytes)?;
						continue;
					}
				};

				// handle request
				let response = handle_request(daemon, request);
				let bytes = serialize_response(&response)
					.map_err(|e| {
						let msg = e.to_string();
						RetrievalError::DaemonCommunication(msg)
					})?;
				writer.write_all(&bytes)?;
				writer.flush()?;

				// check for shutdown
				if daemon.shutdown.load(Ordering::SeqCst) {
					break;
				}
			}
			Err(e) => {
				eprintln!("[daemon] read error: {}", e);
				break;
			}
		}
	}

	Ok(())
}

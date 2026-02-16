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

/// Dispatch core requests (ping, status, shutdown, ML)
fn dispatch_core(
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
		DaemonRequest::Embed { texts } => {
			super::request_handlers::handle_embed(
				daemon, texts,
			)
		}
		DaemonRequest::Rerank { query, documents } => {
			super::request_handlers::handle_rerank(
				daemon, &query, documents,
			)
		}
		DaemonRequest::Expand { query } => {
			super::request_handlers::handle_expand(
				daemon, &query,
			)
		}
		other => dispatch_project_or_doc(daemon, other),
	}
}

/// Dispatch project and doc requests
fn dispatch_project_or_doc(
	daemon: &mut ModelDaemon,
	request: DaemonRequest,
) -> DaemonResponse {
	match request {
		DaemonRequest::IndexProject {
			project_path, force,
		} => super::project_handlers::handle_index_project(
			daemon, &project_path, force,
		),
		DaemonRequest::SearchProject {
			project_path, query, limit,
		} => super::project_handlers::handle_search_project(
			daemon, &project_path, &query, limit,
		),
		DaemonRequest::ProjectStatus { project_path } =>
			super::project_handlers::handle_project_status(
				daemon, &project_path,
			),
		DaemonRequest::EvictProject { project_path } =>
			super::project_handlers::handle_evict_project(
				daemon, &project_path,
			),
		other => dispatch_doc(daemon, other),
	}
}

/// Dispatch documentation requests
#[allow(clippy::too_many_lines)]
fn dispatch_doc(
	daemon: &mut ModelDaemon,
	request: DaemonRequest,
) -> DaemonResponse {
	use super::doc_handlers as docs;
	match request {
		DaemonRequest::StartDocGen {
			project_path, force,
		} => docs::handle_start_doc_gen(
			daemon, &project_path, force,
		),
		DaemonRequest::DocGenStatus {
			project_path,
		} => docs::handle_doc_gen_status(
			daemon, &project_path,
		),
		DaemonRequest::GetDoc {
			project_path, symbol_name,
		} => docs::handle_get_doc(
			daemon, &project_path, &symbol_name,
		),
		DaemonRequest::GetDocByFile {
			project_path, file_path, symbol_name,
		} => docs::handle_get_doc_by_file(
			daemon, &project_path,
			&file_path, &symbol_name,
		),
		DaemonRequest::SearchDocs {
			project_path, query, limit,
		} => docs::handle_search_docs(
			daemon, &project_path, &query, limit,
		),
		_ => DaemonResponse::Error(
			"unknown request type".into(),
		),
	}
}

/// Handle a single client request - dispatches to handler
pub fn handle_request(
	daemon: &mut ModelDaemon,
	request: DaemonRequest,
) -> DaemonResponse {
	dispatch_core(daemon, request)
}

/// Send error response for a malformed request
fn send_parse_error(
	writer: &mut &UnixStream,
	err: serde_json::Error,
) -> std::io::Result<()> {
	let msg = format!("parse error: {}", err);
	let resp = DaemonResponse::Error(msg);
	let bytes = serialize_response(&resp).unwrap();
	writer.write_all(&bytes)
}

/// Process a single request and send response
fn process_and_respond(
	daemon: &mut ModelDaemon,
	writer: &mut &UnixStream,
	request: DaemonRequest,
) -> RetrievalResult<()> {
	let response = handle_request(daemon, request);
	let bytes = serialize_response(&response)
		.map_err(|err| {
			let msg = err.to_string();
			RetrievalError::DaemonCommunication(msg)
		})?;
	writer.write_all(&bytes)?;
	writer.flush()?;
	Ok(())
}

/// Process a single line from the client stream
#[allow(clippy::print_stderr)]
fn process_line(
	daemon: &mut ModelDaemon,
	writer: &mut &UnixStream,
	line: &str,
) -> RetrievalResult<bool> {
	let raw = line.trim().as_bytes();
	match deserialize_request(raw) {
		Ok(req) => {
			process_and_respond(daemon, writer, req)?;
		}
		Err(err) => {
			send_parse_error(writer, err)?;
			return Ok(false); // continue loop
		}
	}
	let should_stop =
		daemon.shutdown.load(Ordering::SeqCst);
	Ok(should_stop)
}

/// Handle a client connection - read/respond loop
#[allow(clippy::print_stderr)]
pub fn handle_client(
	daemon: &mut ModelDaemon,
	stream: UnixStream,
) -> RetrievalResult<()> {
	let mut reader = BufReader::new(&stream);
	let mut writer = &stream;

	loop {
		let mut line = String::new();
		match reader.read_line(&mut line) {
			Ok(0) => break,
			Ok(_) => {
				if process_line(
					daemon, &mut writer, &line,
				)? {
					break;
				}
			}
			Err(err) => {
				eprintln!(
					"[daemon] read error: {}", err
				);
				break;
			}
		}
	}
	Ok(())
}

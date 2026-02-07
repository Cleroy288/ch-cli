//! Core Daemon Requests
//!
//! Provides embed, rerank, expand, status, ping, and shutdown operations.

use std::path::Path;
use std::time::Duration;

use super::connection::send_request;
use crate::retrieval::daemon::protocol::{
	DaemonRequest, DaemonResponse, DaemonStatus, SearchSpec,
};
use crate::retrieval::{RetrievalError, RetrievalResult};

/// Generate embeddings for texts
pub fn embed(
	socket_path: &Path,
	timeout: Duration,
	texts: Vec<String>,
) -> RetrievalResult<Vec<Vec<f32>>> {
	let request = DaemonRequest::Embed { texts };
	match send_request(socket_path, timeout, &request)? {
		DaemonResponse::Embeddings(vecs) => Ok(vecs),
		DaemonResponse::Error(e) => Err(RetrievalError::Embedding(e)),
		other => Err(RetrievalError::DaemonCommunication(format!(
			"unexpected response: {:?}",
			other
		))),
	}
}

/// Rerank documents given a query
pub fn rerank(
	socket_path: &Path,
	timeout: Duration,
	query: String,
	documents: Vec<String>,
) -> RetrievalResult<Vec<f32>> {
	let request = DaemonRequest::Rerank { query, documents };
	match send_request(socket_path, timeout, &request)? {
		DaemonResponse::Scores(scores) => Ok(scores),
		DaemonResponse::Error(e) => Err(RetrievalError::Embedding(e)),
		other => Err(RetrievalError::DaemonCommunication(format!(
			"unexpected response: {:?}",
			other
		))),
	}
}

/// Expand a natural language query into structured SearchSpec
pub fn expand(
	socket_path: &Path,
	timeout: Duration,
	query: String,
) -> RetrievalResult<SearchSpec> {
	let request = DaemonRequest::Expand { query };
	match send_request(socket_path, timeout, &request)? {
		DaemonResponse::SearchSpec(spec) => Ok(spec),
		DaemonResponse::Error(e) => Err(RetrievalError::Embedding(e)),
		other => Err(RetrievalError::DaemonCommunication(format!(
			"unexpected response: {:?}",
			other
		))),
	}
}

/// Get daemon status (delegated to status_requests)
pub fn status(
	socket_path: &Path,
	timeout: Duration,
) -> RetrievalResult<DaemonStatus> {
	super::status_requests::status(socket_path, timeout)
}

/// Ping the daemon to check if it's alive
pub fn ping(
	socket_path: &Path,
	timeout: Duration
) -> RetrievalResult<bool> {
	let request = DaemonRequest::Ping;
	match send_request(socket_path, timeout, &request) {
		Ok(DaemonResponse::Pong) => Ok(true),
		_ => Ok(false),
	}
}

/// Shutdown the daemon
pub fn shutdown(
	socket_path: &Path,
	timeout: Duration
) -> RetrievalResult<()> {
	let request = DaemonRequest::Shutdown;
	match send_request(socket_path, timeout, &request) {
		Ok(DaemonResponse::Ok) => Ok(()),
		Ok(DaemonResponse::Error(e)) =>
			Err(RetrievalError::DaemonCommunication(e)),
		Err(_) => Ok(()), // daemon may have already shut down
		other => Err(RetrievalError::DaemonCommunication(
			format!("unexpected response: {:?}", other)
		)),
	}
}

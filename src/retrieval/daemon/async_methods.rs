//! Async Client Methods
//!
//! Provides async implementations of daemon operations.

use std::path::Path;
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::io::BufReader;
use tokio::net::UnixStream;
use tokio::time::timeout;

use super::protocol::{
	DaemonRequest, DaemonResponse, SearchSpec,
};
use crate::retrieval::{RetrievalError, RetrievalResult};

/// Send request to daemon asynchronously
pub async fn send_async_request(
	socket_path: &Path,
	request_timeout: Duration,
	request: &DaemonRequest,
) -> RetrievalResult<DaemonResponse> {
	// Connect with timeout
	let connect = UnixStream::connect(socket_path);
	let stream = timeout(request_timeout, connect)
		.await
		.map_err(|_| {
			let msg = "connection timeout".to_string();
			RetrievalError::DaemonCommunication(msg)
		})?
		.map_err(|e| {
			RetrievalError::DaemonNotRunning(
				e.to_string(),
			)
		})?;

	// split stream into reader and writer
	let (reader, mut writer) = stream.into_split();

	// Serialize request to JSON
	let json = serde_json::to_string(request).map_err(
		|e| {
			let msg = format!("serialize: {}", e);
			RetrievalError::DaemonCommunication(msg)
		},
	)?;

	// Write request with timeout
	timeout(request_timeout, async {
		writer.write_all(json.as_bytes()).await?;
		writer.write_all(b"\n").await?;
		writer.flush().await?;
		Ok::<_, std::io::Error>(())
	})
	.await
	.map_err(|_| {
		let msg = "write timeout".to_string();
		RetrievalError::DaemonCommunication(msg)
	})?
	.map_err(|e| RetrievalError::Io(e))?;

	// Read response with timeout
	let mut reader = BufReader::new(reader);
	let mut line = String::new();

	timeout(request_timeout, reader.read_line(&mut line))
		.await
		.map_err(|_| {
			let msg = "read timeout".to_string();
			RetrievalError::DaemonCommunication(msg)
		})?
		.map_err(|e| RetrievalError::Io(e))?;

	// Deserialize response from JSON
	serde_json::from_str(line.trim()).map_err(|e| {
		let msg = format!("deserialize: {}", e);
		RetrievalError::DaemonCommunication(msg)
	})
}

/// Generate embeddings async
pub async fn async_embed(
	socket_path: &Path,
	timeout: Duration,
	texts: Vec<String>,
) -> RetrievalResult<Vec<Vec<f32>>> {
	let request = DaemonRequest::Embed { texts };
	let response =
		send_async_request(socket_path, timeout, &request)
			.await?;
	match response {
		DaemonResponse::Embeddings(v) => Ok(v),
		DaemonResponse::Error(e) => {
			Err(RetrievalError::Embedding(e))
		}
		_ => {
			let msg = "unexpected response".to_string();
			Err(RetrievalError::DaemonCommunication(msg))
		}
	}
}

/// Rerank documents async
pub async fn async_rerank(
	socket_path: &Path,
	timeout: Duration,
	query: String,
	documents: Vec<String>,
) -> RetrievalResult<Vec<f32>> {
	let request =
		DaemonRequest::Rerank { query, documents };
	let response =
		send_async_request(socket_path, timeout, &request)
			.await?;
	match response {
		DaemonResponse::Scores(s) => Ok(s),
		DaemonResponse::Error(e) => {
			Err(RetrievalError::Embedding(e))
		}
		_ => {
			let msg = "unexpected response".to_string();
			Err(RetrievalError::DaemonCommunication(msg))
		}
	}
}

/// Expand query async
pub async fn async_expand(
	socket_path: &Path,
	timeout: Duration,
	query: String,
) -> RetrievalResult<SearchSpec> {
	let request = DaemonRequest::Expand { query };
	let response =
		send_async_request(socket_path, timeout, &request)
			.await?;
	match response {
		DaemonResponse::SearchSpec(s) => Ok(s),
		DaemonResponse::Error(e) => {
			Err(RetrievalError::Embedding(e))
		}
		_ => {
			let msg = "unexpected response".to_string();
			Err(RetrievalError::DaemonCommunication(msg))
		}
	}
}

/// Ping daemon async
pub async fn async_ping(
	socket_path: &Path,
	timeout: Duration,
) -> RetrievalResult<bool> {
	let request = DaemonRequest::Ping;
	let response =
		send_async_request(socket_path, timeout, &request)
			.await?;
	match response {
		DaemonResponse::Pong => Ok(true),
		DaemonResponse::Error(e) => {
			Err(RetrievalError::DaemonCommunication(e))
		}
		_ => Ok(false),
	}
}

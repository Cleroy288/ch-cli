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

/// Connect to daemon socket with timeout
async fn connect_async(
	socket_path: &Path,
	dur: Duration,
) -> RetrievalResult<UnixStream> {
	let connect = UnixStream::connect(socket_path);
	timeout(dur, connect)
		.await
		.map_err(|_| {
			let msg = "connection timeout".to_string();
			RetrievalError::DaemonCommunication(msg)
		})?
		.map_err(|err| {
			RetrievalError::DaemonNotRunning(
				err.to_string(),
			)
		})
}

/// Write serialized request to socket with timeout
async fn write_request(
	writer: &mut tokio::net::unix::OwnedWriteHalf,
	dur: Duration,
	request: &DaemonRequest,
) -> RetrievalResult<()> {
	let json = serde_json::to_string(request).map_err(
		|err| {
			let msg = format!("serialize: {}", err);
			RetrievalError::DaemonCommunication(msg)
		},
	)?;
	timeout(dur, async {
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
	.map_err(RetrievalError::IoError)
}

/// Read and deserialize response from socket
async fn read_response(
	reader: tokio::net::unix::OwnedReadHalf,
	dur: Duration,
) -> RetrievalResult<DaemonResponse> {
	let mut buf_reader = BufReader::new(reader);
	let mut line = String::new();
	timeout(dur, buf_reader.read_line(&mut line))
		.await
		.map_err(|_| {
			let msg = "read timeout".to_string();
			RetrievalError::DaemonCommunication(msg)
		})?
		.map_err(RetrievalError::IoError)?;
	serde_json::from_str(line.trim()).map_err(|err| {
		let msg = format!("deserialize: {}", err);
		RetrievalError::DaemonCommunication(msg)
	})
}

/// Send request to daemon asynchronously
pub async fn send_async_request(
	socket_path: &Path,
	request_timeout: Duration,
	request: &DaemonRequest,
) -> RetrievalResult<DaemonResponse> {
	let stream =
		connect_async(socket_path, request_timeout).await?;
	let (reader, mut writer) = stream.into_split();
	write_request(
		&mut writer, request_timeout, request,
	).await?;
	read_response(reader, request_timeout).await
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
		DaemonResponse::Embeddings(vecs) => Ok(vecs),
		DaemonResponse::Error(msg) => {
			Err(RetrievalError::Embedding(msg))
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
		DaemonResponse::Scores(scores) => Ok(scores),
		DaemonResponse::Error(msg) => {
			Err(RetrievalError::Embedding(msg))
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
		DaemonResponse::SearchSpec(spec) => Ok(spec),
		DaemonResponse::Error(msg) => {
			Err(RetrievalError::Embedding(msg))
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
		DaemonResponse::Error(msg) => {
			Err(RetrievalError::DaemonCommunication(msg))
		}
		_ => Ok(false),
	}
}

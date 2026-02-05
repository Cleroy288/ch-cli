//! Async Daemon Client using Tokio
//!
//! Provides async versions of daemon communication methods
//! for better timeout handling and concurrent requests.

use std::path::PathBuf;
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tokio::time::timeout;

use super::protocol::{DaemonRequest, DaemonResponse, SearchSpec};
use crate::retrieval::{RetrievalConfig, RetrievalError, RetrievalResult};

/// Async daemon client for non-blocking ML operations
pub struct AsyncDaemonClient {
	/// path to the Unix socket
	socket_path: PathBuf,
	/// request timeout duration
	timeout: Duration,
}

impl AsyncDaemonClient {
	/// Create a new async daemon client with default config
	pub fn new() -> Self {
		let config = RetrievalConfig::default(); // default retrieval config
		Self {
			socket_path: config.socket_path,
			timeout: Duration::from_secs(30),
		}
	}

	/// Create with custom socket path
	pub fn with_socket_path(socket_path: PathBuf) -> Self {
		Self {
			socket_path,
			timeout: Duration::from_secs(30),
		}
	}

	/// Set custom timeout and return self for chaining
	pub fn with_timeout(mut self, timeout: Duration) -> Self {
		self.timeout = timeout;
		self
	}

	/// Send a request to the daemon asynchronously
	async fn send_request(&self, request: &DaemonRequest) -> RetrievalResult<DaemonResponse> {
		// Connect with timeout
		let stream = timeout(self.timeout, UnixStream::connect(&self.socket_path))
			.await
			.map_err(|_| RetrievalError::DaemonCommunication("connection timeout".to_string()))?
			.map_err(|e| RetrievalError::DaemonNotRunning(e.to_string()))?;

		let (reader, mut writer) = stream.into_split(); // split stream into reader and writer

		// Serialize request to JSON
		let json = serde_json::to_string(request)
			.map_err(|e| RetrievalError::DaemonCommunication(format!("serialize: {}", e)))?;

		// Write request with timeout
		timeout(self.timeout, async {
			writer.write_all(json.as_bytes()).await?;
			writer.write_all(b"\n").await?;
			writer.flush().await?;
			Ok::<_, std::io::Error>(())
		})
		.await
		.map_err(|_| RetrievalError::DaemonCommunication("write timeout".to_string()))?
		.map_err(|e| RetrievalError::Io(e))?;

		// Read response with timeout
		let mut reader = BufReader::new(reader); // buffered reader for line reading
		let mut line = String::new(); // response line buffer

		timeout(self.timeout, reader.read_line(&mut line))
			.await
			.map_err(|_| RetrievalError::DaemonCommunication("read timeout".to_string()))?
			.map_err(|e| RetrievalError::Io(e))?;

		// Deserialize response from JSON
		serde_json::from_str(line.trim())
			.map_err(|e| RetrievalError::DaemonCommunication(format!("deserialize: {}", e)))
	}

	/// Ping the daemon to check if it's alive
	pub async fn ping(&self) -> RetrievalResult<bool> {
		let response = self.send_request(&DaemonRequest::Ping).await?;
		match response {
			DaemonResponse::Pong => Ok(true),
			DaemonResponse::Error(e) => Err(RetrievalError::DaemonCommunication(e)),
			_ => Ok(false),
		}
	}

	/// Generate embeddings for texts asynchronously
	pub async fn embed(&self, texts: Vec<String>) -> RetrievalResult<Vec<Vec<f32>>> {
		let response = self.send_request(&DaemonRequest::Embed { texts }).await?;
		match response {
			DaemonResponse::Embeddings(embeddings) => Ok(embeddings),
			DaemonResponse::Error(e) => Err(RetrievalError::Embedding(e)),
			_ => Err(RetrievalError::DaemonCommunication("unexpected response".to_string())),
		}
	}

	/// Rerank documents asynchronously given a query
	pub async fn rerank(&self, query: String, documents: Vec<String>) -> RetrievalResult<Vec<f32>> {
		let response = self
			.send_request(&DaemonRequest::Rerank { query, documents })
			.await?;
		match response {
			DaemonResponse::Scores(scores) => Ok(scores),
			DaemonResponse::Error(e) => Err(RetrievalError::Embedding(e)),
			_ => Err(RetrievalError::DaemonCommunication("unexpected response".to_string())),
		}
	}

	/// Expand a query asynchronously into SearchSpec
	pub async fn expand(&self, query: String) -> RetrievalResult<SearchSpec> {
		let response = self
			.send_request(&DaemonRequest::Expand { query })
			.await?;
		match response {
			DaemonResponse::SearchSpec(spec) => Ok(spec),
			DaemonResponse::Error(e) => Err(RetrievalError::Embedding(e)),
			_ => Err(RetrievalError::DaemonCommunication("unexpected response".to_string())),
		}
	}
}

impl Default for AsyncDaemonClient {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_async_client_creation() {
		let client = AsyncDaemonClient::new();
		assert_eq!(client.timeout, Duration::from_secs(30));
	}

	#[test]
	fn test_async_client_with_timeout() {
		let client = AsyncDaemonClient::new().with_timeout(Duration::from_secs(60));
		assert_eq!(client.timeout, Duration::from_secs(60));
	}

	#[test]
	fn test_async_client_with_socket_path() {
		let path = PathBuf::from("/tmp/test.sock");
		let client = AsyncDaemonClient::with_socket_path(path.clone());
		assert_eq!(client.socket_path, path);
	}
}

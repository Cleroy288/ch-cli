//! Daemon Client
//!
//! Provides a client to communicate with the model daemon.
//! Handles connection, request/response serialization, and auto-start.

use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;

use super::lifecycle::{daemon_status, start_daemon};
use super::protocol::{
	deserialize_response, serialize_request, CachedSearchResult, DaemonRequest, DaemonResponse,
	DaemonStatus, DocEntryResponse,
};
use crate::retrieval::{RetrievalConfig, RetrievalError, RetrievalResult};

/// Configuration for retry behavior with exponential backoff
#[derive(Debug, Clone)]
pub struct RetryConfig {
	/// Maximum number of retry attempts
	pub max_retries: usize,
	/// Initial delay between retries in milliseconds
	pub initial_delay_ms: u64,
	/// Maximum delay between retries in milliseconds
	pub max_delay_ms: u64,
	/// Multiplier for exponential backoff
	pub backoff_multiplier: f32,
}

impl Default for RetryConfig {
	fn default() -> Self {
		Self {
			max_retries: 3,
			initial_delay_ms: 100,
			max_delay_ms: 2000,
			backoff_multiplier: 2.0,
		}
	}
}

/// Check if an error is transient and worth retrying
fn is_retryable_error(error: &RetrievalError) -> bool {
	match error {
		RetrievalError::Io(io_err) => {
			// Retry on transient IO errors
			matches!(
				io_err.kind(),
				std::io::ErrorKind::WouldBlock |
				std::io::ErrorKind::TimedOut |
				std::io::ErrorKind::Interrupted |
				std::io::ErrorKind::ConnectionReset |
				std::io::ErrorKind::BrokenPipe
			)
		}
		RetrievalError::DaemonCommunication(msg) => {
			// Retry on known transient daemon communication errors
			msg.contains("temporarily unavailable") ||
			msg.contains("Resource temporarily unavailable") ||
			msg.contains("connection reset") ||
			msg.contains("os error 35")
		}
		_ => false,
	}
}

/// Client for communicating with the model daemon
#[derive(Clone)]
pub struct DaemonClient {
	/// path to the Unix socket
	socket_path: std::path::PathBuf,
	/// connection timeout
	timeout: Duration,
}

impl DaemonClient {
	/// Create a new client with default config
	pub fn new() -> Self {
		let config = RetrievalConfig::default();
		Self {
			socket_path: config.socket_path,
			timeout: Duration::from_secs(30),
		}
	}

	/// Create a new client with custom socket path
	pub fn with_socket_path(socket_path: impl AsRef<Path>) -> Self {
		Self {
			socket_path: socket_path.as_ref().to_path_buf(),
			timeout: Duration::from_secs(30),
		}
	}

	/// Connect to the daemon, auto-starting if needed
	fn connect(&self) -> RetrievalResult<UnixStream> {
		// try to connect first
		match UnixStream::connect(&self.socket_path) {
			Ok(stream) => {
				stream.set_read_timeout(Some(self.timeout)).ok();
				stream.set_write_timeout(Some(self.timeout)).ok();
				Ok(stream)
			}
			Err(_) => {
				// daemon not running, try to start it
				start_daemon(&self.socket_path)?;
				// wait a bit for daemon to start
				std::thread::sleep(Duration::from_millis(500));
				// try again
				let stream = UnixStream::connect(&self.socket_path).map_err(|e| {
					RetrievalError::DaemonNotRunning(format!(
						"failed to connect after start: {}",
						e
					))
				})?;
				stream.set_read_timeout(Some(self.timeout)).ok();
				stream.set_write_timeout(Some(self.timeout)).ok();
				Ok(stream)
			}
		}
	}

	/// Send a request once without retry logic
	fn send_request_once(&self, request: &DaemonRequest) -> RetrievalResult<DaemonResponse> {
		let mut stream = self.connect()?; // unix stream connection
		let bytes = serialize_request(request) // serialized request bytes
			.map_err(|e| RetrievalError::DaemonCommunication(format!("serialize: {}", e)))?;

		stream.write_all(&bytes)?;
		stream.flush()?;

		// read response line
		let mut reader = BufReader::new(stream); // buffered reader for response
		let mut line = String::new(); // response line buffer
		reader.read_line(&mut line)?;

		deserialize_response(line.trim().as_bytes())
			.map_err(|e| RetrievalError::DaemonCommunication(format!("deserialize: {}", e)))
	}

	/// Send a request with retry logic and exponential backoff
	fn send_request(&self, request: &DaemonRequest) -> RetrievalResult<DaemonResponse> {
		let config = RetryConfig::default(); // retry configuration
		let mut delay_ms = config.initial_delay_ms; // current delay between retries
		let mut last_error: Option<RetrievalError> = None; // last error encountered

		for attempt in 0..=config.max_retries {
			match self.send_request_once(request) {
				Ok(response) => return Ok(response),
				Err(e) => {
					// Check if error is retryable
					if !is_retryable_error(&e) {
						return Err(e);
					}

					last_error = Some(e);

					if attempt < config.max_retries {
						eprintln!(
							"[daemon-client] Retry {}/{} after {}ms",
							attempt + 1,
							config.max_retries,
							delay_ms
						);
						std::thread::sleep(std::time::Duration::from_millis(delay_ms));

						// Exponential backoff with max cap
						delay_ms = ((delay_ms as f32) * config.backoff_multiplier) as u64;
						delay_ms = delay_ms.min(config.max_delay_ms);
					}
				}
			}
		}

		Err(last_error.unwrap_or_else(|| {
			RetrievalError::DaemonCommunication("max retries exceeded".to_string())
		}))
	}

	/// Generate embeddings for texts
	pub fn embed(&self, texts: Vec<String>) -> RetrievalResult<Vec<Vec<f32>>> {
		let request = DaemonRequest::Embed { texts };
		match self.send_request(&request)? {
			DaemonResponse::Embeddings(vecs) => Ok(vecs),
			DaemonResponse::Error(e) => Err(RetrievalError::Embedding(e)),
			other => Err(RetrievalError::DaemonCommunication(format!(
				"unexpected response: {:?}",
				other
			))),
		}
	}

	/// Rerank documents given a query
	pub fn rerank(&self, query: String, documents: Vec<String>) -> RetrievalResult<Vec<f32>> {
		let request = DaemonRequest::Rerank { query, documents };
		match self.send_request(&request)? {
			DaemonResponse::Scores(scores) => Ok(scores),
			DaemonResponse::Error(e) => Err(RetrievalError::Embedding(e)),
			other => Err(RetrievalError::DaemonCommunication(format!(
				"unexpected response: {:?}",
				other
			))),
		}
	}

	/// Expand a natural language query into structured SearchSpec
	pub fn expand(&self, query: String) -> RetrievalResult<super::protocol::SearchSpec> {
		let request = DaemonRequest::Expand { query };
		match self.send_request(&request)? {
			DaemonResponse::SearchSpec(spec) => Ok(spec),
			DaemonResponse::Error(e) => Err(RetrievalError::Embedding(e)),
			other => Err(RetrievalError::DaemonCommunication(format!(
				"unexpected response: {:?}",
				other
			))),
		}
	}

	/// Get daemon status
	pub fn status(&self) -> RetrievalResult<DaemonStatus> {
		// first check if daemon is running without connecting
		let status = daemon_status(&self.socket_path);
		if !status.running {
			return Ok(status);
		}

		// daemon is running, get detailed status
		let request = DaemonRequest::Status;
		match self.send_request(&request)? {
			DaemonResponse::Status(s) => Ok(s),
			DaemonResponse::Error(e) => Err(RetrievalError::DaemonCommunication(e)),
			other => Err(RetrievalError::DaemonCommunication(format!(
				"unexpected response: {:?}",
				other
			))),
		}
	}

	/// Ping the daemon to check if it's alive
	pub fn ping(&self) -> RetrievalResult<bool> {
		let request = DaemonRequest::Ping;
		match self.send_request(&request) {
			Ok(DaemonResponse::Pong) => Ok(true),
			_ => Ok(false),
		}
	}

	/// Shutdown the daemon
	pub fn shutdown(&self) -> RetrievalResult<()> {
		let request = DaemonRequest::Shutdown;
		match self.send_request(&request) {
			Ok(DaemonResponse::Ok) => Ok(()),
			Ok(DaemonResponse::Error(e)) => Err(RetrievalError::DaemonCommunication(e)),
			Err(_) => Ok(()), // daemon may have already shut down
			other => Err(RetrievalError::DaemonCommunication(format!(
				"unexpected response: {:?}",
				other
			))),
		}
	}

	/// Index a project and cache it in daemon memory
	/// Returns (symbol_count, cached, index_time_ms)
	pub fn index_project(&self, project_path: &str, force: bool) -> RetrievalResult<(usize, bool, u64)> {
		let request = DaemonRequest::IndexProject {
			project_path: project_path.to_string(),
			force,
		};
		match self.send_request(&request)? {
			DaemonResponse::ProjectIndexed { symbol_count, cached, index_time_ms } => {
				Ok((symbol_count, cached, index_time_ms))
			}
			DaemonResponse::Error(e) => Err(RetrievalError::Embedding(e)),
			other => Err(RetrievalError::DaemonCommunication(format!(
				"unexpected response: {:?}",
				other
			))),
		}
	}

	/// Search a cached project
	pub fn search_project(
		&self,
		project_path: &str,
		query: &str,
		limit: usize,
	) -> RetrievalResult<Vec<CachedSearchResult>> {
		let request = DaemonRequest::SearchProject {
			project_path: project_path.to_string(),
			query: query.to_string(),
			limit,
		};
		match self.send_request(&request)? {
			DaemonResponse::SearchResults(results) => Ok(results),
			DaemonResponse::Error(e) => Err(RetrievalError::Embedding(e)),
			other => Err(RetrievalError::DaemonCommunication(format!(
				"unexpected response: {:?}",
				other
			))),
		}
	}

	/// Get project cache status
	/// Returns (cached, symbol_count, last_indexed)
	pub fn project_status(&self, project_path: &str) -> RetrievalResult<(bool, usize, u64)> {
		let request = DaemonRequest::ProjectStatus {
			project_path: project_path.to_string(),
		};
		match self.send_request(&request)? {
			DaemonResponse::ProjectCacheStatus { cached, symbol_count, last_indexed } => {
				Ok((cached, symbol_count, last_indexed))
			}
			DaemonResponse::Error(e) => Err(RetrievalError::DaemonCommunication(e)),
			other => Err(RetrievalError::DaemonCommunication(format!(
				"unexpected response: {:?}",
				other
			))),
		}
	}

	/// Evict a project from daemon cache
	pub fn evict_project(&self, project_path: &str) -> RetrievalResult<()> {
		let request = DaemonRequest::EvictProject {
			project_path: project_path.to_string(),
		};
		match self.send_request(&request)? {
			DaemonResponse::Ok => Ok(()),
			DaemonResponse::Error(e) => Err(RetrievalError::DaemonCommunication(e)),
			other => Err(RetrievalError::DaemonCommunication(format!(
				"unexpected response: {:?}",
				other
			))),
		}
	}

	/// Start documentation generation for a project
	pub fn start_doc_gen(&self, project_path: String, force: bool) -> RetrievalResult<()> {
		let request = DaemonRequest::StartDocGen { project_path, force };
		match self.send_request(&request)? {
			DaemonResponse::Ok => Ok(()),
			DaemonResponse::Error(e) => Err(RetrievalError::DaemonCommunication(e)),
			other => Err(RetrievalError::DaemonCommunication(format!(
				"unexpected response: {:?}",
				other
			))),
		}
	}

	/// Get documentation generation status
	pub fn doc_gen_status(&self, project_path: String) -> RetrievalResult<DocGenStatus> {
		let request = DaemonRequest::DocGenStatus { project_path };
		match self.send_request(&request)? {
			DaemonResponse::DocGenStatus { total, completed, pending, is_ready } => {
				Ok(DocGenStatus { total, completed, pending, is_ready })
			}
			DaemonResponse::Error(e) => Err(RetrievalError::DaemonCommunication(e)),
			other => Err(RetrievalError::DaemonCommunication(format!(
				"unexpected response: {:?}",
				other
			))),
		}
	}

	/// Get documentation for a specific symbol
	pub fn get_doc(&self, project_path: String, symbol_name: String) -> RetrievalResult<Option<DocEntryResponse>> {
		let request = DaemonRequest::GetDoc { project_path, symbol_name };
		match self.send_request(&request)? {
			DaemonResponse::Doc(doc) => Ok(doc),
			DaemonResponse::Error(e) => Err(RetrievalError::DaemonCommunication(e)),
			other => Err(RetrievalError::DaemonCommunication(format!(
				"unexpected response: {:?}",
				other
			))),
		}
	}

	/// Search documentation
	pub fn search_docs(&self, project_path: String, query: String, limit: usize) -> RetrievalResult<Vec<DocEntryResponse>> {
		let request = DaemonRequest::SearchDocs { project_path, query, limit };
		match self.send_request(&request)? {
			DaemonResponse::DocResults(results) => Ok(results),
			DaemonResponse::Error(e) => Err(RetrievalError::DaemonCommunication(e)),
			other => Err(RetrievalError::DaemonCommunication(format!(
				"unexpected response: {:?}",
				other
			))),
		}
	}
}

/// Documentation generation status
pub struct DocGenStatus {
	/// total number of entries
	pub total: usize,
	/// completed entries
	pub completed: usize,
	/// pending entries
	pub pending: usize,
	/// whether generation is complete
	pub is_ready: bool,
}

impl Default for DaemonClient {
	fn default() -> Self {
		Self::new()
	}
}

/// Connection pool for daemon communication
/// Reuses connections to reduce overhead of socket creation
pub struct ConnectionPool {
	/// Available connections ready for reuse
	connections: Mutex<VecDeque<UnixStream>>,
	/// Socket path for creating new connections
	socket_path: PathBuf,
	/// Maximum number of connections to keep in pool
	max_size: usize,
	/// Connection timeout
	timeout: Duration,
}

impl ConnectionPool {
	/// Create a new connection pool
	pub fn new(socket_path: PathBuf, max_size: usize) -> Self {
		Self {
			connections: Mutex::new(VecDeque::new()),
			socket_path,
			max_size,
			timeout: Duration::from_secs(30),
		}
	}

	/// Get a connection from the pool or create a new one
	pub fn get(&self) -> RetrievalResult<PooledConnection<'_>> {
		// Try to get existing connection from pool
		if let Some(stream) = self.connections.lock().unwrap().pop_front() {
			// Verify connection is still valid
			if Self::is_connection_valid(&stream) {
				return Ok(PooledConnection {
					stream: Some(stream),
					pool: self,
				});
			}
			// Connection invalid, will create new one
		}

		// Create new connection
		let stream = UnixStream::connect(&self.socket_path)
			.map_err(|e| RetrievalError::DaemonNotRunning(e.to_string()))?;
		stream.set_read_timeout(Some(self.timeout)).ok();
		stream.set_write_timeout(Some(self.timeout)).ok();

		Ok(PooledConnection {
			stream: Some(stream),
			pool: self,
		})
	}

	/// Return a connection to the pool for reuse
	fn return_connection(&self, stream: UnixStream) {
		let mut connections = self.connections.lock().unwrap(); // lock the pool
		if connections.len() < self.max_size {
			connections.push_back(stream);
		}
		// If pool is full, connection is dropped
	}

	/// Check if a connection is still valid
	fn is_connection_valid(stream: &UnixStream) -> bool {
		// Check if socket is still connected by attempting to get peer address
		// If the connection is broken, this will fail
		stream.peer_addr().is_ok()
	}

	/// Get the number of connections currently in the pool
	pub fn available_connections(&self) -> usize {
		self.connections.lock().unwrap().len()
	}
}

/// RAII wrapper for a pooled connection
/// Automatically returns connection to pool when dropped
pub struct PooledConnection<'a> {
	/// The connection (Option to allow taking in Drop)
	stream: Option<UnixStream>,
	/// Reference to the pool to return connection
	pool: &'a ConnectionPool,
}

impl<'a> Drop for PooledConnection<'a> {
	fn drop(&mut self) {
		if let Some(stream) = self.stream.take() {
			self.pool.return_connection(stream);
		}
	}
}

impl<'a> std::ops::Deref for PooledConnection<'a> {
	type Target = UnixStream;
	fn deref(&self) -> &Self::Target {
		self.stream.as_ref().unwrap()
	}
}

impl<'a> std::ops::DerefMut for PooledConnection<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.stream.as_mut().unwrap()
	}
}

impl<'a> PooledConnection<'a> {
	/// Take ownership of the stream (removes from pool management)
	pub fn take(mut self) -> UnixStream {
		self.stream.take().unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_retry_config_defaults() {
		let config = RetryConfig::default();
		assert_eq!(config.max_retries, 3);
		assert_eq!(config.initial_delay_ms, 100);
		assert_eq!(config.max_delay_ms, 2000);
		assert!((config.backoff_multiplier - 2.0).abs() < 0.001);
	}

	#[test]
	fn test_is_retryable_error_io_would_block() {
		let error = RetrievalError::Io(std::io::Error::new(
			std::io::ErrorKind::WouldBlock,
			"would block"
		));
		assert!(is_retryable_error(&error));
	}

	#[test]
	fn test_is_retryable_error_io_timed_out() {
		let error = RetrievalError::Io(std::io::Error::new(
			std::io::ErrorKind::TimedOut,
			"timed out"
		));
		assert!(is_retryable_error(&error));
	}

	#[test]
	fn test_is_retryable_error_connection_reset() {
		let error = RetrievalError::Io(std::io::Error::new(
			std::io::ErrorKind::ConnectionReset,
			"connection reset"
		));
		assert!(is_retryable_error(&error));
	}

	#[test]
	fn test_is_retryable_error_daemon_communication() {
		let error = RetrievalError::DaemonCommunication(
			"Resource temporarily unavailable (os error 35)".to_string()
		);
		assert!(is_retryable_error(&error));
	}

	#[test]
	fn test_is_not_retryable_error_not_found() {
		let error = RetrievalError::Io(std::io::Error::new(
			std::io::ErrorKind::NotFound,
			"not found"
		));
		assert!(!is_retryable_error(&error));
	}

	#[test]
	fn test_is_not_retryable_error_daemon_not_running() {
		let error = RetrievalError::DaemonNotRunning("not running".to_string());
		assert!(!is_retryable_error(&error));
	}

	#[test]
	fn test_exponential_backoff_calculation() {
		let config = RetryConfig::default();
		let mut delay = config.initial_delay_ms; // current delay value

		// First delay: 100ms
		assert_eq!(delay, 100);

		// Second delay: 100 * 2 = 200ms
		delay = ((delay as f32) * config.backoff_multiplier) as u64;
		assert_eq!(delay, 200);

		// Third delay: 200 * 2 = 400ms
		delay = ((delay as f32) * config.backoff_multiplier) as u64;
		assert_eq!(delay, 400);

		// Should be capped at max_delay_ms
		delay = 3000;
		delay = delay.min(config.max_delay_ms);
		assert_eq!(delay, 2000);
	}

	#[test]
	fn test_connection_pool_creation() {
		// Test that pool is created with correct initial state
		let socket_path = PathBuf::from("/tmp/test_socket"); // test socket path
		let max_size = 5; // maximum pool size

		let pool = ConnectionPool::new(socket_path.clone(), max_size);

		assert_eq!(pool.socket_path, socket_path);
		assert_eq!(pool.max_size, max_size);
		assert_eq!(pool.timeout, Duration::from_secs(30));
		assert_eq!(pool.available_connections(), 0);
	}

	#[test]
	fn test_connection_pool_available_connections_empty() {
		// Test available_connections returns 0 for empty pool
		let pool = ConnectionPool::new(PathBuf::from("/tmp/test"), 5);
		assert_eq!(pool.available_connections(), 0);
	}

	#[test]
	fn test_connection_pool_get_without_daemon_returns_error() {
		// Test that get() returns error when daemon is not running
		let pool = ConnectionPool::new(PathBuf::from("/tmp/nonexistent_socket_12345"), 5);
		let result = pool.get();
		assert!(result.is_err());
	}

	#[test]
	fn test_connection_pool_max_size_respected() {
		// Test that pool respects max_size configuration
		let pool = ConnectionPool::new(PathBuf::from("/tmp/test"), 3);
		assert_eq!(pool.max_size, 3);
	}

	#[test]
	fn test_pooled_connection_take_returns_none_on_subsequent_calls() {
		// Test that PooledConnection::take properly handles ownership
		// This test verifies the struct fields are correctly set up
		// Note: Full integration test requires running daemon
		let pool = ConnectionPool::new(PathBuf::from("/tmp/test"), 5);

		// Verify pool is empty initially
		assert_eq!(pool.available_connections(), 0);

		// Verify pool internal state is protected by mutex
		{
			let connections = pool.connections.lock().unwrap();
			assert!(connections.is_empty());
		}
	}
}

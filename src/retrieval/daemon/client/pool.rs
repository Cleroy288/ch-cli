//! Connection Pool
//!
//! Provides connection pooling for daemon communication.

use std::collections::VecDeque;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Duration;

use crate::retrieval::RetrievalResult;

/// Connection pool for daemon communication
/// Reuses connections to reduce socket overhead
pub struct ConnectionPool {
	/// Available connections ready for reuse
	#[doc(hidden)]
	pub connections: Mutex<VecDeque<UnixStream>>,
	/// Socket path for creating new connections
	pub(crate) socket_path: PathBuf,
	/// Maximum number of connections to keep in pool
	pub(crate) max_size: usize,
	/// Connection timeout
	pub(crate) timeout: Duration,
}

impl ConnectionPool {
	/// Create a new connection pool
	pub fn new(
		socket_path: PathBuf,
		max_size: usize,
	) -> Self {
		Self {
			connections: Mutex::new(VecDeque::new()),
			socket_path,
			max_size,
			timeout: Duration::from_secs(30),
		}
	}

	/// Get a connection from pool or create new one
	pub fn get(
		&self,
	) -> RetrievalResult<PooledConnection<'_>> {
		super::pool_ops::get_connection(self)
	}

	/// Get number of connections in the pool
	pub fn available_connections(&self) -> usize {
		super::pool_getters::available_connections(self)
	}

	/// Get the socket path
	#[doc(hidden)]
	pub fn socket_path(&self) -> &PathBuf {
		super::pool_getters::socket_path(self)
	}

	/// Get the max size
	#[doc(hidden)]
	pub fn max_size(&self) -> usize {
		super::pool_getters::max_size(self)
	}

	/// Get the timeout
	#[doc(hidden)]
	pub fn timeout(&self) -> Duration {
		super::pool_getters::timeout(self)
	}
}

/// RAII wrapper for a pooled connection
/// Automatically returns connection to pool when dropped
#[allow(clippy::min_ident_chars)]
pub struct PooledConnection<'a> {
	/// The connection (Option to allow taking in Drop)
	stream: Option<UnixStream>,
	/// Reference to the pool to return connection
	pool: &'a ConnectionPool,
}

#[allow(clippy::min_ident_chars)]
impl<'a> PooledConnection<'a> {
	/// Create a new pooled connection
	pub(super) fn new(
		stream: UnixStream,
		pool: &'a ConnectionPool,
	) -> Self {
		Self {
			stream: Some(stream),
			pool,
		}
	}

	/// Take ownership of stream (remove from pool)
	pub fn take(mut self) -> UnixStream {
		self.stream.take().unwrap()
	}
}

#[allow(clippy::min_ident_chars)]
impl<'a> Drop for PooledConnection<'a> {
	fn drop(&mut self) {
		if let Some(stream) = self.stream.take() {
			let conns = &self.pool.connections;
			let max = self.pool.max_size;
			super::pool_ops::return_connection(
				conns, max, stream,
			);
		}
	}
}

#[allow(clippy::min_ident_chars)]
impl<'a> std::ops::Deref for PooledConnection<'a> {
	type Target = UnixStream;
	fn deref(&self) -> &Self::Target {
		self.stream.as_ref().unwrap()
	}
}

#[allow(clippy::min_ident_chars)]
impl<'a> std::ops::DerefMut for PooledConnection<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.stream.as_mut().unwrap()
	}
}


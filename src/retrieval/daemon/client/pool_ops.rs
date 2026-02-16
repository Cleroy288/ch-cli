//! Connection Pool Operations
//!
//! Provides core operations for connection pooling.

use std::collections::VecDeque;
use std::os::unix::net::UnixStream;
use std::sync::Mutex;

use crate::retrieval::{RetrievalError, RetrievalResult};

use super::pool::{ConnectionPool, PooledConnection};

/// Get a connection from the pool or create a new one
#[allow(clippy::min_ident_chars)]
pub fn get_connection(
	pool: &ConnectionPool,
) -> RetrievalResult<PooledConnection<'_>> {
	// Try to get existing connection from pool
	let recycled = {
		let mut guard = pool.connections.lock()
			.map_err(|_| {
				RetrievalError::DaemonCommunication(
					"lock poisoned".into(),
				)
			})?;
		guard.pop_front()
	};
	if let Some(stream) = recycled {
		if is_connection_valid(&stream) {
			return Ok(PooledConnection::new(stream, pool));
		}
	}

	// Create new connection
	create_new_connection(pool)
}

/// Create a new connection to the daemon
#[allow(clippy::min_ident_chars)]
fn create_new_connection(
	pool: &ConnectionPool,
) -> RetrievalResult<PooledConnection<'_>> {
	let socket_path = pool.socket_path();
	let timeout = pool.timeout();

	let stream = UnixStream::connect(socket_path)
		.map_err(|err| {
			RetrievalError::DaemonNotRunning(
				err.to_string(),
			)
		})?;
	stream.set_read_timeout(Some(timeout)).ok();
	stream.set_write_timeout(Some(timeout)).ok();

	Ok(PooledConnection::new(stream, pool))
}

/// Check if a connection is still valid
fn is_connection_valid(stream: &UnixStream) -> bool {
	// If the connection is broken, peer_addr will fail
	stream.peer_addr().is_ok()
}

/// Return a connection to the pool for reuse
pub fn return_connection(
	connections: &Mutex<VecDeque<UnixStream>>,
	max_size: usize,
	stream: UnixStream,
) {
	let Ok(mut conns) = connections.lock() else {
		return;
	};
	if conns.len() < max_size {
		conns.push_back(stream);
	}
	// If pool is full, connection is dropped
}

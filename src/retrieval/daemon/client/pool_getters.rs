//! Connection Pool Getter Methods
//!
//! Provides getter methods for ConnectionPool.

use super::pool::ConnectionPool;
use std::path::PathBuf;
use std::time::Duration;

/// Get number of connections in the pool
pub fn available_connections(pool: &ConnectionPool) -> usize {
	pool.connections.lock().unwrap().len()
}

/// Get the socket path
#[doc(hidden)]
pub fn socket_path(pool: &ConnectionPool) -> &PathBuf {
	&pool.socket_path
}

/// Get the max size
#[doc(hidden)]
pub fn max_size(pool: &ConnectionPool) -> usize {
	pool.max_size
}

/// Get the timeout
#[doc(hidden)]
pub fn timeout(pool: &ConnectionPool) -> Duration {
	pool.timeout
}

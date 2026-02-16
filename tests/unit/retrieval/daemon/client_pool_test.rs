//! Tests for retrieval::daemon::client::pool

use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::time::Duration;

use rustean::retrieval::daemon::client::pool::{
	ConnectionPool,
};
use rustean::retrieval::daemon::client::pool_ops;

/// Test pool creation with custom parameters
#[test]
fn test_pool_starts_empty() {
	let pool = ConnectionPool::new(
		PathBuf::from("/tmp/test.sock"), 5,
	);
	assert_eq!(pool.available_connections(), 0);
}

/// Test ConnectionPool creation with custom parameters
#[test]
fn test_connection_pool_new() {
	let socket_path =
		PathBuf::from("/tmp/test-pool.sock");
	let max_size = 5;

	let pool = ConnectionPool::new(
		socket_path.clone(),
		max_size,
	);

	assert_eq!(pool.socket_path(), &socket_path);
	assert_eq!(pool.max_size(), max_size);
	assert_eq!(pool.available_connections(), 0);
	assert_eq!(
		pool.timeout(),
		Duration::from_secs(30),
	);
}

/// Test return_connection via pool_ops respects max
#[test]
fn test_return_connection_respects_max() {
	let sock = std::env::temp_dir()
		.join("test-pool-ret.sock");
	let _ = std::fs::remove_file(&sock);
	let listener = UnixListener::bind(&sock)
		.expect("bind");
	let pool = ConnectionPool::new(sock.clone(), 1);

	// Create and return 2 connections
	for _ in 0..2 {
		let s = sock.clone();
		let t = std::thread::spawn(move || {
			UnixStream::connect(&s).expect("connect")
		});
		let _ = listener.accept().expect("accept");
		let cs = t.join().expect("join");
		pool_ops::return_connection(
			&pool.connections, pool.max_size(), cs,
		);
	}
	// max=1 so only 1 kept
	assert_eq!(pool.available_connections(), 1);

	drop(listener);
	let _ = std::fs::remove_file(&sock);
}

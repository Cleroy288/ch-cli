//! Tests for retrieval::daemon::async_client

use std::path::PathBuf;
use std::time::Duration;

use rustean::retrieval::daemon::AsyncDaemonClient;

#[test]
fn test_async_client_creation() {
	let client = AsyncDaemonClient::new();
	assert_eq!(
		client.timeout(),
		Duration::from_secs(30),
	);
}

#[test]
fn test_async_client_with_timeout() {
	let client = AsyncDaemonClient::new()
		.with_timeout(Duration::from_secs(60));
	assert_eq!(
		client.timeout(),
		Duration::from_secs(60),
	);
}

#[test]
fn test_async_client_with_socket_path() {
	let path = PathBuf::from("/tmp/test.sock");
	let client =
		AsyncDaemonClient::with_socket_path(path.clone());
	assert_eq!(client.socket_path(), &path);
}

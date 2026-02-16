//! Tests for retrieval::daemon::server::lifecycle

use std::fs;
use std::io::Read;
use std::path::PathBuf;

use rustean::retrieval::daemon::server::lifecycle::{
	loaded_models, new_daemon, with_socket_path,
	write_pid_file,
};

/// Test write_pid_file creates correct PID
#[test]
fn test_write_pid_file() {
	let temp_dir = std::env::temp_dir();
	let socket =
		temp_dir.join("test-daemon.sock");
	let pid_file = socket.with_extension("pid");

	let _ = fs::remove_file(&pid_file);

	let daemon = with_socket_path(&socket);
	write_pid_file(&daemon)
		.expect("Failed to write PID file");

	assert!(pid_file.exists());
	let mut contents = String::new();
	fs::File::open(&pid_file)
		.expect("Failed to open PID file")
		.read_to_string(&mut contents)
		.expect("Failed to read PID file");

	let expected = std::process::id().to_string();
	assert_eq!(contents, expected);

	let _ = fs::remove_file(&pid_file);
}

/// Test loaded_models returns empty when none loaded
#[test]
fn test_loaded_models_empty() {
	let temp_dir = std::env::temp_dir();
	let socket =
		temp_dir.join("test-daemon-models.sock");
	let daemon = with_socket_path(&socket);

	let models = loaded_models(&daemon);
	// Phi-3 shows as lazy (not loaded) but still listed
	assert_eq!(models.len(), 1);
}

/// Test new_daemon creates daemon with defaults
#[test]
fn test_new_daemon() {
	let daemon = new_daemon();

	assert!(daemon.embedder.is_none());
	assert!(daemon.reranker.is_none());
	assert!(daemon.project_cache.is_empty());
}

/// Test with_socket_path uses custom path
#[test]
fn test_with_socket_path() {
	let custom =
		PathBuf::from("/tmp/custom-daemon.sock");
	let daemon = with_socket_path(&custom);

	assert_eq!(daemon.socket_path, custom);
}

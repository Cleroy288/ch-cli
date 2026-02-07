//! Tests for the auto-update watch mode
//!
//! Unit tests for WatcherHandle stop behavior.

use std::time::Duration;

use ch_cli::startup::spawn_watcher;
use tempfile::tempdir;

#[test]
fn test_watcher_handle_stop() {
    let dir = tempdir().unwrap(); // temp directory for watching
    let handle = spawn_watcher(dir.path());

    // let watcher initialize
    std::thread::sleep(Duration::from_millis(200));

    // stop should not hang and thread should join cleanly
    handle.stop();
}

#[test]
fn test_watcher_handle_stop_nonexistent_dir() {
    // watcher with invalid path should still create handle and stop cleanly
    let handle = spawn_watcher(std::path::Path::new(
        "/tmp/nonexistent_watcher_test_dir_12345",
    ));

    std::thread::sleep(Duration::from_millis(200));
    handle.stop();
}

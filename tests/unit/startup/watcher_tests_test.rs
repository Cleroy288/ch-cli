//! Tests for the auto-update watch mode
//!
//! Unit tests for WatcherHandle stop behavior.

use std::time::Duration;

use rustean::fs::FileCache;
use rustean::startup::spawn_watcher;
use tempfile::tempdir;

#[test]
fn test_watcher_handle_stop() {
    let dir = tempdir().unwrap();
    let cache = FileCache::empty();
    let handle = spawn_watcher(dir.path(), cache);

    // let watcher initialize
    std::thread::sleep(Duration::from_millis(200));

    // stop should not hang and join cleanly
    handle.stop();
}

#[test]
fn test_watcher_handle_stop_nonexistent_dir() {
    let cache = FileCache::empty();
    let handle = spawn_watcher(
        std::path::Path::new(
            "/tmp/nonexistent_watcher_test_dir_12345",
        ),
        cache,
    );

    std::thread::sleep(Duration::from_millis(200));
    handle.stop();
}

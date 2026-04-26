//! Tests for file watcher stop behavior.
//!
//! These tests verify WatcherHandle::stop joins
//! cleanly. The sleep gives the OS time to
//! register the filesystem watcher before we
//! tear it down -- required for deterministic
//! shutdown on macOS (FSEvents) and Linux
//! (inotify). This is inherently timing-
//! sensitive; the 200ms is a pragmatic floor.

use std::time::Duration;

use rustean::fs::FileCache;
use rustean::startup::spawn_watcher;
use rustean::startup::watcher::new_watcher_msg;
use tempfile::tempdir;

/// Watcher on a real directory stops cleanly
#[test]
fn watcher_stop_on_real_dir() {
    // Arrange
    let dir = tempdir().unwrap();
    let cache = FileCache::empty();
    let msg = new_watcher_msg();
    let handle =
        spawn_watcher(dir.path(), cache, msg);

    // Wait for OS watcher registration
    std::thread::sleep(
        Duration::from_millis(200),
    );

    // Act + Assert -- stop must not hang
    handle.stop();
}

/// Watcher on nonexistent directory stops cleanly
#[test]
fn watcher_stop_on_nonexistent_dir() {
    // Arrange
    let cache = FileCache::empty();
    let msg = new_watcher_msg();
    let handle = spawn_watcher(
        std::path::Path::new(
            "/tmp/nonexistent_watcher_12345",
        ),
        cache,
        msg,
    );

    // Wait for OS watcher registration attempt
    std::thread::sleep(
        Duration::from_millis(200),
    );

    // Act + Assert -- stop must not hang
    handle.stop();
}

//! Auto-update watch mode for file changes.
//!
//! Watches for file changes, re-indexes,
//! and triggers doc regeneration.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::fs::FileCache;
use crate::indexer::{FileWatcher, IndexManager};
use crate::retrieval::daemon::DaemonClient;

/// Debounce delay before triggering reindex
const DEBOUNCE_MS: u64 = 500;

/// Poll interval for checking file events
const POLL_TIMEOUT_MS: u64 = 100;

/// Handle to the background watcher thread
pub struct WatcherHandle {
    /// flag to signal the watcher to stop
    stop_flag: Arc<AtomicBool>,
    /// handle to the watcher thread
    thread_handle:
        Option<std::thread::JoinHandle<()>>,
}

impl WatcherHandle {
    /// Stop the watcher and wait for thread to finish
    pub fn stop(mut self) {
        self.stop_flag.store(true, Ordering::SeqCst);
        if let Some(handle) = self.thread_handle.take()
        {
            let _ = handle.join();
        }
    }
}

/// Spawn background watcher thread for auto-reindex
pub fn spawn_watcher(
    project_path: &Path,
    file_cache: FileCache,
) -> WatcherHandle {
    let stop_flag =
        Arc::new(AtomicBool::new(false));
    let stop_clone = stop_flag.clone();
    let path = project_path.to_path_buf();

    let handle = std::thread::spawn(move || {
        watcher_loop(&path, &stop_clone, &file_cache);
    });

    WatcherHandle {
        stop_flag,
        thread_handle: Some(handle),
    }
}

/// Log a watcher message to stderr
fn log_watcher(msg: &str) {
    let _ = writeln!(
        std::io::stderr(),
        "[watcher] {}",
        msg
    );
}

/// Initialize the file watcher, returning None on failure
fn init_watcher(
    project_path: &Path,
) -> Option<FileWatcher> {
    let mut watcher =
        match FileWatcher::new(project_path) {
            Ok(wtr) => wtr,
            Err(err) => {
                log_watcher(&format!(
                    "Failed to create watcher: {}",
                    err
                ));
                return None;
            }
        };

    if let Err(err) = watcher.start() {
        log_watcher(&format!(
            "Failed to start watching: {}",
            err
        ));
        return None;
    }
    Some(watcher)
}

/// Main watcher loop - watches, debounces, reindexes
fn watcher_loop(
    project_path: &Path,
    stop_flag: &Arc<AtomicBool>,
    file_cache: &FileCache,
) {
    let Some(mut watcher) =
        init_watcher(project_path)
    else {
        return;
    };

    log_watcher(&format!(
        "Watching {} for changes...",
        project_path.display()
    ));

    run_watch_loop(
        &mut watcher, project_path, stop_flag,
        file_cache,
    );

    let _ = watcher.stop();
    log_watcher("Stopped");
}

/// Check if debounce period has elapsed
fn should_flush_changes(
    last_change: &Option<Instant>,
    debounce: Duration,
    has_changes: bool,
) -> bool {
    last_change
        .is_some_and(|time| time.elapsed() >= debounce)
        && has_changes
}

/// Run the file watch polling loop
fn run_watch_loop(
    watcher: &mut FileWatcher,
    project_path: &Path,
    stop_flag: &Arc<AtomicBool>,
    file_cache: &FileCache,
) {
    let mut last_change: Option<Instant> = None;
    let mut changed: Vec<PathBuf> = Vec::new();
    let debounce =
        Duration::from_millis(DEBOUNCE_MS);

    loop {
        if stop_flag.load(Ordering::Relaxed) {
            break;
        }
        poll_events(
            watcher, &mut changed, &mut last_change,
        );
        let should_flush = should_flush_changes(
            &last_change, debounce, !changed.is_empty(),
        );
        if !should_flush {
            continue;
        }
        flush_changes(
            project_path, file_cache,
            &mut changed, &mut last_change,
        );
    }
}

/// Reindex, update docs, and refresh file cache
fn flush_changes(
    project_path: &Path,
    file_cache: &FileCache,
    changed: &mut Vec<PathBuf>,
    last_change: &mut Option<Instant>,
) {
    log_watcher(&format!(
        "{} file(s) changed, reindexing...",
        changed.len()
    ));
    reindex_project(project_path);
    trigger_doc_update(project_path);
    file_cache.refresh();
    changed.clear();
    *last_change = None;
}

/// Poll for file events from the watcher
fn poll_events(
    watcher: &mut FileWatcher,
    changed_paths: &mut Vec<PathBuf>,
    last_change: &mut Option<Instant>,
) {
    let timeout =
        Duration::from_millis(POLL_TIMEOUT_MS);
    match watcher.wait(timeout) {
        Ok(Some(event)) => {
            changed_paths.extend(event.paths);
            *last_change = Some(Instant::now());
        }
        Ok(None) => {}
        Err(err) => {
            log_watcher(&format!("Error: {}", err));
        }
    }
}

/// Run an incremental reindex of the project
fn reindex_project(project_path: &Path) {
    let manager = IndexManager::new()
        .with_persistence()
        .with_semantic_analysis()
        .with_reference_extraction();

    match manager.index_project(project_path) {
        Ok(result) => {
            log_watcher(&format!(
                "Reindexed: {} symbols",
                result.symbols.len()
            ));
        }
        Err(err) => {
            log_watcher(&format!(
                "Reindex error: {}",
                err
            ));
        }
    }
}

/// Send StartDocGen request to daemon
fn trigger_doc_update(project_path: &Path) {
    let client = DaemonClient::new();
    let path_str =
        project_path.to_string_lossy().to_string();

    if client.ping().is_err() {
        return;
    }

    match client.start_doc_gen(path_str, false) {
        Ok(_) => {
            log_watcher("Doc update triggered");
        }
        Err(err) => {
            log_watcher(&format!(
                "Doc update failed: {}",
                err
            ));
        }
    }
}

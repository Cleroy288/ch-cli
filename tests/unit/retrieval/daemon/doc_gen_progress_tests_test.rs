//! Tests for DocGenProgress and doc_gen_cancel flag
//!
//! Unit tests for progress tracking and cancellation.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use ch_cli::retrieval::daemon::server::DocGenProgress;

#[test]
fn test_doc_gen_progress_default() {
    let progress = DocGenProgress::default();

    assert_eq!(progress.total, 0);
    assert_eq!(progress.completed, 0);
    assert_eq!(progress.failed, 0);
    assert!(!progress.is_running);
}

#[test]
fn test_doc_gen_progress_update() {
    let mut progress = DocGenProgress::default();

    progress.total = 100;
    progress.completed = 50;
    progress.failed = 2;
    progress.is_running = true;

    assert_eq!(progress.total, 100);
    assert_eq!(progress.completed, 50);
    assert_eq!(progress.failed, 2);
    assert!(progress.is_running);
}

#[test]
fn test_doc_gen_progress_clone() {
    let mut progress = DocGenProgress::default();
    progress.total = 10;
    progress.completed = 5;
    progress.is_running = true;

    let cloned = progress.clone();

    assert_eq!(cloned.total, 10);
    assert_eq!(cloned.completed, 5);
    assert!(cloned.is_running);
}

#[test]
fn test_doc_gen_cancel_flag_default() {
    let cancel = Arc::new(AtomicBool::new(false));

    assert!(!cancel.load(Ordering::Relaxed));
}

#[test]
fn test_doc_gen_cancel_flag_set() {
    let cancel = Arc::new(AtomicBool::new(false));
    let cancel_clone = cancel.clone();

    // simulate shutdown setting the flag
    cancel.store(true, Ordering::SeqCst);

    // background thread should see it
    assert!(cancel_clone.load(Ordering::Relaxed));
}

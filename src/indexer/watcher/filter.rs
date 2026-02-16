//! Event filtering for file watcher.

use notify::Event;

use crate::indexer::watcher::event::{ChangeKind, FileChangeEvent};

/// Filter event paths to only include Rust files
pub fn filter_rust_paths(event: Event) -> Option<FileChangeEvent> {
	let rust_paths: Vec<_> = event
		.paths
		.into_iter()
		.filter(|path| {
			path.extension().is_some_and(|ext| ext == "rs")
		})
		.collect();

	if rust_paths.is_empty() {
		None
	} else {
		Some(FileChangeEvent {
			paths: rust_paths,
			kind: ChangeKind::from(&event.kind),
		})
	}
}

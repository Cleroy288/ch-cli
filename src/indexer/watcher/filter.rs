use notify::Event;

use crate::indexer::crawler::Language;
use crate::indexer::watcher::event::{
	ChangeKind, FileChangeEvent,
};

pub fn filter_supported_paths(
	event: Event,
) -> Option<FileChangeEvent> {
	let paths: Vec<_> = event
		.paths
		.into_iter()
		.filter(|p| Language::from_path(p).is_some())
		.collect();

	if paths.is_empty() {
		return None;
	}
	Some(FileChangeEvent {
		paths,
		kind: ChangeKind::from(&event.kind),
	})
}

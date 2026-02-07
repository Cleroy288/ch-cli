//! File system watcher for code changes.

use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};

use crate::indexer::watcher::error::WatcherResult;

/// File system watcher for code changes
pub struct FileWatcher {
	pub(super) watcher: RecommendedWatcher,
	pub(super) receiver: Receiver<Result<Event, notify::Error>>,
	pub(super) root: PathBuf,
}

impl FileWatcher {
	/// Create a new file watcher for the given root directory
	pub fn new<P: AsRef<Path>>(root: P) -> WatcherResult<Self> {
		let (tx, rx) = channel();

		let config = Config::default()
			.with_poll_interval(Duration::from_secs(2));

		let watcher = RecommendedWatcher::new(
			move |res| {
				let _ = tx.send(res);
			},
			config,
		)?;

		Ok(Self {
			watcher,
			receiver: rx,
			root: root.as_ref().to_path_buf(),
		})
	}

	/// Start watching the root directory
	pub fn start(&mut self) -> WatcherResult<()> {
		self.watcher
			.watch(&self.root, RecursiveMode::Recursive)?;
		Ok(())
	}

	/// Stop watching
	pub fn stop(&mut self) -> WatcherResult<()> {
		self.watcher.unwatch(&self.root)?;
		Ok(())
	}

	/// Get the root directory being watched
	pub fn root(&self) -> &Path {
		&self.root
	}
}

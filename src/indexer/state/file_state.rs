//! FileState implementation for tracking individual file metadata.

use std::path::Path;
use std::time::SystemTime;

use super::types::FileState;

impl FileState {
	/// Create a new FileState from a file path
	pub fn from_path(
		path: &Path,
		root: &Path,
	) -> std::io::Result<Self> {
		let metadata = std::fs::metadata(path)?;

		// mtime: modification time as seconds since epoch
		let mtime = metadata
			.modified()?
			.duration_since(SystemTime::UNIX_EPOCH)
			.map(|dur| dur.as_secs())
			.unwrap_or(0);

		let relative_path = path
			.strip_prefix(root)
			.unwrap_or(path)
			.to_path_buf();

		Ok(Self {
			path: relative_path,
			mtime,
			size: metadata.len(),
			symbol_count: 0,
		})
	}

	/// Check if the file has changed compared to disk
	pub fn has_changed(&self, root: &Path) -> bool {
		let full_path = root.join(&self.path);

		match std::fs::metadata(&full_path) {
			Ok(metadata) => {
				let mod_time = metadata
					.modified()
					.ok()
					.and_then(|time| {
						time.duration_since(
							SystemTime::UNIX_EPOCH,
						)
						.ok()
					})
					.map(|dur| dur.as_secs())
					.unwrap_or(0);

				let cur_size = metadata.len();
				mod_time != self.mtime
					|| cur_size != self.size
			}
			Err(_) => true,
		}
	}
}

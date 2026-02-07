//! FileState implementation for tracking individual file metadata.

use std::fs;
use std::io;
use std::path::Path;
use std::time::SystemTime;

use super::types::FileState;

impl FileState {
	/// Create a new FileState from a file path
	pub fn from_path(path: &Path, root: &Path) -> io::Result<Self> {
		// metadata: file system metadata for the given path
		let metadata = fs::metadata(path)?;

		// mtime: last modification time as seconds since UNIX epoch
		let mtime = metadata
			.modified()?
			.duration_since(SystemTime::UNIX_EPOCH)
			.map(|d| d.as_secs())
			.unwrap_or(0);

		// relative_path: path relative to the project root
		let relative_path = path.strip_prefix(root).unwrap_or(path).to_path_buf();

		Ok(Self {
			path: relative_path,
			mtime,
			size: metadata.len(),
			symbol_count: 0,
		})
	}

	/// Check if the file has changed compared to current disk state
	pub fn has_changed(&self, root: &Path) -> bool {
		// full_path: absolute path to the file
		let full_path = root.join(&self.path);

		match fs::metadata(&full_path) {
			Ok(metadata) => {
				// current_mtime: current modification time from disk
				let modified_time = metadata
					.modified()
					.ok()
					.and_then(|t| {
						t.duration_since(SystemTime::UNIX_EPOCH).ok()
					})
					.map(|d| d.as_secs())
					.unwrap_or(0);

				// current_size: current file size from disk
				let current_size = metadata.len();

				modified_time != self.mtime || current_size != self.size
			}
			Err(_) => true, // File doesn't exist or can't be read
		}
	}
}

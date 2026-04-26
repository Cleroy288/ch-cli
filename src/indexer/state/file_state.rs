use std::fs::Metadata;
use std::path::Path;
use std::time::SystemTime;

use super::types::FileState;

/// Extract modification time as seconds since epoch.
fn mtime(meta: &Metadata) -> u64 {
	meta.modified()
		.ok()
		.and_then(|t| {
			t.duration_since(SystemTime::UNIX_EPOCH).ok()
		})
		.map(|dur| dur.as_secs())
		.unwrap_or(0)
}

impl FileState {
	pub fn from_path(
		path: &Path,
		root: &Path,
	) -> std::io::Result<Self> {
		let metadata = std::fs::metadata(path)?;
		let mod_time = mtime(&metadata);

		let relative_path = path
			.strip_prefix(root)
			.unwrap_or(path)
			.to_path_buf();

		Ok(Self {
			path: relative_path,
			mtime: mod_time,
			size: metadata.len(),
			symbol_count: 0,
		})
	}

	pub fn has_changed(&self, root: &Path) -> bool {
		let full_path = root.join(&self.path);
		let Ok(meta) = std::fs::metadata(&full_path)
		else {
			return true;
		};
		mtime(&meta) != self.mtime
			|| meta.len() != self.size
	}
}

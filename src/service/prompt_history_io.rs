use std::path::Path;

use crate::domain::data_paths_dirs;
use crate::domain::prompt_history::PromptHistory;

/// Load prompt history from disk, or empty.
pub fn load(root: &Path) -> PromptHistory {
	let path = data_paths_dirs::history_file(root);
	std::fs::read_to_string(&path)
		.ok()
		.and_then(|s| {
			serde_json::from_str(&s).ok()
		})
		.unwrap_or_else(PromptHistory::empty)
}

/// Best-effort write to disk.
pub fn save(
	history: &PromptHistory,
	root: &Path,
) {
	let path = data_paths_dirs::history_file(root);
	if let Some(dir) = path.parent() {
		let _ = std::fs::create_dir_all(dir);
	}
	let json = serde_json::to_string(history)
		.unwrap_or_default();
	let _ = std::fs::write(&path, json);
}

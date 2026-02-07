//! Path and module resolution utilities.

use std::path::Path;

/// Get module path from file path.
///
/// Converts "src/retrieval/docgen/entry.rs" to
/// "retrieval::docgen::entry".
#[doc(hidden)]
pub fn get_module_path(file_path: &Path) -> String {
	let path_str = file_path.to_string_lossy().to_string();
	let path_str = path_str.replace('\\', "/");

	let path_str =
		if let Some(stripped) = path_str.strip_prefix("src/") {
			stripped.to_string()
		} else {
			path_str
		};

	let path_str =
		if let Some(stripped) = path_str.strip_suffix(".rs") {
			stripped.to_string()
		} else {
			path_str
		};

	let path_str =
		if let Some(stripped) = path_str.strip_suffix("/mod")
		{
			stripped.to_string()
		} else {
			path_str
		};

	path_str.replace('/', "::")
}


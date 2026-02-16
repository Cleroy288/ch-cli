//! Utility for path prefix stripping.
//!
//! Used by doc browser to convert absolute paths
//! to relative paths for display.

/// Convert absolute path to relative by stripping
/// the project prefix. Returns original if no match.
pub fn strip_project_prefix(
	abs_path: &str,
	project: &str,
) -> String {
	let trimmed = project.trim_end_matches('/');
	abs_path
		.strip_prefix(trimmed)
		.map(|rest| rest.trim_start_matches('/'))
		.unwrap_or(abs_path)
		.to_string()
}

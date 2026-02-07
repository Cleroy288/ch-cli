//! JSON Extraction Helpers
//!
//! Extracts symbols, filters, and hints from JSON responses.

use crate::retrieval::daemon::protocol::FileFilter;

/// Extract symbols array from JSON
pub(super) fn extract_json_symbols(
	json: &serde_json::Value,
) -> Vec<String> {
	json.get("symbols")
		.and_then(|v| v.as_array())
		.map(|arr| {
			arr.iter()
				.filter_map(|v| {
					v.as_str().map(String::from)
				})
				.collect()
		})
		.unwrap_or_default()
}

/// Extract file filters from JSON
pub(super) fn extract_json_filters(
	json: &serde_json::Value,
) -> Vec<FileFilter> {
	json.get("file_patterns")
		.and_then(|v| v.as_array())
		.map(|arr| {
			arr.iter()
				.filter_map(|v| {
					v.as_str().map(|s| FileFilter {
						pattern: s.to_string(),
						include: true,
					})
				})
				.collect()
		})
		.unwrap_or_default()
}

/// Extract hints from JSON
pub(super) fn extract_json_hints(
	json: &serde_json::Value,
) -> Vec<String> {
	json.get("hints")
		.and_then(|v| v.as_array())
		.map(|arr| {
			arr.iter()
				.filter_map(|v| {
					v.as_str().map(String::from)
				})
				.collect()
		})
		.unwrap_or_default()
}

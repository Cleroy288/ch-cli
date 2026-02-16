//! JSON Extraction Helpers
//!
//! Extracts symbols, filters, and hints from JSON.

use crate::retrieval::daemon::protocol::FileFilter;

/// Extract symbols array from JSON
pub(super) fn extract_json_symbols(
	json: &serde_json::Value,
) -> Vec<String> {
	json.get("symbols")
		.and_then(|val| val.as_array())
		.map(|arr| {
			arr.iter()
				.filter_map(|val| {
					val.as_str().map(String::from)
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
		.and_then(|val| val.as_array())
		.map(|arr| {
			arr.iter()
				.filter_map(|val| {
					val.as_str().map(|str_val| {
						FileFilter {
							pattern: str_val.to_string(),
							include: true,
						}
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
		.and_then(|val| val.as_array())
		.map(|arr| {
			arr.iter()
				.filter_map(|val| {
					val.as_str().map(String::from)
				})
				.collect()
		})
		.unwrap_or_default()
}

//! Doc browser state for documentation browsing.
//!
//! Holds loaded documentation entries and supports
//! query-based filtering for the doc browser picker.

/// A documentation entry for the doc browser.
///
/// Represents a single documented symbol with its
/// metadata for display in the picker.
pub struct DocBrowserEntry {
	/// Symbol name
	pub name: String,
	/// Symbol kind (function, struct, etc.)
	pub kind: String,
	/// File path for display (relative)
	pub rel_path: String,
	/// Source line number
	pub line: usize,
	/// LLM-generated documentation
	pub llm_doc: Option<String>,
}

/// Documentation browser for picker.
///
/// Holds all loaded doc entries and supports
/// filtering by query string.
pub struct DocBrowser {
	/// All loaded entries sorted by path+name
	entries: Vec<DocBrowserEntry>,
}

impl DocBrowser {
	/// Create a new DocBrowser with sorted entries
	pub fn new(
		mut entries: Vec<DocBrowserEntry>,
	) -> Self {
		entries.sort_by(|a, b| {
			a.rel_path
				.cmp(&b.rel_path)
				.then(a.name.cmp(&b.name))
		});
		Self { entries }
	}

	/// Get entries matching the query
	pub fn current_items(
		&self,
		query: &str,
	) -> Vec<&DocBrowserEntry> {
		if query.is_empty() {
			return self.entries.iter().collect();
		}
		let lower = query.to_lowercase();
		self.entries
			.iter()
			.filter(|entry| {
				entry_matches_query(entry, &lower)
			})
			.collect()
	}

	/// Total number of entries
	pub fn len(&self) -> usize {
		self.entries.len()
	}

	/// Check if browser has no entries
	pub fn is_empty(&self) -> bool {
		self.entries.is_empty()
	}

	/// Get entry at index from filtered results
	pub fn entry_at(
		&self,
		query: &str,
		idx: usize,
	) -> Option<&DocBrowserEntry> {
		self.current_items(query).get(idx).copied()
	}
}

/// Check if entry matches a lowercase query
fn entry_matches_query(
	entry: &DocBrowserEntry,
	lower_query: &str,
) -> bool {
	entry.name.to_lowercase().contains(lower_query)
		|| entry
			.rel_path
			.to_lowercase()
			.contains(lower_query)
}

//! DocChunk struct representing a parsed documentation section

/// A parsed documentation chunk from a markdown file
#[derive(Debug, Clone)]
pub struct DocChunk {
	/// section title (header text)
	pub title: String,
	/// full content of the section
	pub content: String,
	/// header level (1, 2, 3, etc.)
	pub level: usize,
	/// line number where section starts
	pub line: usize,
	/// parent section title (if nested)
	pub parent: Option<String>,
}

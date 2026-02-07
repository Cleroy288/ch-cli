//! High-level content type classification for triple pipeline indexing.

use std::fmt;

use serde::{Deserialize, Serialize};

use super::document_type::DocumentType;

/// High-level content classification for triple pipeline indexing.
/// Used to route symbols to separate Code, Doc, or Notes indexes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContentType {
	/// Code files (SourceCode + Test)
	Code,
	/// Documentation files (doc/*.md)
	Doc,
	/// Notes and benchmarks (notes/*.md, notes/benchmarks/*.md)
	Notes,
}

impl ContentType {
	/// Classify from DocumentType
	pub fn from_document_type(doc_type: &DocumentType) -> Self {
		match doc_type {
			DocumentType::SourceCode | DocumentType::Test => Self::Code,
			DocumentType::Documentation => Self::Doc,
			DocumentType::Notes | DocumentType::Benchmark => Self::Notes,
		}
	}

	/// Classify from file path
	pub fn from_path(path: &std::path::Path) -> Self {
		Self::from_document_type(&DocumentType::from_path(path))
	}

	/// Get all variants for iteration
	pub fn all() -> [Self; 3] {
		[Self::Code, Self::Doc, Self::Notes]
	}

	/// Get display name for this content type
	pub fn name(&self) -> &'static str {
		match self {
			Self::Code => "code",
			Self::Doc => "docs",
			Self::Notes => "notes",
		}
	}
}

impl fmt::Display for ContentType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name())
	}
}

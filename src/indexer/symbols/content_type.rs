use std::fmt;

use serde::{Deserialize, Serialize};

use super::document_type::DocumentType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContentType {
	Code,
	Doc,
	Notes,
}

impl ContentType {
	pub fn from_document_type(doc_type: &DocumentType) -> Self {
		match doc_type {
			DocumentType::SourceCode | DocumentType::Test => Self::Code,
			DocumentType::Documentation => Self::Doc,
			DocumentType::Notes | DocumentType::Benchmark => Self::Notes,
		}
	}

	pub fn from_path(path: &std::path::Path) -> Self {
		Self::from_document_type(&DocumentType::from_path(path))
	}

	pub fn all() -> [Self; 3] {
		[Self::Code, Self::Doc, Self::Notes]
	}

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

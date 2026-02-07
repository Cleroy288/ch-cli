//! Code location representation for source code positions.

use std::fmt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Represents a location in source code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeLocation {
	/// Path to the source file
	pub file: PathBuf,
	/// Line number (1-indexed for display)
	pub line: usize,
	/// Column number (1-indexed for display)
	pub column: usize,
	/// Byte offset from start of file
	pub byte_offset: usize,
	/// Byte length of the symbol
	pub byte_length: usize,
}

impl CodeLocation {
	/// Create a new CodeLocation
	pub fn new(
		file: PathBuf,
		line: usize,
		column: usize,
		byte_offset: usize,
		byte_length: usize,
	) -> Self {
		Self {
			file,
			line,
			column,
			byte_offset,
			byte_length,
		}
	}
}

impl fmt::Display for CodeLocation {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}:{}:{}", self.file.display(), self.line, self.column)
	}
}

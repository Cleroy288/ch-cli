//! Code location representation for source code positions.

use std::fmt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Byte range in a source file
#[derive(
	Debug, Clone, Copy, PartialEq, Eq,
	Serialize, Deserialize,
)]
pub struct ByteSpan {
	/// Byte offset from start of file
	pub offset: usize,
	/// Byte length of the symbol
	pub length: usize,
}

impl ByteSpan {
	/// Zero-length span (no byte info)
	pub const ZERO: Self =
		Self { offset: 0, length: 0 };
}

/// Represents a location in source code.
#[derive(
	Debug, Clone, PartialEq, Eq,
	Serialize, Deserialize,
)]
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
		bytes: ByteSpan,
	) -> Self {
		Self {
			file,
			line,
			column,
			byte_offset: bytes.offset,
			byte_length: bytes.length,
		}
	}
}

impl fmt::Display for CodeLocation {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		write!(
			f,
			"{}:{}:{}",
			self.file.display(),
			self.line,
			self.column,
		)
	}
}

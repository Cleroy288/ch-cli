use std::fmt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(
	Debug, Clone, Copy, PartialEq, Eq,
	Serialize, Deserialize,
)]
pub struct ByteSpan {
	pub offset: usize,
	pub length: usize,
}

impl ByteSpan {
	pub const ZERO: Self =
		Self { offset: 0, length: 0 };
}

#[derive(
	Debug, Clone, PartialEq, Eq,
	Serialize, Deserialize,
)]
pub struct CodeLocation {
	pub file: PathBuf,
	/// 1-indexed
	pub line: usize,
	/// 1-indexed
	pub column: usize,
	pub byte_offset: usize,
	pub byte_length: usize,
}

impl CodeLocation {
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

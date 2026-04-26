use std::path::Path;

use crate::indexer::Symbol;

use super::rust_parser::RustParser;
use super::types::{ExtractedReference, Result};

impl RustParser {
	pub fn parse_file<P: AsRef<Path>>(
		&mut self,
		path: P,
	) -> Result<Vec<Symbol>> {
		let path = path.as_ref();
		let source = std::fs::read_to_string(path)?;
		self.parse_source(&source, path)
	}

	pub fn parse_file_references<P: AsRef<Path>>(
		&mut self,
		path: P,
	) -> Result<Vec<ExtractedReference>> {
		let path = path.as_ref();
		let source = std::fs::read_to_string(path)?;
		self.parse_references(&source, path)
	}
}

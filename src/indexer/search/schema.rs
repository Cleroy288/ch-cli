//! Schema definition and field handles for the search index.

use tantivy::schema::*;

use crate::indexer::search::error::{SearchError, SearchResult};

pub use super::schema_builder::build_schema;

/// Field handles for the search schema
#[derive(Clone)]
pub struct SchemaFields {
	pub symbol_name: Field,   // primary search field (tokenized)
	pub symbol_kind: Field,   // symbol kind (string, not tokenized)
	pub file_path: Field,     // file path (string, not tokenized)
	pub line: Field,          // line number (u64)
	pub column: Field,        // column number (u64)
	pub visibility: Field,    // visibility modifier (string)
	pub signature: Field,     // function/method signature (tokenized)
	pub fqn: Field,           // fully qualified name (tokenized)
	pub parent: Field,        // parent symbol name (string)
	pub content: Field,       // documentation/content (tokenized)
	pub document_type: Field, // document type for boost scoring
}

impl SchemaFields {
	/// Extract all field handles from the schema
	pub fn from_schema(schema: &Schema) -> SearchResult<Self> {
		let err_fn = |name: &str| {
			SearchError::FieldNotFound(name.to_string())
		};

		Ok(Self {
			symbol_name: schema
				.get_field("symbol_name")
				.map_err(|_| err_fn("symbol_name"))?,
			symbol_kind: schema
				.get_field("symbol_kind")
				.map_err(|_| err_fn("symbol_kind"))?,
			file_path: schema
				.get_field("file_path")
				.map_err(|_| err_fn("file_path"))?,
			line: schema
				.get_field("line")
				.map_err(|_| err_fn("line"))?,
			column: schema
				.get_field("column")
				.map_err(|_| err_fn("column"))?,
			visibility: schema
				.get_field("visibility")
				.map_err(|_| err_fn("visibility"))?,
			signature: schema
				.get_field("signature")
				.map_err(|_| err_fn("signature"))?,
			fqn: schema.get_field("fqn").map_err(|_| err_fn("fqn"))?,
			parent: schema
				.get_field("parent")
				.map_err(|_| err_fn("parent"))?,
			content: schema
				.get_field("content")
				.map_err(|_| err_fn("content"))?,
			document_type: schema
				.get_field("document_type")
				.map_err(|_| err_fn("document_type"))?,
		})
	}
}

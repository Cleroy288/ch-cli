//! Schema definition and field handles for the search index.

use tantivy::schema::*;

use crate::indexer::search::error::{SearchError, SearchResult};

pub use super::schema_builder::build_schema;

/// Field handles for the search schema
#[derive(Clone)]
pub struct SchemaFields {
	pub symbol_name: Field,   // primary search field
	pub symbol_kind: Field,   // symbol kind (string)
	pub file_path: Field,     // file path (string)
	pub line: Field,          // line number (u64)
	pub column: Field,        // column number (u64)
	pub visibility: Field,    // visibility modifier
	pub signature: Field,     // function signature
	pub fqn: Field,           // fully qualified name
	pub parent: Field,        // parent symbol name
	pub content: Field,       // documentation content
	pub document_type: Field, // document type for boost
}

impl SchemaFields {
	/// Extract all field handles from the schema
	pub fn from_schema(
		schema: &Schema,
	) -> SearchResult<Self> {
		Ok(Self {
			symbol_name: get(schema, "symbol_name")?,
			symbol_kind: get(schema, "symbol_kind")?,
			file_path: get(schema, "file_path")?,
			line: get(schema, "line")?,
			column: get(schema, "column")?,
			visibility: get(schema, "visibility")?,
			signature: get(schema, "signature")?,
			fqn: get(schema, "fqn")?,
			parent: get(schema, "parent")?,
			content: get(schema, "content")?,
			document_type: get(schema, "document_type")?,
		})
	}
}

/// Get a field from schema, returning SearchError on failure
fn get(
	schema: &Schema,
	name: &str,
) -> SearchResult<Field> {
	schema
		.get_field(name)
		.map_err(|_| SearchError::FieldNotFound(name.to_string()))
}

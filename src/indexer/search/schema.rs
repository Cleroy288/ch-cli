use tantivy::schema::{Field, Schema};

use crate::indexer::search::error::{SearchError, SearchResult};

pub use super::schema_builder::build_schema;

#[derive(Clone)]
pub struct SchemaFields {
	pub symbol_name: Field,
	pub symbol_kind: Field,
	pub file_path: Field,
	pub line: Field,
	pub column: Field,
	pub visibility: Field,
	pub signature: Field,
	pub fqn: Field,
	pub parent: Field,
	pub content: Field,
	pub document_type: Field,
}

impl SchemaFields {
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

fn get(
	schema: &Schema,
	name: &str,
) -> SearchResult<Field> {
	schema
		.get_field(name)
		.map_err(|_| SearchError::FieldNotFound(name.to_string()))
}

//! Schema builder function for Tantivy index.

use tantivy::schema::*;

/// Build the Tantivy schema for symbol indexing
pub fn build_schema() -> Schema {
	let mut schema_builder = Schema::builder(); // schema builder instance

	// Primary search field - tokenized for full-text search
	schema_builder.add_text_field("symbol_name", TEXT | STORED);

	// Filterable fields - not tokenized
	schema_builder.add_text_field("symbol_kind", STRING | STORED);
	schema_builder.add_text_field("file_path", STRING | STORED);
	schema_builder.add_text_field("visibility", STRING | STORED);

	// Numeric fields for location
	schema_builder.add_u64_field("line", STORED | INDEXED);
	schema_builder.add_u64_field("column", STORED | INDEXED);

	// Additional metadata
	schema_builder.add_text_field("signature", TEXT | STORED);
	schema_builder.add_text_field("fqn", TEXT | STORED);
	schema_builder.add_text_field("parent", STRING | STORED);

	// Content field for full-text search on documentation
	schema_builder.add_text_field("content", TEXT | STORED);

	// Document type field for boost scoring
	schema_builder.add_text_field("document_type", STRING | STORED);

	schema_builder.build()
}

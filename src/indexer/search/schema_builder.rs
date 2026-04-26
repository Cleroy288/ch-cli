use tantivy::schema::{
	Schema, INDEXED, STORED, STRING, TEXT,
};

///
/// Fields: symbol_name (TEXT), symbol_kind/file_path/
/// visibility/parent/document_type (STRING),
/// line/column (u64), signature/fqn/content (TEXT).
pub fn build_schema() -> Schema {
	let mut b = Schema::builder();

	b.add_text_field("symbol_name", TEXT | STORED);
	b.add_text_field("symbol_kind", STRING | STORED);
	b.add_text_field("file_path", STRING | STORED);
	b.add_text_field("visibility", STRING | STORED);
	b.add_u64_field("line", STORED | INDEXED);
	b.add_u64_field("column", STORED | INDEXED);
	b.add_text_field("signature", TEXT | STORED);
	b.add_text_field("fqn", TEXT | STORED);
	b.add_text_field("parent", STRING | STORED);
	b.add_text_field("content", TEXT | STORED);
	b.add_text_field("document_type", STRING | STORED);

	b.build()
}

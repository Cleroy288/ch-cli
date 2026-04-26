use tantivy::schema::{
	Field, Schema, INDEXED, STORED, STRING, TEXT,
};

use crate::domain::errors::memory::{
	MemoryError, MemoryResult,
};

const FIELD_ID: &str = "id";
const FIELD_SESSION: &str = "session_id";
const FIELD_TIMESTAMP: &str = "timestamp";
const FIELD_INPUT: &str = "input_text";
const FIELD_RESPONSE: &str = "response_text";
const FIELD_RTYPE: &str = "response_type";
const FIELD_FILES: &str = "files";

#[derive(Clone)]
pub struct MemoryFields {
	pub id: Field,
	pub session_id: Field,
	pub timestamp: Field,
	pub input_text: Field,
	pub response_text: Field,
	pub response_type: Field,
	/// Space-separated file paths
	pub files: Field,
}

pub fn build_memory_schema() -> Schema {
	let mut builder = Schema::builder();

	builder.add_text_field(FIELD_ID, STRING | STORED);
	builder.add_text_field(
		FIELD_SESSION, STRING | STORED,
	);
	builder.add_u64_field(
		FIELD_TIMESTAMP, STORED | INDEXED,
	);
	builder.add_text_field(
		FIELD_INPUT, TEXT | STORED,
	);
	builder.add_text_field(
		FIELD_RESPONSE, TEXT | STORED,
	);
	builder.add_text_field(
		FIELD_RTYPE, STRING | STORED,
	);
	builder.add_text_field(
		FIELD_FILES, TEXT | STORED,
	);

	builder.build()
}

impl MemoryFields {
	pub fn from_schema(
		schema: &Schema,
	) -> MemoryResult<Self> {
		Ok(Self {
			id: get(schema, FIELD_ID)?,
			session_id: get(
				schema, FIELD_SESSION,
			)?,
			timestamp: get(
				schema, FIELD_TIMESTAMP,
			)?,
			input_text: get(
				schema, FIELD_INPUT,
			)?,
			response_text: get(
				schema, FIELD_RESPONSE,
			)?,
			response_type: get(
				schema, FIELD_RTYPE,
			)?,
			files: get(schema, FIELD_FILES)?,
		})
	}
}

fn get(
	schema: &Schema,
	name: &str,
) -> MemoryResult<Field> {
	schema.get_field(name).map_err(|_| {
		MemoryError::Search(format!(
			"Field not found: {name}"
		))
	})
}

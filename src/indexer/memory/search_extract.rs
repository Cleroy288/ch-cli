use tantivy::TantivyDocument;

use crate::domain::memory::{
	AiResponse, Interaction, UserInput,
};
use crate::domain::memory_helpers::response_from_label;

use super::search_schema::MemoryFields;
use super::types::MemoryHit;

/// Reconstruct a MemoryHit from a Tantivy doc
pub fn doc_to_hit(
	fields: &MemoryFields,
	doc: &TantivyDocument,
	score: f32,
) -> Option<MemoryHit> {
	let base = extract_base(fields, doc)?;
	let resp =
		extract_response(fields, doc)?;
	Some(MemoryHit {
		interaction: assemble(base, resp),
		score,
	})
}

/// Base fields: (id, session, timestamp, input)
type BaseFields = (String, String, u64, String);

/// Extract base fields from a Tantivy doc
fn extract_base(
	fields: &MemoryFields,
	doc: &TantivyDocument,
) -> Option<BaseFields> {
	use tantivy::schema::Value;
	let id = extract_str(doc, fields.id)?;
	let session =
		extract_str(doc, fields.session_id)?;
	let timestamp = doc
		.get_first(fields.timestamp)?
		.as_u64()?;
	let input = extract_str(
		doc, fields.input_text,
	)?;
	Some((id, session, timestamp, input))
}

/// Response + files from a Tantivy doc
type ResponseData = (AiResponse, Vec<String>);

/// Extract response + files from a Tantivy doc
fn extract_response(
	fields: &MemoryFields,
	doc: &TantivyDocument,
) -> Option<ResponseData> {
	let text = extract_str(
		doc, fields.response_text,
	)?;
	let rtype = extract_str(
		doc, fields.response_type,
	)
	.unwrap_or_default();
	let files_str =
		extract_str(doc, fields.files)
			.unwrap_or_default();
	let files = if files_str.is_empty() {
		Vec::new()
	} else {
		files_str
			.split_whitespace()
			.map(String::from)
			.collect()
	};
	let response =
		response_from_label(&rtype, text);
	Some((response, files))
}

/// Assemble Interaction from base + response
fn assemble(
	base: BaseFields,
	resp: ResponseData,
) -> Interaction {
	let (id, session, time, input_text) = base;
	let (response, files) = resp;
	Interaction {
		id,
		timestamp: time,
		session_id: session,
		input: UserInput {
			text: input_text,
			command: None,
			files,
		},
		response,
	}
}

/// Extract a string field from a Tantivy doc
fn extract_str(
	doc: &TantivyDocument,
	field: tantivy::schema::Field,
) -> Option<String> {
	use tantivy::schema::Value;
	doc.get_first(field)?
		.as_str()
		.map(String::from)
}

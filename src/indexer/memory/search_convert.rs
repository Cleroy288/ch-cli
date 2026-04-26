use tantivy::TantivyDocument;

use crate::domain::memory::{
	AiResponse, Interaction,
};
use crate::domain::memory_helpers;

use super::search_schema::MemoryFields;

pub fn interaction_to_doc(
	fields: &MemoryFields,
	interaction: &Interaction,
) -> TantivyDocument {
	let mut doc = TantivyDocument::new();
	doc.add_text(fields.id, &interaction.id);
	doc.add_text(
		fields.session_id,
		&interaction.session_id,
	);
	doc.add_u64(
		fields.timestamp, interaction.timestamp,
	);
	doc.add_text(
		fields.input_text,
		&interaction.input.text,
	);
	add_response_fields(
		&mut doc, fields, interaction,
	);
	doc
}

fn add_response_fields(
	doc: &mut TantivyDocument,
	fields: &MemoryFields,
	interaction: &Interaction,
) {
	let resp = flatten_response(
		&interaction.response,
	);
	doc.add_text(fields.response_text, &resp);
	let rtype =
		memory_helpers::response_type_label(
			&interaction.response,
		);
	doc.add_text(fields.response_type, rtype);
	let files_str =
		collect_files(interaction);
	doc.add_text(fields.files, &files_str);
}

pub fn flatten_response(
	response: &AiResponse,
) -> String {
	match response {
		AiResponse::Answer { text }
		| AiResponse::Question { text }
		| AiResponse::CodeChange {
			text, ..
		} => text.clone(),
	}
}

pub fn collect_files(
	interaction: &Interaction,
) -> String {
	interaction.input.files.join(" ")
}

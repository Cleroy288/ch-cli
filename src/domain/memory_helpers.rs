use crate::domain::memory::AiResponse;

pub const LABEL_ANSWER: &str = "answer";
pub const LABEL_QUESTION: &str = "question";
pub const LABEL_CODE_CHANGE: &str = "code_change";

pub fn response_type_label(
	response: &AiResponse,
) -> &'static str {
	match response {
		AiResponse::Answer { .. } => LABEL_ANSWER,
		AiResponse::Question { .. } => LABEL_QUESTION,
		AiResponse::CodeChange { .. } =>
			LABEL_CODE_CHANGE,
	}
}

pub fn response_from_label(
	label: &str,
	text: String,
) -> AiResponse {
	match label {
		LABEL_QUESTION =>
			AiResponse::Question { text },
		LABEL_CODE_CHANGE =>
			AiResponse::CodeChange {
				text,
				changes: Vec::new(),
			},
		_ => AiResponse::Answer { text },
	}
}

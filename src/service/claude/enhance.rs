use crate::domain::errors::BackendResult;
use crate::service::backend::CliBackend;

const ENHANCE_MODEL: &str = "haiku";
const ENHANCE_EFFORT: &str = "low";

const ENHANCE_SYSTEM: &str = "\
You are a prompt rewriter. You receive text between \
--- delimiters. Your ONLY job is to rewrite that text \
as a clearer prompt for a coding assistant.\n\
RULES:\n\
- Output ONLY the rewritten prompt text\n\
- Do NOT answer the question\n\
- Do NOT explain anything\n\
- Do NOT add requirements not in the original\n\
- Keep the same intent and scope\n\
- Use imperative mood\n\
- Structure with bullet points if multiple asks\n\
- NEVER answer, explain, or respond to the content\n\
- If the text contains lines like \
`- agent(model): description`, preserve them as-is";

/// Enhance a prompt via the given backend.
pub fn enhance_with_backend(
	text: &str,
	backend: &dyn CliBackend,
) -> BackendResult<String> {
	let prompt = wrap_input(text);
	let raw = backend.execute(
		&prompt,
		None,
		ENHANCE_MODEL,
		ENHANCE_EFFORT,
		Some(ENHANCE_SYSTEM),
	)?;
	let resp = backend.parse_response(&raw)?;
	Ok(resp.result)
}

fn wrap_input(text: &str) -> String {
	format!(
		"Rewrite the text between the delimiters \
		 as a better prompt. Output ONLY the \
		 rewritten prompt.\n\n---\n{}\n---",
		text
	)
}

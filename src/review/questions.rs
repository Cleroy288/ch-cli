/// Per-block question handling for inline blocks.

use crate::domain::review::BlockIdx;

use super::state::InlineBlocks;

const ANSWER_SUFFIX: &str =
	"\nAnswer concisely about the code above.";

impl InlineBlocks {
	/// None if question is empty or idx invalid.
	pub fn submit_question(
		&mut self,
		idx: BlockIdx,
	) -> Option<String> {
		let block =
			self.blocks.get(idx.val())?;
		let q = block.question.trim();
		(!q.is_empty()).then(|| {
			question_prompt(
				&block.lang, &block.edited, q,
			)
		})
	}

	pub fn set_answer(
		&mut self,
		idx: BlockIdx,
		text: String,
	) {
		if let Some(b) =
			self.blocks.get_mut(idx.val())
		{
			b.answer = Some(text);
		}
	}
}

pub fn question_prompt(
	lang: &str,
	code: &str,
	question: &str,
) -> String {
	format!(
		"About this {lang} code:\n\
		 ```{lang}\n{code}\n```\n\
		 Question: {question}{ANSWER_SUFFIX}",
	)
}

//! Feedback Action Handlers
//!
//! Implements the concrete execution of each FeedbackAction
//! variant: more context, refined search, and file filtering.

use super::feedback::FeedbackLoop;
use super::feedback_actions::FeedbackAction;
use super::RetrievalOutput;
use crate::retrieval::{RetrievalError, RetrievalResult};

/// Action handlers for the feedback loop
impl FeedbackLoop {
	/// Execute a specific feedback action
	pub(super) fn execute_action(
		&mut self,
		action: FeedbackAction,
	) -> RetrievalResult<()> {
		match action {
			FeedbackAction::MoreContext { symbols } => {
				self.handle_more_context(&symbols)
			}
			FeedbackAction::RefineSearch { query } => {
				self.handle_refine_search(&query)
			}
			FeedbackAction::ExpandRelated { symbols } => {
				let joined = symbols.join(" OR ");
				self.handle_refine_search(&joined)
			}
			FeedbackAction::FilterFiles { patterns } => {
				self.handle_filter_files(&patterns)
			}
			FeedbackAction::Accept => Ok(()),
		}
	}

	/// Handle MoreContext action
	fn handle_more_context(
		&mut self,
		symbols: &[String],
	) -> RetrievalResult<()> {
		let xml =
			self.pipeline().get_more_context(symbols)?;
		let last =
			self.history().last().ok_or_else(|| {
				RetrievalError::Embedding(
					"No previous output".to_string(),
				)
			})?;

		let output = RetrievalOutput {
			query: last.query.clone(),
			search_spec: last.search_spec.clone(),
			xml_output: xml,
			result_count: symbols.len(),
			token_count: last.token_count * 2,
			has_more: false,
		};
		self.push_output(output);
		Ok(())
	}
}

/// Search and filter action handlers
impl FeedbackLoop {
	/// Handle RefineSearch action
	fn handle_refine_search(
		&mut self,
		query: &str,
	) -> RetrievalResult<()> {
		let output =
			self.pipeline().retrieve(query)?;
		self.push_output(output);
		Ok(())
	}

	/// Handle FilterFiles action
	fn handle_filter_files(
		&mut self,
		patterns: &[String],
	) -> RetrievalResult<()> {
		let last =
			self.history().last().ok_or_else(|| {
				RetrievalError::Embedding(
					"No previous output".to_string(),
				)
			})?;

		let filtered_query = format!(
			"{} path:{}",
			last.query,
			patterns.join(" OR path:"),
		);
		let output =
			self.pipeline().retrieve(&filtered_query)?;
		self.push_output(output);
		Ok(())
	}
}

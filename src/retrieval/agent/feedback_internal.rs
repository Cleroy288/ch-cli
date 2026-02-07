//! Internal feedback loop accessors
//!
//! Provides access to internal state for sibling modules.

use super::feedback::FeedbackLoop;
use super::pipeline::RetrievalPipeline;
use super::RetrievalOutput;

/// Internal accessors for sibling modules.
impl FeedbackLoop {
	/// Access the pipeline (for handler methods)
	pub(super) fn pipeline(
		&mut self,
	) -> &mut RetrievalPipeline {
		&mut self.pipeline
	}

	/// Get all outputs in history
	pub fn history(&self) -> &[RetrievalOutput] {
		&self.history
	}

	/// Push an output to history
	#[doc(hidden)]
	pub fn push_output(
		&mut self,
		output: RetrievalOutput,
	) {
		self.history.push(output);
	}

	/// Clear history (for testing)
	#[doc(hidden)]
	pub fn clear_history(&mut self) {
		self.history.clear();
	}
}

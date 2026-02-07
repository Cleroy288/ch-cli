//! Feedback Loop Accessors and Feedback Application
//!
//! Provides read access to feedback loop state and
//! the main apply_feedback entry point.

use super::feedback::FeedbackLoop;
use super::feedback_actions::FeedbackAction;
use super::RetrievalOutput;
use crate::retrieval::{RetrievalError, RetrievalResult};

/// Read accessors for the feedback loop.
impl FeedbackLoop {
	/// Get the current output
	pub fn current_output(
		&self,
	) -> Option<&RetrievalOutput> {
		self.history().last()
	}

	/// Get current iteration count
	pub fn iteration(&self) -> usize {
		self.current_iteration
	}

	/// Check if more iterations are allowed
	pub fn can_continue(&self) -> bool {
		self.current_iteration < self.max_iterations
	}

	/// Apply a feedback action
	pub fn apply_feedback(
		&mut self,
		action: FeedbackAction,
	) -> RetrievalResult<&RetrievalOutput> {
		if self.current_iteration
			>= self.max_iterations
		{
			return Err(RetrievalError::Embedding(
				"Maximum iterations reached"
					.to_string(),
			));
		}

		self.current_iteration += 1;
		self.execute_action(action)?;

		Ok(self.history().last().unwrap())
	}
}


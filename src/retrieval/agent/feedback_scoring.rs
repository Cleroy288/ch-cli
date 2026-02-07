//! Feedback Scoring and Suggestion Logic
//!
//! Extends FeedbackLoop with suggest_action to recommend
//! the next feedback action based on current results.

use super::feedback::FeedbackLoop;
use super::feedback_actions::FeedbackAction;

impl FeedbackLoop {
	/// Suggest next action based on current results
	pub fn suggest_action(&self) -> Option<FeedbackAction> {
		let output = self.history().last()?;

		// If no results, suggest refining search
		if output.result_count == 0 {
			return Some(FeedbackAction::RefineSearch {
				query: format!(
					"{} OR similar",
					output.query,
				),
			});
		}

		// If has_more, suggest getting more context
		let has_symbols = !output
			.search_spec
			.symbol_names
			.is_empty();

		if output.has_more && has_symbols {
			return Some(FeedbackAction::MoreContext {
				symbols: output
					.search_spec
					.symbol_names
					.clone(),
			});
		}

		// Otherwise, accept current results
		Some(FeedbackAction::Accept)
	}
}


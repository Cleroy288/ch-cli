//! Feedback Actions
//!
//! Defines FeedbackAction variants for the feedback loop.

/// Actions that can be taken in the feedback loop
#[derive(Debug, Clone)]
pub enum FeedbackAction {
	/// Get more context for specific symbols
	MoreContext { symbols: Vec<String> },
	/// Refine the search with a new query
	RefineSearch { query: String },
	/// Expand search to include related symbols
	ExpandRelated { symbols: Vec<String> },
	/// Narrow search to specific files
	FilterFiles { patterns: Vec<String> },
	/// Accept current results
	Accept,
}

/// Re-export parse function from parsers module
pub use super::feedback_parsers::parse_feedback_action;


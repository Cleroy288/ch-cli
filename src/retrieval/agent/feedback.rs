//! Feedback Loop for Iterative Refinement
//!
//! Allows the retrieval pipeline to be refined based on initial results.
//! Supports actions like requesting more context, narrowing search, or
//! expanding to related symbols.

use super::pipeline::RetrievalPipeline;
use super::RetrievalOutput;
use crate::retrieval::{RetrievalError, RetrievalResult};

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

/// Feedback loop for iterative retrieval refinement
pub struct FeedbackLoop {
	/// the pipeline instance
	pipeline: RetrievalPipeline,
	/// history of retrieval outputs
	history: Vec<RetrievalOutput>,
	/// max iterations allowed
	max_iterations: usize,
	/// current iteration
	current_iteration: usize,
}

impl FeedbackLoop {
	/// Create a new feedback loop
	pub fn new(pipeline: RetrievalPipeline) -> Self {
		Self {
			pipeline,
			history: Vec::new(),
			max_iterations: 5,
			current_iteration: 0,
		}
	}

	/// Create with custom max iterations
	pub fn with_max_iterations(pipeline: RetrievalPipeline, max: usize) -> Self {
		Self {
			pipeline,
			history: Vec::new(),
			max_iterations: max,
			current_iteration: 0,
		}
	}

	/// Initialize the pipeline
	pub fn initialize(&mut self) -> RetrievalResult<()> {
		self.pipeline.initialize()
	}

	/// Run initial retrieval
	pub fn initial_retrieve(&mut self, query: &str) -> RetrievalResult<&RetrievalOutput> {
		self.current_iteration = 1;
		let output = self.pipeline.retrieve(query)?;
		self.history.push(output);
		Ok(self.history.last().unwrap())
	}

	/// Apply a feedback action
	pub fn apply_feedback(&mut self, action: FeedbackAction) -> RetrievalResult<&RetrievalOutput> {
		// Check iteration limit
		if self.current_iteration >= self.max_iterations {
			return Err(RetrievalError::Embedding(
				"Maximum iterations reached".to_string(),
			));
		}

		self.current_iteration += 1;

		match action {
			FeedbackAction::MoreContext { symbols } => {
				let xml = self.pipeline.get_more_context(&symbols)?;
				let last = self.history.last().ok_or_else(|| {
					RetrievalError::Embedding("No previous output".to_string())
				})?;

				// Create new output with expanded context
				let output = RetrievalOutput {
					query: last.query.clone(),
					search_spec: last.search_spec.clone(),
					xml_output: xml,
					result_count: symbols.len(),
					token_count: last.token_count * 2, // estimate
					has_more: false,
				};
				self.history.push(output);
			}

			FeedbackAction::RefineSearch { query } => {
				let output = self.pipeline.retrieve(&query)?;
				self.history.push(output);
			}

			FeedbackAction::ExpandRelated { symbols } => {
				// Search for symbols and their relationships
				let query = symbols.join(" OR ");
				let output = self.pipeline.retrieve(&query)?;
				self.history.push(output);
			}

			FeedbackAction::FilterFiles { patterns } => {
				// Re-run last query with file filters
				let last = self.history.last().ok_or_else(|| {
					RetrievalError::Embedding("No previous output".to_string())
				})?;

				// Add file patterns to query
				let filtered_query = format!(
					"{} path:{}",
					last.query,
					patterns.join(" OR path:")
				);
				let output = self.pipeline.retrieve(&filtered_query)?;
				self.history.push(output);
			}

			FeedbackAction::Accept => {
				// No action needed, just return current output
			}
		}

		Ok(self.history.last().unwrap())
	}

	/// Get the current output
	pub fn current_output(&self) -> Option<&RetrievalOutput> {
		self.history.last()
	}

	/// Get all outputs in history
	pub fn history(&self) -> &[RetrievalOutput] {
		&self.history
	}

	/// Get current iteration count
	pub fn iteration(&self) -> usize {
		self.current_iteration
	}

	/// Check if more iterations are allowed
	pub fn can_continue(&self) -> bool {
		self.current_iteration < self.max_iterations
	}

	/// Suggest next action based on current results
	pub fn suggest_action(&self) -> Option<FeedbackAction> {
		let output = self.history.last()?;

		// If no results, suggest refining search
		if output.result_count == 0 {
			return Some(FeedbackAction::RefineSearch {
				query: format!("{} OR similar", output.query),
			});
		}

		// If has_more, suggest getting more context
		if output.has_more && !output.search_spec.symbol_names.is_empty() {
			return Some(FeedbackAction::MoreContext {
				symbols: output.search_spec.symbol_names.clone(),
			});
		}

		// Otherwise, accept
		Some(FeedbackAction::Accept)
	}
}

/// Parse a feedback action from string (for CLI/LLM use)
pub fn parse_feedback_action(input: &str) -> Option<FeedbackAction> {
	let input = input.trim().to_lowercase();

	if input == "accept" || input == "done" || input == "ok" {
		return Some(FeedbackAction::Accept);
	}

	if input.starts_with("more ") || input.starts_with("expand ") {
		let symbols: Vec<String> = input
			.split_whitespace()
			.skip(1)
			.map(String::from)
			.collect();
		if !symbols.is_empty() {
			return Some(FeedbackAction::MoreContext { symbols });
		}
	}

	if input.starts_with("search ") || input.starts_with("find ") {
		let query = input.split_whitespace().skip(1).collect::<Vec<_>>().join(" ");
		if !query.is_empty() {
			return Some(FeedbackAction::RefineSearch { query });
		}
	}

	if input.starts_with("filter ") || input.starts_with("in ") {
		let patterns: Vec<String> = input
			.split_whitespace()
			.skip(1)
			.map(String::from)
			.collect();
		if !patterns.is_empty() {
			return Some(FeedbackAction::FilterFiles { patterns });
		}
	}

	if input.starts_with("related ") {
		let symbols: Vec<String> = input
			.split_whitespace()
			.skip(1)
			.map(String::from)
			.collect();
		if !symbols.is_empty() {
			return Some(FeedbackAction::ExpandRelated { symbols });
		}
	}

	None
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_feedback_action() {
		assert!(matches!(
			parse_feedback_action("accept"),
			Some(FeedbackAction::Accept)
		));

		assert!(matches!(
			parse_feedback_action("more AuthService"),
			Some(FeedbackAction::MoreContext { .. })
		));

		assert!(matches!(
			parse_feedback_action("search authentication"),
			Some(FeedbackAction::RefineSearch { .. })
		));

		assert!(matches!(
			parse_feedback_action("filter src/auth/*.rs"),
			Some(FeedbackAction::FilterFiles { .. })
		));
	}
}

//! Feedback Loop for Iterative Refinement
//!
//! Allows the retrieval pipeline to be refined based on
//! initial results. Supports actions like requesting more
//! context, narrowing search, or expanding to related
//! symbols.

use super::pipeline::RetrievalPipeline;
use super::RetrievalOutput;
use crate::retrieval::RetrievalResult;

/// Feedback loop for iterative retrieval refinement
pub struct FeedbackLoop {
	/// the pipeline instance
	pub(crate) pipeline: RetrievalPipeline,
	/// history of retrieval outputs
	pub(crate) history: Vec<RetrievalOutput>,
	/// max iterations allowed
	#[doc(hidden)]
	pub max_iterations: usize,
	/// current iteration
	#[doc(hidden)]
	pub current_iteration: usize,
}

/// Constructors and initialization.
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
	pub fn with_max_iterations(
		pipeline: RetrievalPipeline,
		max: usize,
	) -> Self {
		Self {
			pipeline,
			history: Vec::new(),
			max_iterations: max,
			current_iteration: 0,
		}
	}

	/// Initialize the pipeline
	pub fn initialize(
		&mut self,
	) -> RetrievalResult<()> {
		self.pipeline.initialize()
	}

	/// Run initial retrieval
	pub fn initial_retrieve(
		&mut self,
		query: &str,
	) -> RetrievalResult<&RetrievalOutput> {
		self.current_iteration = 1;
		let output = self.pipeline.retrieve(query)?;
		self.history.push(output);
		Ok(self.history.last().unwrap())
	}
}


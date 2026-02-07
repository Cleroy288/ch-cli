//! Tests for retrieval::agent::feedback_accessors

use ch_cli::retrieval::agent::feedback::FeedbackLoop;
use ch_cli::retrieval::agent::pipeline::RetrievalPipeline;
use ch_cli::retrieval::agent::RetrievalOutput;
use ch_cli::retrieval::daemon::protocol::{
	QueryIntent, SearchSpec,
};

#[test]
fn test_current_output() {
	let pipeline = RetrievalPipeline::new();
	let mut loop_obj =
		FeedbackLoop::new(pipeline);

	assert!(loop_obj.current_output().is_none());

	let output = RetrievalOutput {
		query: "test".to_string(),
		search_spec: SearchSpec {
			original_query: "test".to_string(),
			symbol_names: vec![],
			intent: QueryIntent::Search,
			file_filters: vec![],
			context_hints: vec![],
		},
		xml_output: "".to_string(),
		result_count: 1,
		token_count: 10,
		has_more: false,
	};
	loop_obj.push_output(output);

	assert!(loop_obj.current_output().is_some());
	assert_eq!(
		loop_obj
			.current_output()
			.unwrap()
			.result_count,
		1,
	);
}

#[test]
fn test_iteration() {
	let pipeline = RetrievalPipeline::new();
	let loop_obj = FeedbackLoop::new(pipeline);

	assert_eq!(loop_obj.iteration(), 0);
}

#[test]
fn test_can_continue() {
	let pipeline = RetrievalPipeline::new();
	let mut loop_obj =
		FeedbackLoop::with_max_iterations(
			pipeline, 3,
		);

	assert!(loop_obj.can_continue());

	loop_obj.current_iteration = 2;
	assert!(loop_obj.can_continue());

	loop_obj.current_iteration = 3;
	assert!(!loop_obj.can_continue());

	loop_obj.current_iteration = 5;
	assert!(!loop_obj.can_continue());
}

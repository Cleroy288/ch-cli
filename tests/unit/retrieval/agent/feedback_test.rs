//! Tests for retrieval::agent::feedback

use ch_cli::retrieval::agent::feedback::FeedbackLoop;
use ch_cli::retrieval::agent::pipeline::RetrievalPipeline;
use ch_cli::retrieval::agent::RetrievalOutput;
use ch_cli::retrieval::daemon::protocol::{
	QueryIntent, SearchSpec,
};

#[test]
fn test_history() {
	let pipeline = RetrievalPipeline::new();
	let mut loop_obj =
		FeedbackLoop::new(pipeline);

	assert_eq!(loop_obj.history().len(), 0);

	let output1 = RetrievalOutput {
		query: "test1".to_string(),
		search_spec: SearchSpec {
			original_query: "test1".to_string(),
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
	let output2 = RetrievalOutput {
		query: "test2".to_string(),
		search_spec: SearchSpec {
			original_query: "test2".to_string(),
			symbol_names: vec![],
			intent: QueryIntent::Search,
			file_filters: vec![],
			context_hints: vec![],
		},
		xml_output: "".to_string(),
		result_count: 2,
		token_count: 20,
		has_more: false,
	};

	loop_obj.push_output(output1);
	loop_obj.push_output(output2);

	assert_eq!(loop_obj.history().len(), 2);
	assert_eq!(
		loop_obj.history()[0].query, "test1"
	);
	assert_eq!(
		loop_obj.history()[1].query, "test2"
	);
}

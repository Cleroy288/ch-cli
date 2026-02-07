//! Tests for retrieval::agent::feedback_scoring

use ch_cli::retrieval::agent::feedback::FeedbackLoop;
use ch_cli::retrieval::agent::feedback_actions::FeedbackAction;
use ch_cli::retrieval::agent::pipeline::RetrievalPipeline;
use ch_cli::retrieval::agent::RetrievalOutput;
use ch_cli::retrieval::daemon::protocol::{
	QueryIntent, SearchSpec,
};

#[test]
fn test_suggest_action() {
	let pipeline = RetrievalPipeline::new();
	let mut loop_obj = FeedbackLoop::new(pipeline);

	// no history
	assert!(loop_obj.suggest_action().is_none());

	// no results -> suggest refine
	let output_empty = RetrievalOutput {
		query: "missing".to_string(),
		search_spec: SearchSpec {
			original_query: "missing".to_string(),
			symbol_names: vec![],
			intent: QueryIntent::Search,
			file_filters: vec![],
			context_hints: vec![],
		},
		xml_output: "".to_string(),
		result_count: 0,
		token_count: 0,
		has_more: false,
	};
	loop_obj.push_output(output_empty);

	assert!(matches!(
		loop_obj.suggest_action(),
		Some(FeedbackAction::RefineSearch { .. })
	));

	// has_more with symbols -> suggest more context
	let output_more = RetrievalOutput {
		query: "test".to_string(),
		search_spec: SearchSpec {
			original_query: "test".to_string(),
			symbol_names: vec![
				"Symbol1".to_string(),
			],
			intent: QueryIntent::Search,
			file_filters: vec![],
			context_hints: vec![],
		},
		xml_output: "".to_string(),
		result_count: 3,
		token_count: 100,
		has_more: true,
	};
	loop_obj.clear_history();
	loop_obj.push_output(output_more);

	assert!(matches!(
		loop_obj.suggest_action(),
		Some(FeedbackAction::MoreContext { .. })
	));

	// normal results -> suggest accept
	let output_normal = RetrievalOutput {
		query: "test".to_string(),
		search_spec: SearchSpec {
			original_query: "test".to_string(),
			symbol_names: vec![],
			intent: QueryIntent::Search,
			file_filters: vec![],
			context_hints: vec![],
		},
		xml_output: "".to_string(),
		result_count: 5,
		token_count: 200,
		has_more: false,
	};
	loop_obj.clear_history();
	loop_obj.push_output(output_normal);

	assert!(matches!(
		loop_obj.suggest_action(),
		Some(FeedbackAction::Accept)
	));
}

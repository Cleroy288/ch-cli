//! Tests for retrieval::agent::feedback_actions

use rustean::retrieval::agent::feedback_actions::{
	parse_feedback_action, FeedbackAction,
};

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
		parse_feedback_action(
			"search authentication"
		),
		Some(FeedbackAction::RefineSearch { .. })
	));

	assert!(matches!(
		parse_feedback_action(
			"filter src/auth/*.rs"
		),
		Some(FeedbackAction::FilterFiles { .. })
	));
}

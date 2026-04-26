use rustean::domain::agent_suggest_rules::{
	detect_suggestions,
};

#[test]
fn short_input_returns_empty() {
	// Arrange & Act
	let result = detect_suggestions("fix bug");

	// Assert
	assert!(result.is_empty());
}

#[test]
fn input_with_existing_agent_returns_empty() {
	// Arrange
	let input = "Fix the api endpoint \
		- agent(sonnet): review";

	// Act
	let result = detect_suggestions(input);

	// Assert
	assert!(result.is_empty());
}

#[test]
fn api_keyword_suggests_security() {
	// Arrange
	let input = "Fix the api endpoint now";

	// Act
	let result = detect_suggestions(input);

	// Assert
	assert_eq!(result.len(), 1);
	assert_eq!(result[0].trigger, "api");
	assert_eq!(result[0].description, "Review security");
}

#[test]
fn refactor_keyword_suggests_tests() {
	// Arrange
	let input = "Refactor the user module";

	// Act
	let result = detect_suggestions(input);

	// Assert
	assert_eq!(result.len(), 1);
	assert_eq!(result[0].trigger, "refactor");
}

#[test]
fn bug_keyword_suggests_analysis() {
	// Arrange
	let input = "There is a bug in login";

	// Act
	let result = detect_suggestions(input);

	// Assert
	assert_eq!(result.len(), 1);
	assert_eq!(result[0].trigger, "bug");
}

#[test]
fn feature_keyword_suggests_test_coverage() {
	// Arrange
	let input = "Add a new feature for export";

	// Act
	let result = detect_suggestions(input);

	// Assert
	assert_eq!(result.len(), 1);
	assert_eq!(result[0].trigger, "feature");
}

#[test]
fn test_keyword_suggests_quality() {
	// Arrange
	let input = "Run the test suite again";

	// Act
	let result = detect_suggestions(input);

	// Assert
	assert_eq!(result.len(), 1);
	assert_eq!(result[0].trigger, "test");
}

#[test]
fn max_two_suggestions_returned() {
	// Arrange — "test" and "refactor" both match
	let input = "refactor and test the module";

	// Act
	let result = detect_suggestions(input);

	// Assert
	assert!(result.len() <= 2);
	assert!(result.len() >= 1);
}

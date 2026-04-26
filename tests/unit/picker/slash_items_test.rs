//! Tests for slash_items filtering

use rustean::picker::mode::PickerMode;
use rustean::picker::slash_items;

/// Helper: query slash commands via items_for_mode
fn query_commands(
	q: &str,
) -> Vec<&'static str> {
	let mode = PickerMode::SlashCommand;
	slash_items::items_for_mode(&mode, q)
		.into_iter()
		.map(|i| i.name)
		.collect()
}

/// Empty query returns all commands
#[test]
fn commands_empty_returns_all() {
	// Arrange / Act
	let names = query_commands("");

	// Assert — Q, A, P, new, model, effort,
	//          e-prompt, agent, backend, git
	assert_eq!(names.len(), 10);
	assert_eq!(names[0], "Q");
	assert_eq!(names[3], "new");
	assert_eq!(names[4], "model");
	assert_eq!(names[5], "effort");
	assert_eq!(names[6], "e-prompt");
	assert_eq!(names[7], "agent");
	assert_eq!(names[8], "backend");
	assert_eq!(names[9], "git");
}

/// Query "m" filters to model only
#[test]
fn commands_m_returns_model() {
	// Arrange / Act
	let names = query_commands("m");

	// Assert
	assert_eq!(names, vec!["model"]);
}

/// Lowercase "q" matches uppercase "Q" item
#[test]
fn commands_case_insensitive() {
	// Arrange / Act
	let names = query_commands("q");

	// Assert
	assert_eq!(names, vec!["Q"]);
}

/// Uppercase "Q" also matches itself
#[test]
fn commands_uppercase_identity() {
	// Arrange / Act
	let names = query_commands("Q");

	// Assert
	assert_eq!(names, vec!["Q"]);
}

/// Query "n" filters to new only
#[test]
fn commands_n_returns_new() {
	// Arrange / Act
	let names = query_commands("n");

	// Assert
	assert_eq!(names, vec!["new"]);
}

/// Non-matching query returns empty
#[test]
fn commands_no_match_returns_empty() {
	// Arrange / Act
	let names = query_commands("xyz");

	// Assert
	assert!(names.is_empty());
}

/// Helper: query slash args via items_for_mode
fn query_args(
	cmd: &str,
	q: &str,
) -> Vec<&'static str> {
	let mode = PickerMode::SlashArg {
		command: cmd.to_string(),
	};
	slash_items::items_for_mode(&mode, q)
		.into_iter()
		.map(|i| i.name)
		.collect()
}

/// Model args with empty query returns all 3
#[test]
fn args_model_returns_all() {
	// Arrange / Act
	let names = query_args("model", "");

	// Assert
	assert_eq!(names.len(), 3);
	assert_eq!(names[0], "haiku");
	assert_eq!(names[1], "sonnet");
	assert_eq!(names[2], "opus");
}

/// Unknown command returns empty
#[test]
fn args_unknown_returns_empty() {
	// Arrange / Act
	let names = query_args("foo", "");

	// Assert
	assert!(names.is_empty());
}

/// Model args filter by prefix
#[test]
fn args_model_prefix_filters() {
	// Arrange / Act
	let names = query_args("model", "s");

	// Assert
	assert_eq!(names, vec!["sonnet"]);
}

/// Effort args with empty query returns all 4
#[test]
fn args_effort_returns_all() {
	// Arrange / Act
	let names = query_args("effort", "");

	// Assert
	assert_eq!(names.len(), 4);
	assert_eq!(names[0], "low");
	assert_eq!(names[3], "max");
}

/// Effort args filter by prefix
#[test]
fn args_effort_prefix_filters() {
	// Arrange / Act
	let names = query_args("effort", "m");

	// Assert
	assert_eq!(names, vec!["medium", "max"]);
}

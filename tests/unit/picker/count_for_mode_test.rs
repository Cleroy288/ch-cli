//! Tests for slash_items::count_for_mode

use rustean::picker::mode::PickerMode;
use rustean::picker::slash_items;

/// Empty query in SlashCommand returns all 5
#[test]
fn count_slash_command_empty_returns_all() {
	// Arrange
	let mode = PickerMode::SlashCommand;

	// Act
	let count =
		slash_items::count_for_mode(&mode, "");

	// Assert
	assert_eq!(count, 5);
}

/// Prefix "m" in SlashCommand returns 1 (model)
#[test]
fn count_slash_command_prefix_m() {
	// Arrange
	let mode = PickerMode::SlashCommand;

	// Act
	let count =
		slash_items::count_for_mode(&mode, "m");

	// Assert
	assert_eq!(count, 1);
}

/// Non-matching query returns 0
#[test]
fn count_slash_command_no_match() {
	// Arrange
	let mode = PickerMode::SlashCommand;

	// Act
	let count =
		slash_items::count_for_mode(&mode, "xyz");

	// Assert
	assert_eq!(count, 0);
}

/// SlashArg model empty returns 3
#[test]
fn count_slash_arg_model_empty() {
	// Arrange
	let mode = PickerMode::SlashArg {
		command: "model".to_string(),
	};

	// Act
	let count =
		slash_items::count_for_mode(&mode, "");

	// Assert
	assert_eq!(count, 3);
}

/// Browse mode returns 0
#[test]
fn count_browse_mode_returns_zero() {
	// Arrange
	let mode = PickerMode::Browse {
		dir: std::path::PathBuf::from("."),
	};

	// Act
	let count =
		slash_items::count_for_mode(&mode, "");

	// Assert
	assert_eq!(count, 0);
}

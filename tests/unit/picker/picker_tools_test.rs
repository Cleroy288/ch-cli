//! Tests for Picker tools mode

use rustean::picker::{Picker, PickerMode};

// -- activate_tools --

/// activate_tools sets Tools mode
#[test]
fn activate_tools_sets_mode() {
	let mut picker = Picker::default();
	picker.activate_tools(5);

	assert_eq!(*picker.mode(), PickerMode::Tools);
	assert_eq!(picker.trigger_position(), 5);
	assert!(picker.is_active());
}

/// activate_tools clears query
#[test]
fn activate_tools_clears_query() {
	let mut picker = Picker::default();
	picker.activate(0);
	picker.push_query('x');

	picker.activate_tools(3);

	assert_eq!(picker.query(), "");
}

/// is_tools_mode returns true for Tools
#[test]
fn is_tools_mode_true() {
	let mut picker = Picker::default();
	picker.activate_tools(0);
	assert!(picker.is_tools_mode());
}

/// is_tools_mode returns false for other modes
#[test]
fn is_tools_mode_false_for_browse() {
	let mut picker = Picker::default();
	picker.activate(0);
	assert!(!picker.is_tools_mode());
}

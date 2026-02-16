//! Tests for Picker tools and doc browser modes

use rustean::picker::{Picker, PickerMode};
use rustean::picker::doc_browser::{
	DocBrowser, DocBrowserEntry,
};

/// Helper: build a minimal DocBrowser
fn make_doc_browser() -> DocBrowser {
	let entries = vec![DocBrowserEntry {
		name: "test_fn".to_string(),
		kind: "Function".to_string(),
		rel_path: "src/lib.rs".to_string(),
		line: 1,
		llm_doc: Some("A test function".to_string()),
	}];
	DocBrowser::new(entries)
}

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

// -- activate_doc_browser --

/// activate_doc_browser sets DocBrowser mode
#[test]
fn activate_doc_browser_sets_mode() {
	let mut picker = Picker::default();
	picker.activate_tools(0);

	let browser = make_doc_browser();
	picker.activate_doc_browser(browser);

	assert_eq!(*picker.mode(), PickerMode::DocBrowser);
}

/// doc_browser() returns browser after activation
#[test]
fn doc_browser_returns_some_after_activate() {
	let mut picker = Picker::default();
	let browser = make_doc_browser();
	picker.activate_doc_browser(browser);

	let result = picker.doc_browser();
	assert!(result.is_some());
	assert_eq!(result.unwrap().len(), 1);
}

/// doc_browser() returns None when not active
#[test]
fn doc_browser_returns_none_by_default() {
	let picker = Picker::default();
	assert!(picker.doc_browser().is_none());
}

// -- deactivate clears doc_browser --

/// deactivate clears doc_browser
#[test]
fn deactivate_clears_doc_browser() {
	let mut picker = Picker::default();
	let browser = make_doc_browser();
	picker.activate_doc_browser(browser);

	picker.deactivate();

	assert!(picker.doc_browser().is_none());
	assert_eq!(*picker.mode(), PickerMode::Inactive);
}

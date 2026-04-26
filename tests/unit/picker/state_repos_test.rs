//! Tests for picker repo selection state.

use rustean::domain::repo_info::RepoEntry;
use rustean::domain::tool_ref::ToolKind;
use rustean::fs::FileCache;
use rustean::picker::mode::PickerMode;
use rustean::picker::state::Picker;

/// Build test repo entries
fn make_repos() -> Vec<RepoEntry> {
	vec![
		RepoEntry {
			folder: "backend".into(),
			path: "./backend".into(),
			host: "bitbucket.org".into(),
			workspace: "ws".into(),
			repo_slug: "backend".into(),
		},
		RepoEntry {
			folder: "frontend".into(),
			path: "./frontend".into(),
			host: "bitbucket.org".into(),
			workspace: "ws".into(),
			repo_slug: "frontend".into(),
		},
	]
}

/// activate_repo_select sets mode correctly
#[test]
fn activate_sets_mode() {
	// Arrange
	let cache = FileCache::empty();
	let mut picker = Picker::new(cache);

	// Act
	picker.activate_repo_select(
		ToolKind::Branch, make_repos(),
	);

	// Assert
	assert!(matches!(
		picker.mode(),
		PickerMode::RepoSelect {
			tool: ToolKind::Branch,
		},
	));
}

/// filtered_repos returns all when query empty
#[test]
fn filtered_returns_all_when_no_query() {
	// Arrange
	let cache = FileCache::empty();
	let mut picker = Picker::new(cache);
	picker.activate_repo_select(
		ToolKind::Branch, make_repos(),
	);

	// Act
	let filtered = picker.filtered_repos();

	// Assert
	assert_eq!(filtered.len(), 2);
}

/// filtered_repos filters by folder name
#[test]
fn filtered_by_folder_name() {
	// Arrange
	let cache = FileCache::empty();
	let mut picker = Picker::new(cache);
	picker.activate_repo_select(
		ToolKind::Branch, make_repos(),
	);
	picker.push_query('b');
	picker.push_query('a');
	picker.push_query('c');
	picker.push_query('k');

	// Act
	let filtered = picker.filtered_repos();

	// Assert
	assert_eq!(filtered.len(), 1);
	assert_eq!(filtered[0].folder, "backend");
}

/// selected_repo returns first by default
#[test]
fn selected_repo_returns_first() {
	// Arrange
	let cache = FileCache::empty();
	let mut picker = Picker::new(cache);
	picker.activate_repo_select(
		ToolKind::PullRequest, make_repos(),
	);

	// Act
	let selected = picker.selected_repo();

	// Assert
	assert!(selected.is_some());
	assert_eq!(
		selected.unwrap().folder, "backend",
	);
}

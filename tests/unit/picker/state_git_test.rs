//! Tests for picker git history state

use rustean::domain::git_commit::GitCommit;
use rustean::domain::git_graph::{
	Connector, GitHistory, GraphNode,
};
use rustean::picker::mode::PickerMode;
use rustean::picker::Picker;
use std::path::PathBuf;

fn empty_history() -> GitHistory {
	GitHistory {
		repo_path: PathBuf::from("/tmp/repo"),
		nodes: Vec::new(),
		max_lanes: 0,
	}
}

fn history_with_nodes(count: usize) -> GitHistory {
	let nodes = (0..count)
		.map(|i| GraphNode {
			column: 0,
			commit: GitCommit {
				hash: format!("{i:07}"),
				short_hash: format!("{i:04}"),
				author: "A".to_string(),
				date: "1h".to_string(),
				message: format!("c{i}"),
				parent_hashes: Vec::new(),
				refs: Vec::new(),
			},
			connectors: vec![Connector::Pipe],
			lane_count: 1,
			merge_to: Vec::new(),
		})
		.collect();
	GitHistory {
		repo_path: PathBuf::from("/tmp/repo"),
		nodes,
		max_lanes: 1,
	}
}

/// Activate sets mode and resets selection.
#[test]
fn activate_sets_mode_and_selected() {
	// Arrange
	let mut picker = Picker::default();

	// Act
	picker.activate_git_history(
		history_with_nodes(5),
	);

	// Assert
	assert_eq!(
		*picker.mode(),
		PickerMode::GitHistory,
	);
	assert_eq!(picker.git_selected(), 0);
	assert!(picker.git_history().is_some());
}

/// Select down increments.
#[test]
fn select_down_increments() {
	// Arrange
	let mut picker = Picker::default();
	picker.activate_git_history(
		history_with_nodes(10),
	);

	// Act
	picker.select_git_down();
	picker.select_git_down();

	// Assert
	assert_eq!(picker.git_selected(), 2);
}

/// Select down stops at last node.
#[test]
fn select_down_capped_at_max() {
	// Arrange
	let mut picker = Picker::default();
	picker.activate_git_history(
		history_with_nodes(3),
	);

	// Act
	for _ in 0..10 {
		picker.select_git_down();
	}

	// Assert
	assert_eq!(picker.git_selected(), 2);
}

/// Select up saturates at zero.
#[test]
fn select_up_saturates_at_zero() {
	// Arrange
	let mut picker = Picker::default();
	picker.activate_git_history(empty_history());

	// Act
	picker.select_git_up();

	// Assert
	assert_eq!(picker.git_selected(), 0);
}

/// Enter detail mode for selected commit.
#[test]
fn enter_detail_switches_mode() {
	// Arrange
	let mut picker = Picker::default();
	picker.activate_git_history(
		history_with_nodes(3),
	);

	// Act
	picker.enter_git_detail();

	// Assert
	assert_eq!(
		*picker.mode(),
		PickerMode::GitDetail,
	);
}

/// Back from detail returns to list.
#[test]
fn back_from_detail_returns_to_list() {
	// Arrange
	let mut picker = Picker::default();
	picker.activate_git_history(
		history_with_nodes(3),
	);
	picker.enter_git_detail();

	// Act
	picker.back_to_git_list();

	// Assert
	assert_eq!(
		*picker.mode(),
		PickerMode::GitHistory,
	);
}

/// Scrolling past the old hardcoded cap of 20 works.
#[test]
fn scroll_git_detail_down_scrolls_past_old_cap_of_twenty() {
	// Arrange
	let mut picker = Picker::default();
	picker.activate_git_history(
		history_with_nodes(3),
	);
	picker.enter_git_detail();

	// Act
	for _ in 0..25 {
		picker.scroll_git_detail_down();
	}

	// Assert
	assert_eq!(picker.git_detail_scroll(), 25);
}

/// Clear resets to Inactive.
#[test]
fn clear_resets_to_inactive() {
	// Arrange
	let mut picker = Picker::default();
	picker.activate_git_history(empty_history());

	// Act
	picker.clear_git_history();

	// Assert
	assert_eq!(
		*picker.mode(),
		PickerMode::Inactive,
	);
	assert!(picker.git_history().is_none());
}

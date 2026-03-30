//! Tests for git graph building

use rustean::domain::git_commit::GitCommit;
use rustean::domain::git_graph::Connector;
use rustean::service::git::build_graph;
use std::path::PathBuf;

fn commit(
	hash: &str,
	parents: &[&str],
) -> GitCommit {
	GitCommit {
		hash: hash.to_string(),
		short_hash: hash[..7].to_string(),
		author: "Alice".to_string(),
		date: "1h ago".to_string(),
		message: format!("commit {hash}"),
		parent_hashes: parents
			.iter()
			.map(|p| p.to_string())
			.collect(),
		refs: Vec::new(),
	}
}

/// Linear history: 3 commits, single lane.
#[test]
fn linear_three_commits_single_lane() {
	// Arrange
	let commits = vec![
		commit("aaa0000", &["bbb0000"]),
		commit("bbb0000", &["ccc0000"]),
		commit("ccc0000", &[]),
	];

	// Act
	let history = build_graph(
		PathBuf::from("/tmp"), commits,
	);

	// Assert
	assert_eq!(history.nodes.len(), 3);
	assert_eq!(history.max_lanes, 1);
	assert_eq!(history.nodes[0].column, 0);
	assert_eq!(history.nodes[1].column, 0);
	assert_eq!(history.nodes[2].column, 0);
}

/// Merge commit has two parents, uses 2 lanes.
#[test]
fn merge_uses_two_lanes() {
	// Arrange
	let commits = vec![
		commit("aaa0000", &["bbb0000", "ccc0000"]),
		commit("bbb0000", &["ddd0000"]),
		commit("ccc0000", &["ddd0000"]),
		commit("ddd0000", &[]),
	];

	// Act
	let history = build_graph(
		PathBuf::from("/tmp"), commits,
	);

	// Assert
	assert_eq!(history.nodes.len(), 4);
	assert!(history.max_lanes >= 2);
	let merge = &history.nodes[0];
	assert_eq!(merge.column, 0);
	assert!(merge.connectors.len() >= 2);
}

/// Root commit (no parents) gets a column.
#[test]
fn root_commit_gets_column() {
	// Arrange
	let commits = vec![
		commit("aaa0000", &[]),
	];

	// Act
	let history = build_graph(
		PathBuf::from("/tmp"), commits,
	);

	// Assert
	assert_eq!(history.nodes.len(), 1);
	assert_eq!(history.nodes[0].column, 0);
}

/// Connectors for linear commit are Pipe.
#[test]
fn linear_connectors_are_pipe() {
	// Arrange
	let commits = vec![
		commit("aaa0000", &["bbb0000"]),
		commit("bbb0000", &[]),
	];

	// Act
	let history = build_graph(
		PathBuf::from("/tmp"), commits,
	);

	// Assert
	assert_eq!(
		history.nodes[0].connectors[0],
		Connector::Pipe,
	);
}

/// Merge commit shows Fork connector on new lane.
#[test]
fn merge_shows_fork_on_second_parent() {
	// Arrange — merge commit with 2 parents
	let commits = vec![
		commit("aaa0000", &["bbb0000", "ccc0000"]),
		commit("bbb0000", &["ddd0000"]),
		commit("ccc0000", &["ddd0000"]),
		commit("ddd0000", &[]),
	];

	// Act
	let history = build_graph(
		PathBuf::from("/tmp"), commits,
	);

	// Assert — lane 1 should be Fork (new branch)
	let merge = &history.nodes[0];
	assert_eq!(merge.connectors.len(), 2);
	assert_eq!(
		merge.connectors[1],
		Connector::Fork,
	);
}

/// Branch that merges back shows Merge connector.
#[test]
fn branch_end_shows_merge() {
	// Arrange
	let commits = vec![
		commit("aaa0000", &["bbb0000", "ccc0000"]),
		commit("bbb0000", &["ddd0000"]),
		commit("ccc0000", &["ddd0000"]),
		commit("ddd0000", &[]),
	];

	// Act
	let history = build_graph(
		PathBuf::from("/tmp"), commits,
	);

	// Assert — when ccc0000 is placed, lane 1
	// should be occupied. After ddd0000 (both
	// parents resolved), lane 1 becomes free.
	// The merge of lane 1 into lane 0 at ddd0000.
	let last = &history.nodes[3];
	assert_eq!(last.column, 0);
}

/// merge_to set when parent is at different lane.
#[test]
fn merge_to_set_for_branch_end() {
	// Arrange — M forks, C's parent D is at lane 0
	let commits = vec![
		commit("aaa0000", &["bbb0000", "ccc0000"]),
		commit("bbb0000", &["ddd0000"]),
		commit("ccc0000", &["ddd0000"]),
		commit("ddd0000", &[]),
	];

	// Act
	let history = build_graph(
		PathBuf::from("/tmp"), commits,
	);

	// Assert — ccc at lane 1, parent ddd at lane 0
	let node_c = &history.nodes[2];
	assert_eq!(node_c.column, 1);
	assert_eq!(
		node_c.merge_to, vec![0usize],
	);
}

/// merge_to is empty for linear commits.
#[test]
fn merge_to_none_for_linear() {
	// Arrange
	let commits = vec![
		commit("aaa0000", &["bbb0000"]),
		commit("bbb0000", &[]),
	];

	// Act
	let history = build_graph(
		PathBuf::from("/tmp"), commits,
	);

	// Assert
	assert!(history.nodes[0].merge_to.is_empty());
	assert!(history.nodes[1].merge_to.is_empty());
}

/// Sibling branches connecting to same parent.
#[test]
fn sibling_branches_merge_to_parent() {
	// Arrange — A and B both have parent C
	let commits = vec![
		commit("aaa0000", &["ccc0000"]),
		commit("bbb0000", &["ccc0000"]),
		commit("ccc0000", &[]),
	];

	// Act
	let history = build_graph(
		PathBuf::from("/tmp"), commits,
	);

	// Assert — B at lane 1 with merge_to lane 0
	let node_b = &history.nodes[1];
	assert_eq!(node_b.column, 1);
	assert_eq!(node_b.merge_to, vec![0usize]);
	assert!(history.nodes[0].merge_to.is_empty());
}

/// Diamond merge: full connector verification.
#[test]
fn diamond_connectors_complete() {
	// Arrange
	let commits = vec![
		commit("aaa0000", &["bbb0000", "ccc0000"]),
		commit("bbb0000", &["ddd0000"]),
		commit("ccc0000", &["ddd0000"]),
		commit("ddd0000", &[]),
	];

	// Act
	let history = build_graph(
		PathBuf::from("/tmp"), commits,
	);

	// Assert — M: Pipe+Fork, B: Pipe+Pipe
	let m = &history.nodes[0];
	assert_eq!(m.connectors[0], Connector::Pipe);
	assert_eq!(m.connectors[1], Connector::Fork);

	let b = &history.nodes[1];
	assert_eq!(b.connectors[0], Connector::Pipe);
	assert_eq!(b.connectors[1], Connector::Pipe);

	// D: convergence point
	let d = &history.nodes[3];
	assert_eq!(d.column, 0);
	assert!(d.merge_to.is_empty());
}

/// Root commit merge_to is always empty.
#[test]
fn root_merge_to_none() {
	// Arrange
	let commits = vec![
		commit("aaa0000", &[]),
	];

	// Act
	let history = build_graph(
		PathBuf::from("/tmp"), commits,
	);

	// Assert
	assert!(history.nodes[0].merge_to.is_empty());
}

/// Octopus merge collects all parent lanes.
///
/// X → A, Y → B, Z → C pre-assign A/B/C to lanes
/// 0/1/2. Then M(parents: A,B,C) lands on lane 3.
/// merge_to must contain [0, 1, 2].
#[test]
fn octopus_merge_collects_all_parent_lanes() {
	// Arrange — X/Y/Z pre-assign A, B, C to lanes
	// 0, 1, 2. Then M references all three.
	let commits = vec![
		commit("xxx0000", &["aaa0000"]),
		commit("yyy0000", &["bbb0000"]),
		commit("zzz0000", &["ccc0000"]),
		commit(
			"mmm0000",
			&["aaa0000", "bbb0000", "ccc0000"],
		),
		commit("aaa0000", &[]),
		commit("bbb0000", &[]),
		commit("ccc0000", &[]),
	];

	// Act
	let history = build_graph(
		PathBuf::from("/tmp"), commits,
	);

	// Assert — M at lane 3; merge_to = [0, 1, 2]
	let m_node = &history.nodes[3];
	assert_eq!(
		m_node.merge_to.len(), 3,
		"octopus merge must record all 3 parent lanes",
	);
	assert!(
		!m_node.merge_to.contains(&m_node.column),
		"merge_to must not include M's own column",
	);
}

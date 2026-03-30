use std::path::PathBuf;

use super::git_commit::GitCommit;

/// Visual connector between graph lanes.
#[derive(Debug, Clone, PartialEq)]
pub enum Connector {
	/// Vertical continuation │
	Pipe,
	/// New branch starts here ╮
	Fork,
	/// Branch merges back ╯
	Merge,
	/// Empty (no connector)
	Empty,
}

/// One row of the visual git graph.
#[derive(Debug, Clone, PartialEq)]
pub struct GraphNode {
	/// Which lane (column) this commit sits in
	pub column: usize,
	/// The parsed commit data
	pub commit: GitCommit,
	/// Connector chars for each active lane
	pub connectors: Vec<Connector>,
	/// Total active lanes at this row
	pub lane_count: usize,
	/// Parent lanes when commit's lane is freed
	pub merge_to: Vec<usize>,
}

/// Full git history for one repository.
#[derive(Debug, Clone)]
pub struct GitHistory {
	/// Path to the repository root
	pub repo_path: PathBuf,
	/// Graph nodes in topological order
	pub nodes: Vec<GraphNode>,
	/// Maximum lane count across all rows
	pub max_lanes: usize,
}

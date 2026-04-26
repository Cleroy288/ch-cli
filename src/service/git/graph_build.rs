use std::path::PathBuf;

use crate::domain::git_commit::GitCommit;
use crate::domain::git_graph::GitHistory;

use super::graph_builder::GraphBuilder;

/// Build a visual graph from parsed commits.
pub fn build_graph(
	repo_path: PathBuf,
	commits: Vec<GitCommit>,
) -> GitHistory {
	let mut builder = GraphBuilder::new();
	let nodes: Vec<_> = commits
		.into_iter()
		.map(|c| builder.place_commit(c))
		.collect();
	let max_lanes = nodes
		.iter()
		.map(|n| n.connectors.len())
		.max()
		.unwrap_or(0);
	GitHistory {
		repo_path,
		nodes,
		max_lanes,
	}
}

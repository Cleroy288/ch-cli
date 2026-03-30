use std::collections::HashMap;

use crate::domain::git_commit::GitCommit;
use crate::domain::git_graph::GraphNode;

use super::graph_connect::build_connectors;

pub struct GraphBuilder {
	/// lane → hash occupying it (None = free)
	pub(super) lanes: Vec<Option<String>>,
	/// hash → assigned lane
	assigned: HashMap<String, usize>,
}

impl GraphBuilder {
	pub fn new() -> Self {
		Self {
			lanes: Vec::new(),
			assigned: HashMap::new(),
		}
	}

	pub fn place_commit(
		&mut self,
		commit: GitCommit,
	) -> GraphNode {
		let col = self.resolve_column(&commit);
		let merge_to: Vec<usize> = commit
			.parent_hashes
			.iter()
			.filter_map(|p| self.assigned.get(p).copied())
			.filter(|&lane| lane != col)
			.collect();
		let before = self.lane_snapshot();
		self.register_parents(&commit, col);
		let after = self.lane_snapshot();
		let connectors = build_connectors(
			col, &before, &after,
		);
		let lane_count = self.active_count();
		GraphNode {
			column: col,
			commit,
			connectors,
			lane_count,
			merge_to,
		}
	}

	fn resolve_column(
		&mut self,
		commit: &GitCommit,
	) -> usize {
		if let Some(&lane) =
			self.assigned.get(&commit.hash)
		{
			self.lanes[lane] = None;
			self.assigned.remove(&commit.hash);
			return lane;
		}
		self.allocate_lane()
	}
}

impl GraphBuilder {
	pub(super) fn allocate_lane(
		&mut self,
	) -> usize {
		let free = self.lanes
			.iter()
			.position(|l| l.is_none());
		match free {
			Some(idx) => idx,
			None => {
				self.lanes.push(None);
				self.lanes.len() - 1
			}
		}
	}

	fn register_parents(
		&mut self,
		commit: &GitCommit,
		col: usize,
	) {
		for (idx, parent) in
			commit.parent_hashes.iter().enumerate()
		{
			if self.assigned.contains_key(parent) {
				continue;
			}
			let lane = if idx == 0 {
				col
			} else {
				self.allocate_lane()
			};
			self.lanes[lane] =
				Some(parent.clone());
			self.assigned
				.insert(parent.clone(), lane);
		}
	}

	fn lane_snapshot(&self) -> Vec<bool> {
		self.lanes
			.iter()
			.map(|l| l.is_some())
			.collect()
	}

	fn active_count(&self) -> usize {
		self.lanes
			.iter()
			.filter(|l| l.is_some())
			.count()
	}
}

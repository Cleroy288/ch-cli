use crate::domain::git_graph::GraphNode;

use super::state::Picker;

impl Picker {
	/// Current git history (if viewing).
	pub fn git_history(
		&self,
	) -> Option<&crate::domain::git_graph::GitHistory> {
		self.git_history.as_ref()
	}

	/// Currently selected commit index.
	pub fn git_selected(&self) -> usize {
		self.git_selected
	}

	/// The selected GraphNode (if any).
	pub fn selected_git_node(
		&self,
	) -> Option<&GraphNode> {
		self.git_history
			.as_ref()?
			.nodes
			.get(self.git_selected)
	}

	/// Move selection up.
	pub fn select_git_up(&mut self) {
		self.git_selected =
			self.git_selected.saturating_sub(1);
	}

	/// Move selection down.
	pub fn select_git_down(&mut self) {
		let max = self.git_history
			.as_ref()
			.map(|h| h.nodes.len())
			.unwrap_or(0);
		if self.git_selected + 1 < max {
			self.git_selected += 1;
		}
	}
}

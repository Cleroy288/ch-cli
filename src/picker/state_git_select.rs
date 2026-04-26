use std::path::PathBuf;

use crate::picker::mode::PickerMode;

use super::state::Picker;

impl Picker {
	/// Enter repo selection mode.
	pub fn activate_git_repo_select(
		&mut self,
		repos: Vec<PathBuf>,
	) {
		self.git_repos = repos;
		self.query.clear();
		self.mode = PickerMode::GitRepoSelect;
	}

	/// Repos filtered by current query.
	pub fn filtered_git_repos(
		&self,
	) -> Vec<&PathBuf> {
		let q = self.query.query().to_lowercase();
		if q.is_empty() {
			return self.git_repos
				.iter().collect();
		}
		self.git_repos
			.iter()
			.filter(|r| repo_name(r).contains(&q))
			.collect()
	}

	/// Currently selected git repo path.
	pub fn selected_git_repo(
		&self,
	) -> Option<&PathBuf> {
		let filtered = self.filtered_git_repos();
		filtered
			.get(self.selected_index())
			.copied()
	}
}

fn repo_name(path: &PathBuf) -> String {
	path.file_name()
		.and_then(|n| n.to_str())
		.unwrap_or("")
		.to_lowercase()
}

use crate::domain::repo_info::RepoEntry;
use crate::domain::tool_ref::ToolKind;
use crate::picker::mode::PickerMode;
use crate::picker::state::Picker;

impl Picker {
	/// Enter RepoSelect mode with discovered repos
	pub fn activate_repo_select(
		&mut self,
		tool: ToolKind,
		repos: Vec<RepoEntry>,
	) {
		self.repo_entries = repos;
		self.query.clear();
		self.mode = PickerMode::RepoSelect { tool };
	}

	pub fn filtered_repos(
		&self,
	) -> Vec<&RepoEntry> {
		let q = self.query.query().to_lowercase();
		if q.is_empty() {
			return self.repo_entries
				.iter().collect();
		}
		self.repo_entries
			.iter()
			.filter(|repo| {
				repo.folder
					.to_lowercase()
					.contains(&q)
					|| repo.repo_slug
						.to_lowercase()
						.contains(&q)
			})
			.collect()
	}

	/// Currently selected repo entry
	pub fn selected_repo(
		&self,
	) -> Option<&RepoEntry> {
		let filtered = self.filtered_repos();
		filtered.get(self.selected_index()).copied()
	}
}

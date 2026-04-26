use std::path::{Path, PathBuf};
use std::sync::mpsc;

use crate::app::App;
use crate::domain::errors::git::GitResult;
use crate::domain::git_graph::GitHistory;
use crate::service::git;

const NO_GIT_REPO: &str =
	"No git repository found";
const GIT_LOADING: &str =
	"Loading git history...";

impl App {
	/// Entry point for `/git` command.
	pub(crate) fn handle_git_command(&mut self) {
		let root =
			Path::new(&self.project_root);
		let repos = git::discover_git_repos(root);
		self.dispatch_git_repos(repos);
	}

	fn dispatch_git_repos(
		&mut self,
		repos: Vec<PathBuf>,
	) {
		match repos.len() {
			0 => {
				self.status_message =
					Some(NO_GIT_REPO.to_string());
			}
			1 => self.spawn_git_fetch(
				repos[0].clone(),
			),
			_ => self.picker
				.activate_git_repo_select(repos),
		}
	}

	/// Spawn background thread to fetch git log.
	pub(super) fn spawn_git_fetch(
		&mut self,
		repo: PathBuf,
	) {
		crate::app::git_poll_watcher::stop_watcher(
			self,
		);
		let (tx, rx) = mpsc::channel();
		self.git_rx = Some(rx);
		self.picker.activate_git_loading();
		self.status_message =
			Some(GIT_LOADING.to_string());
		std::thread::spawn(move || {
			let result = fetch_and_build(&repo);
			let _ = tx.send(result);
		});
	}
}

fn fetch_and_build(
	repo: &Path,
) -> GitResult<GitHistory> {
	let commits = git::fetch_git_log(repo)?;
	Ok(git::build_graph(
		repo.to_path_buf(), commits,
	))
}

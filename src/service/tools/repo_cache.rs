use std::io;
use std::path::Path;

use crate::domain::repo_info::RepoCache;
use crate::service::config;

pub fn load_repos(
	root: &Path,
) -> Option<RepoCache> {
	config::load_config(root).cache.repos
}

pub fn save_repos(
	root: &Path,
	cache: &RepoCache,
) -> io::Result<()> {
	let owned = cache.clone();
	config::update_config(root, move |cfg| {
		cfg.cache.repos = Some(owned);
	})
}

pub fn has_repos_cache(root: &Path) -> bool {
	load_repos(root).is_some()
}

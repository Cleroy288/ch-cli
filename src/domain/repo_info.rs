#[derive(
	Debug, Clone,
	serde::Serialize, serde::Deserialize,
)]
pub struct RepoEntry {
	pub folder: String,
	pub path: String,
	pub host: String,
	pub workspace: String,
	pub repo_slug: String,
}

#[derive(
	Debug, Clone,
	serde::Serialize, serde::Deserialize,
)]
pub struct RepoCache {
	pub scan_depth: u32,
	pub discovered_at: String,
	pub repos: Vec<RepoEntry>,
}

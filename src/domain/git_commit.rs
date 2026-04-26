/// A single parsed git commit.
#[derive(Debug, Clone, PartialEq)]
pub struct GitCommit {
	/// Full commit hash
	pub hash: String,
	/// Abbreviated hash
	pub short_hash: String,
	/// Author name
	pub author: String,
	/// Relative date (e.g. "2h ago")
	pub date: String,
	/// First line of commit message
	pub message: String,
	/// Parent commit hashes (0=root, 2+=merge)
	pub parent_hashes: Vec<String>,
	/// Branch/tag refs decorating this commit
	pub refs: Vec<String>,
}

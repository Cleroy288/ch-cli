use serde::{Deserialize, Serialize};

/// Maps project hashes to filesystem paths
#[derive(
	Debug, Clone, Default,
	Serialize, Deserialize,
)]
pub struct ProjectManifest {
	#[serde(default)]
	pub projects: Vec<ProjectEntry>,
}

/// One registered project
#[derive(
	Debug, Clone,
	Serialize, Deserialize, PartialEq, Eq,
)]
pub struct ProjectEntry {
	/// 8-char blake3 hash of canonical path
	pub hash: String,
	/// Absolute path to project root
	pub path: String,
}

impl ProjectManifest {
	/// Insert or update a project entry
	pub fn upsert(&mut self, entry: ProjectEntry) {
		if let Some(existing) = self
			.projects
			.iter_mut()
			.find(|p| p.hash == entry.hash)
		{
			existing.path = entry.path;
		} else {
			self.projects.push(entry);
		}
	}
}

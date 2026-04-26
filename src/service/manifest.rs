use std::io;
use std::path::Path;

use crate::domain::data_paths;
use crate::domain::data_paths_dirs;
use crate::domain::manifest::{
	ProjectEntry, ProjectManifest,
};

/// Load manifest from ~/.config/rustean/projects.json
pub fn load_manifest() -> ProjectManifest {
	let path = data_paths_dirs::manifest_file();
	let Ok(data) = std::fs::read_to_string(&path)
	else {
		return ProjectManifest::default();
	};
	serde_json::from_str(&data)
		.unwrap_or_default()
}

/// Save manifest to ~/.config/rustean/projects.json
pub fn save_manifest(
	manifest: &ProjectManifest,
) -> io::Result<()> {
	let path = data_paths_dirs::manifest_file();
	ensure_parent(&path)?;
	let json =
		serde_json::to_string_pretty(manifest)
			.map_err(io::Error::other)?;
	std::fs::write(&path, json)
}

/// Register current project in the manifest
pub fn ensure_registered(
	root: &Path,
) -> io::Result<()> {
	let canonical = root
		.canonicalize()
		.unwrap_or_else(|_| root.to_path_buf());
	let hash = data_paths::project_hash(root);
	let entry = ProjectEntry {
		hash,
		path: canonical.to_string_lossy().to_string(),
	};
	let mut manifest = load_manifest();
	manifest.upsert(entry);
	save_manifest(&manifest)
}

/// Create parent directory if needed
fn ensure_parent(path: &Path) -> io::Result<()> {
	if let Some(parent) = path.parent() {
		std::fs::create_dir_all(parent)?;
	}
	Ok(())
}

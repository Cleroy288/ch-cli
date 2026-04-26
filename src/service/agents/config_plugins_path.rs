use std::path::{Path, PathBuf};

/// Cache dir relative to `~/.claude/`
const PLUGIN_CACHE: &str = "plugins/cache";

/// Find the `.mcp.json` for a plugin
/// Path: `~/.claude/plugins/cache/{ns}/{name}/{ver}/`
pub fn plugin_config_path(
	claude_home: &Path,
	plugin_name: &str,
) -> Option<PathBuf> {
	let parts: Vec<&str> =
		plugin_name.splitn(2, '@').collect();
	let (pkg_name, namespace) =
		match parts.as_slice() {
			[name, nsp] => (*name, *nsp),
			_ => return None,
		};
	let plugin_dir = claude_home
		.join(PLUGIN_CACHE)
		.join(namespace)
		.join(pkg_name);
	let version_dir =
		latest_version_dir(&plugin_dir)?;
	let mcp = version_dir.join(".mcp.json");
	if mcp.exists() { Some(mcp) } else { None }
}

/// Parse "1.10.0" → [1, 10, 0] for numeric sort
fn parse_version(name: &str) -> Option<Vec<u32>> {
	name.split('.')
		.map(|s| s.parse::<u32>().ok())
		.collect()
}

/// Find the latest semver subdir (numeric sort)
pub fn latest_version_dir(
	plugin_dir: &Path,
) -> Option<PathBuf> {
	let entries =
		std::fs::read_dir(plugin_dir).ok()?;
	entries
		.filter_map(|e| e.ok())
		.filter(|e| {
			e.file_type()
				.map(|ft| ft.is_dir())
				.unwrap_or(false)
		})
		.map(|e| e.path())
		.max_by(|a, b| {
			let va = a
				.file_name()
				.and_then(|n| n.to_str())
				.and_then(parse_version);
			let vb = b
				.file_name()
				.and_then(|n| n.to_str())
				.and_then(parse_version);
			va.cmp(&vb)
		})
}

/// Extract short name from "name@namespace"
pub(super) fn short_plugin_name(
	full: &str,
) -> String {
	full.split('@')
		.next()
		.unwrap_or(full)
		.to_string()
}

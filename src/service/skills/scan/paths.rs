use std::path::{Path, PathBuf};

/// Home directory for user-scoped commands.
pub(super) fn user_commands_dir() -> Option<PathBuf> {
	dirs::home_dir()
		.map(|h| h.join(".claude").join("commands"))
}

/// Project-relative directory for project commands.
pub(super) fn project_commands_dir() -> PathBuf {
	PathBuf::from(".claude/commands")
}

/// True when the path has a `.md` extension.
pub(super) fn is_markdown(path: &Path) -> bool {
	path.extension()
		.is_some_and(|ext| ext == "md")
}

/// The file stem as a lossy UTF-8 string, empty if absent.
pub(super) fn file_stem(path: &Path) -> String {
	path.file_stem()
		.unwrap_or_default()
		.to_string_lossy()
		.to_string()
}

/// Join a namespace prefix and a name with `:`.
pub(super) fn build_namespace(
	ns: &str,
	name: &str,
) -> String {
	if ns.is_empty() {
		name.to_string()
	} else {
		format!("{ns}:{name}")
	}
}

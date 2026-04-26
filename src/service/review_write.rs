
use std::fs;
use std::io;
use std::path::{Component, Path};

/// Reject absolute paths and `..` components.
fn is_safe_path(path: &str) -> io::Result<()> {
	let p = Path::new(path);
	if p.is_absolute() {
		return Err(io::Error::new(
			io::ErrorKind::PermissionDenied,
			"absolute paths are not allowed",
		));
	}
	let has_parent_dir = p
		.components()
		.any(|c| matches!(c, Component::ParentDir));
	if has_parent_dir {
		return Err(io::Error::new(
			io::ErrorKind::PermissionDenied,
			"path traversal (..) is not allowed",
		));
	}
	Ok(())
}

pub fn write_single_file(
	path: &str,
	content: &str,
) -> io::Result<()> {
	is_safe_path(path)?;
	if let Some(p) = Path::new(path).parent() {
		fs::create_dir_all(p)?;
	}
	fs::write(path, content)
}

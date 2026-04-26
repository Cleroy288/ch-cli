use std::path::Path;

use super::types::ChangeSet;

/// Distinct replacements prevent collisions:
/// `src/foo/bar.rs` → `src__foo__bar_d_rs.json`
/// `src_foo/bar.rs` → `src_foo__bar_d_rs.json`
pub(crate) fn ref_file_key(
	source_file: &Path,
	root: &Path,
) -> String {
	let relative = source_file
		.strip_prefix(root)
		.unwrap_or(source_file);

	let safe: String = relative
		.to_string_lossy()
		.replace('/', "__")
		.replace('\\', "__")
		.replace('.', "_d_");

	format!("{}.json", safe)
}

pub(crate) fn delete_stale_files(
	refs_dir: &Path,
	changes: &Option<ChangeSet>,
	root: &Path,
) -> std::io::Result<()> {
	let Some(change_set) = changes else {
		// Full reindex: clear entire refs/ directory
		if refs_dir.exists() {
			std::fs::remove_dir_all(refs_dir)?;
		}
		return Ok(());
	};

	let stale = change_set
		.modified
		.iter()
		.chain(change_set.deleted.iter());

	for file in stale {
		let key = ref_file_key(file, root);
		let ref_file = refs_dir.join(&key);
		let _ = std::fs::remove_file(&ref_file);
	}
	Ok(())
}

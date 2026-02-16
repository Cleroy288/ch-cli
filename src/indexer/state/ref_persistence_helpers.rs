//! Helpers for per-file reference persistence.
//!
//! Provides file naming, grouping, migration, and
//! stale file deletion for the `refs/` directory layout.

use std::collections::HashMap;
use std::path::Path;

use crate::indexer::semantic::SymbolReference;

use super::types::{ChangeSet, IndexState};

/// Convert a source path to a safe ref filename.
///
/// Example: `src/main.rs` → `src_main_rs.json`
pub(crate) fn ref_file_key(
	source_file: &Path,
	root: &Path,
) -> String {
	let relative = source_file
		.strip_prefix(root)
		.unwrap_or(source_file);

	let safe: String = relative
		.to_string_lossy()
		.chars()
		.map(|chr| match chr {
			'/' | '\\' | '.' => '_',
			_ => chr,
		})
		.collect();

	format!("{}.json", safe)
}

/// Delete ref files for modified/deleted source files.
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

/// Group refs by source file and save each group.
pub(crate) fn save_grouped_refs(
	refs_dir: &Path,
	refs: &[SymbolReference],
	root: &Path,
) -> std::io::Result<()> {
	let mut grouped: HashMap<String, Vec<&SymbolReference>> =
		HashMap::new();

	for sym_ref in refs {
		let key =
			ref_file_key(&sym_ref.location.file, root);
		grouped.entry(key).or_default().push(sym_ref);
	}

	for (key, file_refs) in &grouped {
		let path = refs_dir.join(key);
		let json =
			serde_json::to_string(file_refs).map_err(
				|err| {
					std::io::Error::new(
						std::io::ErrorKind::InvalidData,
						err,
					)
				},
			)?;
		std::fs::write(&path, json)?;
	}
	Ok(())
}

/// Migrate from legacy `refs.json` to per-file `refs/`.
pub(crate) fn migrate_if_needed(
	root: &Path,
	refs_dir: &Path,
) -> std::io::Result<()> {
	let legacy = IndexState::refs_file(root);
	if !legacy.exists() || refs_dir.exists() {
		return Ok(());
	}

	let content = std::fs::read_to_string(&legacy)?;
	let refs: Vec<SymbolReference> =
		serde_json::from_str(&content).map_err(|err| {
			std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				err,
			)
		})?;

	std::fs::create_dir_all(refs_dir)?;
	save_grouped_refs(refs_dir, &refs, root)?;
	std::fs::remove_file(&legacy)?;
	Ok(())
}

/// Load all refs from every file in `refs/` directory.
pub(crate) fn load_all_refs(
	refs_dir: &Path,
) -> std::io::Result<Vec<SymbolReference>> {
	if !refs_dir.exists() {
		return Ok(Vec::new());
	}

	let mut all = Vec::new();
	for entry in std::fs::read_dir(refs_dir)? {
		let path = entry?.path();
		if path
			.extension()
			.is_some_and(|ext| ext == "json")
		{
			let content =
				std::fs::read_to_string(&path)?;
			let refs: Vec<SymbolReference> =
				serde_json::from_str(&content).map_err(
					|err| {
						std::io::Error::new(
							std::io::ErrorKind::InvalidData,
							err,
						)
					},
				)?;
			all.extend(refs);
		}
	}
	Ok(all)
}

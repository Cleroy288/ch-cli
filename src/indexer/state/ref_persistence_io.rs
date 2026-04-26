use std::collections::HashMap;
use std::io;
use std::path::Path;

use crate::indexer::semantic::SymbolReference;

use super::ref_persistence_helpers::ref_file_key;
use super::types::IndexState;

/// Convert serde_json error to io::Error
fn json_io_err(err: serde_json::Error) -> io::Error {
	io::Error::new(io::ErrorKind::InvalidData, err)
}

pub(crate) fn save_grouped_refs(
	refs_dir: &Path,
	refs: &[SymbolReference],
	root: &Path,
) -> io::Result<()> {
	let mut grouped:
		HashMap<String, Vec<&SymbolReference>> =
		HashMap::new();

	for sym_ref in refs {
		let key =
			ref_file_key(&sym_ref.location.file, root);
		grouped
			.entry(key).or_default().push(sym_ref);
	}

	for (key, file_refs) in &grouped {
		let path = refs_dir.join(key);
		let json =
			serde_json::to_string(file_refs)
				.map_err(json_io_err)?;
		std::fs::write(&path, json)?;
	}
	Ok(())
}

/// Migrate from legacy `refs.json` to per-file.
pub(crate) fn migrate_if_needed(
	root: &Path,
	refs_dir: &Path,
) -> io::Result<()> {
	let legacy = IndexState::refs_file(root);
	if !legacy.exists() || refs_dir.exists() {
		return Ok(());
	}

	let content =
		std::fs::read_to_string(&legacy)?;
	let refs: Vec<SymbolReference> =
		serde_json::from_str(&content)
			.map_err(json_io_err)?;

	std::fs::create_dir_all(refs_dir)?;
	save_grouped_refs(refs_dir, &refs, root)?;
	std::fs::remove_file(&legacy)?;
	Ok(())
}

pub(crate) fn load_all_refs(
	refs_dir: &Path,
) -> io::Result<Vec<SymbolReference>> {
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
				serde_json::from_str(&content)
					.map_err(json_io_err)?;
			all.extend(refs);
		}
	}
	Ok(all)
}

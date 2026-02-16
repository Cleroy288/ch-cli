//! Batch processing for documentation generation.
//!
//! Provides batch and ID-based generation methods
//! for processing multiple entries at once.

use std::io::Write;

use crate::retrieval::docgen::entry::DocEntry;
use crate::retrieval::docgen::generator::DocGenerator;
use crate::retrieval::docgen::generator_utils::extract_code_snippet;
use crate::retrieval::models::ModelResult;

/// Fill in code snippet from file if currently empty
fn fill_code_snippet(entry: &mut DocEntry) {
	if !entry.code_snippet.is_empty() {
		return;
	}
	if let Ok(snippet) = extract_code_snippet(
		&entry.file_path,
		entry.line,
	) {
		entry.code_snippet = snippet;
	}
}

/// Batch generation methods.
impl DocGenerator {
	/// Generate documentation for multiple entries.
	///
	/// Returns number of successfully generated docs.
	pub fn generate_batch(
		&mut self,
		entries: &mut [&mut DocEntry],
	) -> ModelResult<usize> {
		let mut success_count = 0;

		for entry in entries.iter_mut() {
			match self.generate(entry) {
				Ok(()) => success_count += 1,
				Err(err) => {
					let _ = writeln!(
						std::io::stderr().lock(),
						"[docgen] Failed for {}: {}",
						entry.name, err
					);
				}
			}
		}

		Ok(success_count)
	}

	/// Generate documentation for entries by ID.
	pub fn generate_for_ids(
		&mut self,
		store: &mut crate::retrieval::docgen::DocStore,
		ids: &[String],
	) -> ModelResult<usize> {
		let mut success_count = 0;

		for entry_id in ids {
			let Some(entry) = store.get_mut(entry_id)
			else {
				continue;
			};
			fill_code_snippet(entry);
			match self.generate(entry) {
				Ok(()) => success_count += 1,
				Err(err) => {
					let _ = writeln!(
						std::io::stderr().lock(),
						"[docgen] Failed for {}: {}",
						entry.name, err
					);
				}
			}
		}

		Ok(success_count)
	}
}

//! Helper methods for IndexManager indexing.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use rayon::prelude::*;

use crate::indexer::crawler::FileResult;
use crate::indexer::parser::ExtractedReference;

use super::builder::IndexManager;

/// Return type for parallel file parsing
type ParseOutput =
	(Vec<FileResult>, Vec<ExtractedReference>);

impl IndexManager {
	/// Parse files in parallel via rayon collect
	pub(super) fn parse_files_parallel(
		&self,
		files_to_process: &[PathBuf],
		total_count: usize,
	) -> ParseOutput {
		let processed = AtomicUsize::new(0);

		let pairs: Vec<_> = files_to_process
			.par_iter()
			.map(|path| {
				let result = self.parse_file(path);
				let refs = self.extract_refs(path);
				self.report_progress(
					&processed, total_count, path,
				);
				(result, refs)
			})
			.collect();

		split_results(pairs)
	}

	/// Report indexing progress via callback
	fn report_progress(
		&self,
		processed: &AtomicUsize,
		total: usize,
		path: &Path,
	) {
		let current =
			processed.fetch_add(1, Ordering::SeqCst) + 1;
		if let Some(ref cb) = self.progress_callback {
			cb(current, total, path);
		}
	}

	/// Extract references from a file if enabled
	fn extract_refs(
		&self,
		path: &Path,
	) -> Vec<ExtractedReference> {
		if !self.flags.reference_extraction {
			return Vec::new();
		}
		self.parse_references(path)
			.unwrap_or_default()
	}
}

/// Split paired results into two separate vectors
fn split_results(
	pairs: Vec<(FileResult, Vec<ExtractedReference>)>,
) -> ParseOutput {
	let cap = pairs.len();
	let mut files = Vec::with_capacity(cap);
	let mut refs = Vec::new();
	for (file, extracted) in pairs {
		files.push(file);
		refs.extend(extracted);
	}
	(files, refs)
}

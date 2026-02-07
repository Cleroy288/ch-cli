//! Helper methods for IndexManager indexing.

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use rayon::prelude::*;

use crate::indexer::crawler::FileResult;
use crate::indexer::parser::ExtractedReference;

use super::builder::IndexManager;

impl IndexManager {
	/// Parse files in parallel and collect results
	pub(super) fn parse_files_parallel(
		&self,
		files_to_process: &[PathBuf],
		files_to_process_count: usize,
	) -> (Vec<FileResult>, Arc<Mutex<Vec<ExtractedReference>>>) {
		let processed = AtomicUsize::new(0);
		let all_references: Arc<Mutex<Vec<ExtractedReference>>> =
			Arc::new(Mutex::new(Vec::new()));

		let file_results: Vec<FileResult> = files_to_process
			.par_iter()
			.map(|path| {
				// Parse symbols
				let result = self.parse_file(path);

				// Extract references if enabled
				if self.enable_reference_extraction {
					if let Ok(refs) = self.parse_references(path) {
						all_references.lock().unwrap().extend(refs);
					}
				}

				// Update counter and call progress callback
				let current = processed.fetch_add(1, Ordering::SeqCst) + 1;
				if let Some(ref callback) = self.progress_callback {
					callback(current, files_to_process_count, path);
				}

				result
			})
			.collect();

		(file_results, all_references)
	}
}

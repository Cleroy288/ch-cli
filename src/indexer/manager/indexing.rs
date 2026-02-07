//! Core indexing logic for IndexManager.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use crate::indexer::crawler::{CrawlStats, Crawler};
use crate::indexer::parser::ExtractedReference;
use crate::indexer::semantic::{SemanticGraph, SymbolReference};
use crate::indexer::state::IndexState;
use crate::indexer::Symbol;

use super::builder::IndexManager;
use super::error::IndexManagerResult;
use super::incremental::{detect_changes, merge_and_persist};
use super::types::IndexResult;

impl IndexManager {
	/// Index an entire project directory
	///
	/// This method:
	/// 1. Discovers all indexable files using the crawler
	/// 2. Detects changes if persistence is enabled (incremental indexing)
	/// 3. Parses files in parallel using rayon (only changed files if incremental)
	/// 4. Extracts references if enabled
	/// 5. Builds semantic graph if enabled
	/// 6. Saves index state to disk if persistence is enabled
	///
	/// Returns an IndexResult with all symbols,
	/// references, and optional search index.
	pub fn index_project<P: AsRef<Path>>(
		&self,
		root: P,
	) -> IndexManagerResult<IndexResult> {
		let root = root
			.as_ref()
			.canonicalize()
			.unwrap_or_else(|_| root.as_ref().to_path_buf());
		let start_time = Instant::now();

		// Step 1: Discover all files
		let crawler = Crawler::with_config(self.crawler_config.clone());
		let all_files = crawler.discover_files(&root);
		let total_files = all_files.len();

		// Step 2: Check for existing index and detect changes (incremental)
		let (files_to_process, changes, mut index_state, is_incremental) =
			if self.enable_persistence {
				detect_changes(&root, &all_files)?
			} else {
				(
					all_files.clone(),
					None,
					IndexState::new(root.clone()),
					false,
				)
			};

		let files_to_process_count = files_to_process.len();

		// Step 3: Parse files in parallel
		let (file_results, all_references) =
			self.parse_files_parallel(&files_to_process, files_to_process_count);

		// Step 4: Collect all symbols from parsed files
		let new_symbols: Vec<Symbol> = file_results
			.iter()
			.flat_map(|r| r.symbols.clone())
			.collect();

		// Step 5: Handle incremental - merge with unchanged files' symbols if needed
		let (all_symbols, search_index) = if self.enable_persistence {
			merge_and_persist(
				&root, &new_symbols,
				&file_results, &changes,
				&mut index_state,
			)?
		} else {
			(new_symbols, None)
		};

		// Step 6: Build semantic graph if enabled
		let references = match Arc::try_unwrap(all_references) {
			Ok(mutex) => mutex.into_inner().unwrap_or_default(),
			Err(arc) => arc.lock().unwrap().clone(),
		};

		let semantic_graph = self.build_semantic_graph(&all_symbols, &references);

		let duration_ms = start_time.elapsed().as_millis() as u64;
		let failed_count = file_results.iter().filter(|r| r.error.is_some()).count();
		let symbols_count: usize = file_results.iter().map(|r| r.symbols.len()).sum();

		Ok(IndexResult {
			root: root.to_path_buf(),
			symbols: all_symbols,
			references,
			file_results,
			stats: CrawlStats {
				files_found: total_files,
				files_parsed: files_to_process_count - failed_count,
				files_failed: failed_count,
				symbols_found: symbols_count,
				duration_ms,
			},
			semantic_graph,
			incremental: is_incremental,
			changes,
			search_index,
		})
	}

	/// Build semantic graph from symbols and references
	fn build_semantic_graph(
		&self,
		symbols: &[Symbol],
		references: &[ExtractedReference],
	) -> Option<SemanticGraph> {
		if self.enable_semantic_analysis {
			let mut graph = SemanticGraph::new();
			graph.add_symbols(symbols);

			// Add references to semantic graph
			for reference in references {
				let sym_ref = SymbolReference {
					name: reference.name.clone(),
					location: reference.location.clone(),
					context: reference.context.clone(),
				};
				graph.add_reference(sym_ref);
			}

			Some(graph)
		} else {
			None
		}
	}

	/// Index specific files only (for targeted re-indexing)
	///
	/// Useful for re-indexing a subset of files
	/// without crawling the entire project.
	pub fn index_files<P: AsRef<Path>>(
		&self,
		root: P,
		files: &[PathBuf],
	) -> IndexManagerResult<IndexResult> {
		let root = root
			.as_ref()
			.canonicalize()
			.unwrap_or_else(|_| root.as_ref().to_path_buf());
		let start_time = Instant::now();

		let (file_results, all_references) =
			self.parse_files_parallel(files, files.len());

		let all_symbols: Vec<Symbol> = file_results
			.iter()
			.flat_map(|r| r.symbols.clone())
			.collect();

		let references = match Arc::try_unwrap(all_references) {
			Ok(mutex) => mutex.into_inner().unwrap_or_default(),
			Err(arc) => arc.lock().unwrap().clone(),
		};

		let semantic_graph = self.build_semantic_graph(&all_symbols, &references);

		let duration_ms = start_time.elapsed().as_millis() as u64;
		let failed_count = file_results.iter().filter(|r| r.error.is_some()).count();
		let symbols_count: usize = file_results.iter().map(|r| r.symbols.len()).sum();

		Ok(IndexResult {
			root: root.to_path_buf(),
			symbols: all_symbols,
			references,
			file_results,
			stats: CrawlStats {
				files_found: files.len(),
				files_parsed: files.len() - failed_count,
				files_failed: failed_count,
				symbols_found: symbols_count,
				duration_ms,
			},
			semantic_graph,
			incremental: false,
			changes: None,
			search_index: None,
		})
	}

}

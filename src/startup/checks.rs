//! Codebase analysis and index state checks.

use std::path::PathBuf;

use crate::indexer::crawler::Crawler;
use crate::indexer::state::IndexState;
use crate::indexer::{
	ChangeSet, CodebaseAnalysis, CodebaseAnalyzer, IndexManager,
};

/// Check if an index exists for the current directory
pub fn check_index_exists() -> bool {
	IndexManager::has_index(".")
}

/// Analyze the codebase to detect its primary language
pub fn analyze_codebase() -> CodebaseAnalysis {
	let analyzer = CodebaseAnalyzer::new();
	analyzer.analyze(".")
}

/// Check if the codebase has changes since last index
pub fn detect_codebase_changes() -> Option<ChangeSet> {
	if !IndexState::exists(".".as_ref()) {
		return None;
	}

	let state = match IndexState::load(".".as_ref()) {
		Ok(loaded) => loaded,
		Err(_) => return None,
	};

	let crawler = Crawler::new();
	let current_files: Vec<PathBuf> =
		crawler.discover_files(".");

	let changes = state.detect_changes(&current_files);

	if changes.has_changes() {
		Some(changes)
	} else {
		None
	}
}


use std::path::{Path, PathBuf};

use crate::indexer::crawler::Crawler;
use crate::indexer::state::IndexState;
use crate::indexer::{
	ChangeSet, CodebaseAnalysis,
	CodebaseAnalyzer, IndexManager,
};

pub fn check_index_exists() -> bool {
	IndexManager::has_index(".")
}

/// Analyze the codebase to detect its primary language
pub fn analyze_codebase() -> CodebaseAnalysis {
	let analyzer = CodebaseAnalyzer::new();
	analyzer.analyze(".")
}

pub fn detect_codebase_changes(
) -> Option<ChangeSet> {
	let root: &Path = ".".as_ref();
	if !IndexState::exists(root) {
		return None;
	}
	let state = IndexState::load(root).ok()?;
	let files: Vec<PathBuf> =
		Crawler::new().discover_files(".");
	let changes = state.detect_changes(&files);
	if changes.has_changes() {
		Some(changes)
	} else {
		None
	}
}


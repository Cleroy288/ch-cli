//! Tests for retrieval::context::graph_walker

use ch_cli::indexer::SemanticGraph;
use ch_cli::retrieval::context::{
	ContextConfig, GraphWalker,
};

#[test]
fn test_extract_snippet() {
	use std::io::Write;
	use tempfile::NamedTempFile;

	let mut temp_file =
		NamedTempFile::new().unwrap();
	writeln!(temp_file, "line 1").unwrap();
	writeln!(temp_file, "line 2").unwrap();
	writeln!(temp_file, "line 3").unwrap();
	writeln!(temp_file, "line 4").unwrap();
	writeln!(temp_file, "line 5").unwrap();

	let graph = SemanticGraph::new();
	let config = ContextConfig::default();
	let walker = GraphWalker::new(&graph, config);

	let snippet = walker.extract_snippet(
		temp_file.path(), 3, 1,
	);

	assert!(snippet.is_some());
	let text = snippet.unwrap();

	assert!(text.contains("line 2"));
	assert!(text.contains("line 3"));
	assert!(text.contains("line 4"));
}

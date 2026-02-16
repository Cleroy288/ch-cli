//! Tests for retrieval::context::graph_walker_usage

use std::path::PathBuf;

use rustean::indexer::ReferenceContext;
use rustean::retrieval::context::graph_walker_usage::{
	UsageCollection, UsageInfo,
};

#[test]
fn test_usage_collection_deduplication() {
	let mut collection = UsageCollection::new();
	let file = PathBuf::from("/test/file.rs");

	// add first usage
	collection.add(UsageInfo {
		file: file.clone(),
		line: 10,
		context: ReferenceContext::Call,
		snippet: Some("test snippet".to_string()),
		containing_symbol: Some(
			"test_fn".to_string(),
		),
	});

	// add duplicate (same file+line)
	collection.add(UsageInfo {
		file: file.clone(),
		line: 10,
		context: ReferenceContext::Type,
		snippet: Some(
			"different snippet".to_string(),
		),
		containing_symbol: Some(
			"other_fn".to_string(),
		),
	});

	// should only have one entry
	assert_eq!(collection.len(), 1);
}

#[test]
fn test_usage_collection_sort_by_file() {
	let mut collection = UsageCollection::new();

	collection.add(UsageInfo {
		file: PathBuf::from("/z/file.rs"),
		line: 5,
		context: ReferenceContext::Call,
		snippet: None,
		containing_symbol: None,
	});
	collection.add(UsageInfo {
		file: PathBuf::from("/a/file.rs"),
		line: 10,
		context: ReferenceContext::Call,
		snippet: None,
		containing_symbol: None,
	});
	collection.add(UsageInfo {
		file: PathBuf::from("/a/file.rs"),
		line: 5,
		context: ReferenceContext::Call,
		snippet: None,
		containing_symbol: None,
	});

	collection.sort_by_file();

	let usages = collection.into_vec();
	assert_eq!(
		usages[0].file,
		PathBuf::from("/a/file.rs"),
	);
	assert_eq!(usages[0].line, 5);
	assert_eq!(
		usages[1].file,
		PathBuf::from("/a/file.rs"),
	);
	assert_eq!(usages[1].line, 10);
	assert_eq!(
		usages[2].file,
		PathBuf::from("/z/file.rs"),
	);
}

#[test]
fn test_usage_collection_truncate() {
	let mut collection = UsageCollection::new();

	for i in 1..=5 {
		collection.add(UsageInfo {
			file: PathBuf::from(
				format!("/test/file{}.rs", i),
			),
			line: i,
			context: ReferenceContext::Call,
			snippet: None,
			containing_symbol: None,
		});
	}

	assert_eq!(collection.len(), 5);
	collection.truncate(3);
	assert_eq!(collection.len(), 3);
}

#[test]
fn test_usage_collection_contains() {
	let mut collection = UsageCollection::new();
	let file = PathBuf::from("/test/file.rs");

	collection.add(UsageInfo {
		file: file.clone(),
		line: 42,
		context: ReferenceContext::Call,
		snippet: None,
		containing_symbol: None,
	});

	assert!(collection.contains(&file, 42));
	assert!(!collection.contains(&file, 43));
	assert!(!collection.contains(
		&PathBuf::from("/other/file.rs"),
		42
	));
}

#[test]
fn test_usage_collection_is_empty() {
	let mut collection = UsageCollection::new();

	assert!(collection.is_empty());
	assert_eq!(collection.len(), 0);

	collection.add(UsageInfo {
		file: PathBuf::from("/test/file.rs"),
		line: 1,
		context: ReferenceContext::Call,
		snippet: None,
		containing_symbol: None,
	});

	assert!(!collection.is_empty());
	assert_eq!(collection.len(), 1);
}

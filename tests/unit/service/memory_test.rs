//! Tests for DefaultMemoryService.
//!
//! Each test uses a temporary directory that is
//! cleaned up on completion via TempMemDir guard.

use rustean::service::memory::DefaultMemoryService;
use rustean::service::memory::MemoryService;

use crate::helpers::factories::{
	TempMemDir, make_interaction,
	make_interaction_with_text,
};

/// Helper: add N interactions to a service
fn add_items(
	svc: &DefaultMemoryService,
	dir: &TempMemDir,
	count: usize,
) {
	for idx in 0..count {
		let text = format!("query {idx}");
		let item = make_interaction(&text);
		svc.add(&dir.path, &item).unwrap();
	}
}

/// add persists an interaction that can be loaded
#[test]
fn add_stores_interaction() {
	// Arrange
	let dir = TempMemDir::new("svc-add");
	let svc = DefaultMemoryService::default();
	let item = make_interaction("hello world");

	// Act
	svc.add(&dir.path, &item).unwrap();
	let recent =
		svc.show_recent(&dir.path, 10).unwrap();

	// Assert
	assert_eq!(recent.len(), 1);
	assert_eq!(recent[0].id, item.id);
}

/// show_recent returns items after adding multiple
#[test]
fn show_recent_returns_all() {
	// Arrange
	let dir = TempMemDir::new("svc-recent");
	let svc = DefaultMemoryService::default();
	add_items(&svc, &dir, 3);

	// Act
	let recent =
		svc.show_recent(&dir.path, 10).unwrap();

	// Assert
	assert_eq!(recent.len(), 3);
}

/// show_recent on empty store returns empty vec
#[test]
fn show_recent_empty_returns_none() {
	// Arrange
	let dir = TempMemDir::new("svc-empty");
	let svc = DefaultMemoryService::default();

	// Act
	let recent =
		svc.show_recent(&dir.path, 10).unwrap();

	// Assert
	assert!(recent.is_empty());
}

/// search finds interactions matching the query
#[test]
fn search_finds_matching() {
	// Arrange
	let dir = TempMemDir::new("svc-search");
	let svc = DefaultMemoryService::default();
	let item = make_interaction_with_text(
		"authentication", "JWT middleware",
	);
	svc.add(&dir.path, &item).unwrap();

	// Act
	let hits = svc
		.search(&dir.path, "authentication", 10)
		.unwrap();

	// Assert
	assert_eq!(hits.len(), 1);
	assert_eq!(
		hits[0].interaction.input.text,
		"authentication",
	);
}

/// search returns empty when nothing matches
#[test]
fn search_no_results() {
	// Arrange
	let dir = TempMemDir::new("svc-no-match");
	let svc = DefaultMemoryService::default();
	let item = make_interaction("hello");
	svc.add(&dir.path, &item).unwrap();

	// Act
	let hits = svc
		.search(&dir.path, "zzzznonexistent", 10)
		.unwrap();

	// Assert
	assert!(hits.is_empty());
}

/// stats returns correct counts after adding items
#[test]
fn stats_returns_counts() {
	// Arrange
	let dir = TempMemDir::new("svc-stats");
	let svc = DefaultMemoryService::default();
	add_items(&svc, &dir, 4);

	// Act
	let stats = svc.stats(&dir.path).unwrap();

	// Assert
	assert_eq!(stats.total, 4);
	assert!(stats.sessions >= 1);
	assert!(stats.size_bytes > 0);
}

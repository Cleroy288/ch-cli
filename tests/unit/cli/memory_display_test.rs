//! Tests for memory_display module.
//!
//! All display functions accept `impl Write` so we
//! test by writing to `Vec<u8>` buffers.

use rustean::cli::commands::memory_display::{
	print_interactions, print_search_hits,
	print_stats, truncate_text,
};
use rustean::indexer::memory::types::{
	MemoryHit, MemoryStats,
};

use crate::helpers::factories::{
	make_interaction, make_interaction_with_text,
};

/// truncate_text: short input unchanged
#[test]
fn truncate_short_text_unchanged() {
	let result = truncate_text("hello", 100);
	assert_eq!(result, "hello");
}

/// truncate_text: long input gets "..."
#[test]
fn truncate_long_text_adds_ellipsis() {
	let input = "a".repeat(200);
	let result = truncate_text(&input, 50);
	assert!(result.ends_with("..."));
	assert!(result.len() <= 53 + 3);
}

/// truncate_text: newlines replaced with spaces
#[test]
fn truncate_replaces_newlines() {
	let result = truncate_text("line1\nline2", 100);
	assert_eq!(result, "line1 line2");
}

/// print_interactions: empty list shows no-results
#[test]
fn print_interactions_empty() {
	let mut buf = Vec::new();
	print_interactions(&mut buf, &[]).unwrap();
	let out = String::from_utf8(buf).unwrap();
	assert!(!out.is_empty());
}

/// print_interactions: non-empty prints entries
#[test]
fn print_interactions_with_items() {
	let items = vec![
		make_interaction("query one"),
		make_interaction("query two"),
	];
	let mut buf = Vec::new();
	print_interactions(&mut buf, &items).unwrap();
	let out = String::from_utf8(buf).unwrap();
	assert!(out.contains("query one"));
	assert!(out.contains("query two"));
}

/// print_search_hits: empty shows no-results
#[test]
fn print_search_hits_empty() {
	let mut buf = Vec::new();
	let hits: Vec<MemoryHit> = vec![];
	print_search_hits(&mut buf, &hits).unwrap();
	let out = String::from_utf8(buf).unwrap();
	assert!(!out.is_empty());
}

/// print_search_hits: shows scores for hits
#[test]
fn print_search_hits_with_items() {
	let item = make_interaction_with_text(
		"auth query", "JWT response",
	);
	let hits = vec![MemoryHit {
		interaction: item,
		score: 0.95,
	}];
	let mut buf = Vec::new();
	print_search_hits(&mut buf, &hits).unwrap();
	let out = String::from_utf8(buf).unwrap();
	assert!(out.contains("0.950"));
	assert!(out.contains("auth query"));
}

/// print_stats: shows all stat fields
#[test]
fn print_stats_shows_all_fields() {
	let stats = MemoryStats {
		total: 42,
		sessions: 3,
		size_bytes: 1024,
		last_updated: 1707900000,
	};
	let mut buf = Vec::new();
	print_stats(&mut buf, &stats).unwrap();
	let out = String::from_utf8(buf).unwrap();
	assert!(out.contains("42"));
	assert!(out.contains("3"));
	assert!(out.contains("1024"));
}

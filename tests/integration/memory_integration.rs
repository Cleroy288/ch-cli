//! Integration tests for the memory system.
//!
//! End-to-end: service -> indexer -> domain.
//! Tests full lifecycle, search, stats, and
//! persistence across service restarts.

use rustean::domain::memory::{
	AiResponse, Interaction, UserInput,
};
use rustean::service::memory::DefaultMemoryService;
use rustean::service::memory::MemoryService;

use crate::helpers::factories::TempMemDir;

/// Build a test interaction with explicit fields
fn build_interaction(
	session: &str,
	input: &str,
	answer: &str,
	ts: u64,
) -> Interaction {
	Interaction {
		id: format!("int-{ts}-{input}"),
		timestamp: ts,
		session_id: session.to_string(),
		input: UserInput {
			text: input.to_string(),
			command: None,
			files: Vec::new(),
		},
		response: AiResponse::Answer {
			text: answer.to_string(),
		},
	}
}

/// Full lifecycle: add → show_recent
#[test]
fn add_and_show_recent_roundtrip() {
	// Arrange
	let tmp = TempMemDir::new("integ-lifecycle");
	let svc = DefaultMemoryService::default();
	let item = build_interaction(
		"ses-1", "how to parse JSON",
		"use serde_json", 1000,
	);

	// Act
	svc.add(&tmp.path, &item).unwrap();
	let recent =
		svc.show_recent(&tmp.path, 10).unwrap();

	// Assert
	assert_eq!(recent.len(), 1);
	assert_eq!(
		recent[0].input.text,
		"how to parse JSON",
	);
}

/// show_recent respects limit parameter
#[test]
fn show_recent_respects_limit() {
	// Arrange
	let tmp = TempMemDir::new("integ-limit");
	let svc = DefaultMemoryService::default();
	for idx in 0..5u64 {
		let item = build_interaction(
			"ses-1",
			&format!("query {idx}"),
			&format!("answer {idx}"),
			1000 + idx,
		);
		svc.add(&tmp.path, &item).unwrap();
	}

	// Act
	let recent =
		svc.show_recent(&tmp.path, 3).unwrap();

	// Assert
	assert_eq!(recent.len(), 3);
}

/// Search finds matching interactions
#[test]
fn search_finds_matching_interactions() {
	// Arrange
	let tmp = TempMemDir::new("integ-search");
	let svc = DefaultMemoryService::default();
	let items = vec![
		build_interaction(
			"ses-1", "how to parse JSON",
			"use serde_json", 1000,
		),
		build_interaction(
			"ses-1", "rust error handling",
			"use Result type", 1001,
		),
	];
	for item in &items {
		svc.add(&tmp.path, item).unwrap();
	}

	// Act
	let hits =
		svc.search(&tmp.path, "parse", 10).unwrap();

	// Assert
	assert!(
		!hits.is_empty(),
		"expected at least one search hit",
	);
}

/// Stats reflect stored counts and sessions
#[test]
fn stats_reflect_stored_counts() {
	// Arrange
	let tmp = TempMemDir::new("integ-stats");
	let svc = DefaultMemoryService::default();
	let items = vec![
		build_interaction(
			"ses-a", "q1", "a1", 1000,
		),
		build_interaction(
			"ses-a", "q2", "a2", 1001,
		),
		build_interaction(
			"ses-b", "q3", "a3", 1002,
		),
	];
	for item in &items {
		svc.add(&tmp.path, item).unwrap();
	}

	// Act
	let stats = svc.stats(&tmp.path).unwrap();

	// Assert
	assert_eq!(stats.total, 3);
	assert_eq!(stats.sessions, 2);
	assert!(stats.size_bytes > 0);
}

/// Data persists across service instances
#[test]
fn data_persists_across_service_restart() {
	// Arrange
	let tmp = TempMemDir::new("integ-persist");
	let item = build_interaction(
		"ses-1", "persistent query",
		"persistent answer", 2000,
	);

	// Act — write with first instance
	{
		let svc = DefaultMemoryService::default();
		svc.add(&tmp.path, &item).unwrap();
	}

	// Recreate service (simulates restart)
	let svc2 = DefaultMemoryService::default();
	let recent =
		svc2.show_recent(&tmp.path, 10).unwrap();

	// Assert
	assert_eq!(recent.len(), 1);
	assert_eq!(
		recent[0].input.text,
		"persistent query",
	);
}

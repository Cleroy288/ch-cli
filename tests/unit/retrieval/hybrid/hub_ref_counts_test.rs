use std::collections::HashMap;

use rustean::retrieval::hybrid::hub_ref_counts::lookup_ref_count;

/// Test lookup_ref_count returns count when present
#[test]
fn lookup_ref_count_found() {
	let mut counts: HashMap<String, usize> = HashMap::new();
	counts.insert("foo".to_string(), 15);
	counts.insert("bar".to_string(), 3);

	assert_eq!(lookup_ref_count("foo", &counts), 15);
	assert_eq!(lookup_ref_count("bar", &counts), 3);
}

/// Test lookup_ref_count returns 0 when not found
#[test]
fn lookup_ref_count_missing() {
	let counts: HashMap<String, usize> = HashMap::new();
	assert_eq!(lookup_ref_count("missing", &counts), 0);
}

/// Test lookup_ref_count with empty map
#[test]
fn lookup_ref_count_empty_map() {
	let counts: HashMap<String, usize> = HashMap::new();
	assert_eq!(lookup_ref_count("any", &counts), 0);
}

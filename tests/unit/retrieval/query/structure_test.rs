use rustean::retrieval::query::detect_structure_query;

#[test]
fn test_detect_modules_in_pattern() {
	let result =
		detect_structure_query("modules in retrieval");
	assert!(result.is_some());
	assert_eq!(result.unwrap().target, "retrieval");
}

#[test]
fn test_detect_what_modules_are_in() {
	let result = detect_structure_query(
		"what modules are in the retrieval directory",
	);
	assert!(result.is_some());
	assert_eq!(result.unwrap().target, "retrieval");
}

#[test]
fn test_detect_whats_in() {
	let result = detect_structure_query(
		"what's in the indexer directory?",
	);
	assert!(result.is_some());
	assert_eq!(result.unwrap().target, "indexer");
}

#[test]
fn test_detect_structure_of() {
	let result = detect_structure_query(
		"structure of daemon module",
	);
	assert!(result.is_some());
	assert_eq!(result.unwrap().target, "daemon");
}

#[test]
fn test_detect_x_structure() {
	let result =
		detect_structure_query("retrieval structure");
	assert!(result.is_some());
	assert_eq!(result.unwrap().target, "retrieval");
}

#[test]
fn test_no_false_positive_conceptual() {
	let result = detect_structure_query(
		"how does retrieval work",
	);
	assert!(result.is_none());
}

#[test]
fn test_no_false_positive_search() {
	let result =
		detect_structure_query("find function in file");
	assert!(result.is_none());
}

#[test]
fn test_no_false_positive_definition() {
	let result =
		detect_structure_query("BgeEmbedder struct");
	assert!(result.is_none());
}

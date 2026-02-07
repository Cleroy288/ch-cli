use ch_cli::retrieval::query::{
	detect_caller_query, CallerDirection,
};

#[test]
fn test_detect_who_calls() {
	let result =
		detect_caller_query("who calls process_data");
	assert!(result.is_some());
	let cq = result.unwrap();
	assert_eq!(cq.symbol_name, "process_data");
	assert_eq!(cq.direction, CallerDirection::Callers);
}

#[test]
fn test_detect_callers_of() {
	let result =
		detect_caller_query("callers of handle_request");
	assert!(result.is_some());
	let cq = result.unwrap();
	assert_eq!(cq.symbol_name, "handle_request");
	assert_eq!(cq.direction, CallerDirection::Callers);
}

#[test]
fn test_detect_symbol_callers() {
	let result =
		detect_caller_query("parse_config callers");
	assert!(result.is_some());
	let cq = result.unwrap();
	assert_eq!(cq.symbol_name, "parse_config");
	assert_eq!(cq.direction, CallerDirection::Callers);
}

#[test]
fn test_detect_functions_that_call() {
	let result = detect_caller_query(
		"functions that call validate",
	);
	assert!(result.is_some());
	let cq = result.unwrap();
	assert_eq!(cq.symbol_name, "validate");
	assert_eq!(cq.direction, CallerDirection::Callers);
}

#[test]
fn test_detect_what_does_call() {
	let result =
		detect_caller_query("what does main call");
	assert!(result.is_some());
	let cq = result.unwrap();
	assert_eq!(cq.symbol_name, "main");
	assert_eq!(cq.direction, CallerDirection::Callees);
}

#[test]
fn test_no_match() {
	let result = detect_caller_query(
		"where is AuthService defined",
	);
	assert!(result.is_none());
}

//! Symbol extraction from queries

/// Extract potential symbol names from query
#[doc(hidden)]
pub fn extract_symbols(query: &str) -> Vec<String> {
	let mut symbols = Vec::new();

	for word in query.split_whitespace() {
		let clean = word.trim_matches(|chr: char| {
			!chr.is_alphanumeric() && chr != '_'
		});

		let is_symbol = is_camel_case(clean)
			|| is_snake_case(clean)
			|| is_screaming_case(clean);
		if is_symbol {
			symbols.push(clean.to_string());
		}
	}

	symbols
}

/// Check if word is CamelCase
fn is_camel_case(word: &str) -> bool {
	if word.len() < 2 {
		return false;
	}
	let chars: Vec<char> = word.chars().collect();
	let has_upper = chars[0].is_uppercase();
	let has_lower =
		chars.iter().any(|chr| chr.is_lowercase());
	let multi_upper = chars
		.iter()
		.filter(|chr| chr.is_uppercase())
		.count()
		>= 2;
	has_upper && has_lower && multi_upper
}

/// Check if word is snake_case
fn is_snake_case(word: &str) -> bool {
	let has_underscore = word.contains('_');
	let all_lower = word.chars().all(|chr| {
		chr.is_lowercase()
			|| chr.is_numeric()
			|| chr == '_'
	});
	has_underscore && all_lower && word.len() >= 3
}

/// Check if word is SCREAMING_CASE
fn is_screaming_case(word: &str) -> bool {
	let has_underscore = word.contains('_');
	let all_upper = word.chars().all(|chr| {
		chr.is_uppercase()
			|| chr.is_numeric()
			|| chr == '_'
	});
	has_underscore && all_upper && word.len() >= 3
}

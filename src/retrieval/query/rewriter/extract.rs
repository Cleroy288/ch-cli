//! Symbol extraction from queries

/// Extract potential symbol names from query
#[doc(hidden)]
pub fn extract_symbols(query: &str) -> Vec<String> {
	let mut symbols = Vec::new();

	for word in query.split_whitespace() {
		let clean = word.trim_matches(|c: char| {
			!c.is_alphanumeric() && c != '_'
		});

		if is_camel_case(clean) {
			symbols.push(clean.to_string());
		} else if is_snake_case(clean) {
			symbols.push(clean.to_string());
		} else if is_screaming_case(clean) {
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
	let has_lower = chars.iter().any(|c| c.is_lowercase());
	let multi_upper =
		chars.iter().filter(|c| c.is_uppercase()).count() >= 2;
	has_upper && has_lower && multi_upper
}

/// Check if word is snake_case
fn is_snake_case(word: &str) -> bool {
	let has_underscore = word.contains('_');
	let all_lower_or_digit = word.chars().all(|c| {
		c.is_lowercase() || c.is_numeric() || c == '_'
	});
	has_underscore && all_lower_or_digit && word.len() >= 3
}

/// Check if word is SCREAMING_CASE
fn is_screaming_case(word: &str) -> bool {
	let has_underscore = word.contains('_');
	let all_upper_or_digit = word.chars().all(|c| {
		c.is_uppercase() || c.is_numeric() || c == '_'
	});
	has_underscore && all_upper_or_digit && word.len() >= 3
}

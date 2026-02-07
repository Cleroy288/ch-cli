//! Conceptual to technical term mapping

/// Common conceptual to technical mappings
const CONCEPT_MAPPINGS: &[(&str, &str)] = &[
	("how does", "implementation"),
	("how do", "implementation"),
	("what is", "definition"),
	("where is", "location"),
	("authentication", "auth"),
	("authorization", "auth"),
	("database", "db"),
	("configuration", "config"),
	("initialization", "init"),
	("retrieval", "retrieve search"),
	("pipeline", "Pipeline flow"),
	("works", "implementation"),
	("work", "implementation"),
];

/// Map conceptual terms to technical/code terms
#[doc(hidden)]
pub fn map_concepts(query: &str) -> String {
	let lower = query.to_lowercase();
	let mut result = query.to_string();

	for (concept, technical) in CONCEPT_MAPPINGS {
		if lower.contains(concept) {
			add_technical_term(&mut result, technical);
		}
	}

	result
}

/// Add technical term to query if not already present
fn add_technical_term(result: &mut String, technical: &str) {
	if !result.to_lowercase().contains(technical) {
		result.push(' ');
		result.push_str(technical);
	}
}

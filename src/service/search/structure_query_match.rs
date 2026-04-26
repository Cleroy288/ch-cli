use super::structure_query::{
	extract_target, StructureQuery,
};

/// "modules in X" or "what modules are in X"
pub(crate) fn detect_modules_in(
	lower: &str,
) -> Option<StructureQuery> {
	if !lower.contains("modules in")
		&& !lower.contains("modules are in")
	{
		return None;
	}
	let target = extract_target(lower, "in ")?;
	Some(StructureQuery {
		target,
		is_module: true,
	})
}

/// "what's in X" or "what is in X"
pub(crate) fn detect_whats_in(
	lower: &str,
) -> Option<StructureQuery> {
	if !lower.contains("what's in")
		&& !lower.contains("what is in")
	{
		return None;
	}
	let target = extract_target(lower, "in ")?;
	Some(StructureQuery {
		target,
		is_module: false,
	})
}

/// "structure of X"
pub(crate) fn detect_structure_of(
	lower: &str,
) -> Option<StructureQuery> {
	if !lower.contains("structure of") {
		return None;
	}
	let target =
		extract_target(lower, "structure of ")?;
	Some(StructureQuery {
		target,
		is_module: true,
	})
}

/// "X structure" (word before "structure")
pub(crate) fn detect_x_structure(
	lower: &str,
) -> Option<StructureQuery> {
	if !lower.contains(" structure") {
		return None;
	}
	let words: Vec<&str> =
		lower.split_whitespace().collect();
	let target = words
		.windows(2)
		.find(|pair| pair[1] == "structure")?;
	let name = target[0].to_string();
	let skip = name == "module"
		|| name == "directory"
		|| name.len() <= 2;
	if skip {
		return None;
	}
	Some(StructureQuery {
		target: name,
		is_module: true,
	})
}

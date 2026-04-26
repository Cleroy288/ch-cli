use super::structure_search::{ModuleInfo, SubmoduleDecl};

pub(super) fn parse_module_file(
	path: &std::path::Path,
) -> Option<ModuleInfo> {
	let content =
		std::fs::read_to_string(path).ok()?;
	let mut submodules = Vec::new();
	let mut reexports = Vec::new();
	let mut doc_lines: Vec<String> = Vec::new();

	for line in content.lines() {
		process_line(
			line.trim(),
			&mut doc_lines,
			&mut submodules,
			&mut reexports,
		);
	}
	Some(ModuleInfo {
		path: path.to_path_buf(),
		submodules,
		reexports,
	})
}

/// Process a single line from a module file
fn process_line(
	trimmed: &str,
	doc_lines: &mut Vec<String>,
	submodules: &mut Vec<SubmoduleDecl>,
	reexports: &mut Vec<String>,
) {
	if is_doc_comment(trimmed) {
		let txt = trimmed
			.trim_start_matches("///")
			.trim_start_matches("//!")
			.trim();
		doc_lines.push(txt.to_string());
		return;
	}
	if is_mod_decl(trimmed) {
		parse_mod_decl(
			trimmed, doc_lines, submodules,
		);
	} else if trimmed.starts_with("pub use ") {
		let reexport = trimmed
			.trim_start_matches("pub use ")
			.trim_end_matches(';')
			.to_string();
		reexports.push(reexport);
	}
	if !trimmed.is_empty() {
		doc_lines.clear();
	}
}

fn is_doc_comment(line: &str) -> bool {
	line.starts_with("///")
		|| line.starts_with("//!")
}

fn is_mod_decl(line: &str) -> bool {
	line.starts_with("pub mod ")
		|| line.starts_with("mod ")
}

fn parse_mod_decl(
	trimmed: &str,
	doc_lines: &mut Vec<String>,
	submodules: &mut Vec<SubmoduleDecl>,
) {
	let is_public = trimmed.starts_with("pub ");
	let name = trimmed
		.trim_start_matches("pub mod ")
		.trim_start_matches("mod ")
		.trim_end_matches(';')
		.trim()
		.to_string();
	if name.is_empty() || name.contains('{') {
		return;
	}
	let doc = if doc_lines.is_empty() {
		None
	} else {
		Some(doc_lines.join(" "))
	};
	submodules.push(SubmoduleDecl {
		name,
		is_public,
		doc,
	});
	doc_lines.clear();
}

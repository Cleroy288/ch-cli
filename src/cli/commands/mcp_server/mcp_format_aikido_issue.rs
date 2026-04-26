use crate::domain::aikido::Issue;

/// Max chars for file path display
const MAX_FILE_DISPLAY: usize = 40;

/// Format one issue as a compact line
pub(super) fn issue_line(
	issue: &Issue,
) -> String {
	let mut parts = vec![format!(
		"#{} [{}]", issue.id, issue.issue_type,
	)];
	push_part(
		&mut parts, "", &issue.cve_id,
	);
	push_part(
		&mut parts,
		"pkg:",
		&issue.affected_package,
	);
	add_file_part(&mut parts, issue);
	push_part(
		&mut parts,
		"image:",
		&issue.container_repo_name,
	);
	if !issue.patched_versions.is_empty() {
		parts.push(format!(
			"fix:{}",
			issue.patched_versions.join(","),
		));
	}
	parts.join(" | ")
}

fn push_part(
	parts: &mut Vec<String>,
	prefix: &str,
	value: &str,
) {
	if !value.is_empty() {
		parts.push(format!("{prefix}{value}"));
	}
}

fn add_file_part(
	parts: &mut Vec<String>,
	issue: &Issue,
) {
	if issue.affected_file.is_empty() {
		return;
	}
	let file = truncate(
		&issue.affected_file,
		MAX_FILE_DISPLAY,
	);
	match issue.start_line {
		Some(line) => parts.push(
			format!("file:{file}:{line}"),
		),
		None => parts.push(
			format!("file:{file}"),
		),
	}
}

fn truncate(
	s: &str,
	max: usize,
) -> String {
	if s.len() <= max {
		return s.into();
	}
	let cut: String =
		s.chars().take(max - 1).collect();
	format!("{cut}…")
}

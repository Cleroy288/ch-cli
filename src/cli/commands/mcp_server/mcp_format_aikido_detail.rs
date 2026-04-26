use crate::domain::aikido::{
	IssueCounts, IssueDetail,
};

/// Format issue counts by severity
pub fn format_counts(
	counts: &IssueCounts,
) -> String {
	let c = &counts.issues;
	format!(
		"Total: {}\n\
		 \x20 critical: {}\n\
		 \x20 high: {}\n\
		 \x20 medium: {}\n\
		 \x20 low: {}",
		c.all, c.critical, c.high,
		c.medium, c.low,
	)
}

/// Format single issue detail
pub fn format_detail(
	d: &IssueDetail,
) -> String {
	let container =
		container_suffix(&d.container_repo_name);
	format!(
		"Issue #{id}\n\
		 Type: {typ}\n\
		 Severity: {sev} (score: {score})\n\
		 Status: {status}\n\
		 Source: {repo}{container}\n\
		 CVE: {cve}\n\
		 Package: {pkg}\n\
		 File: {file}\n\
		 Reachability: {reach}\n\
		 Fix: {fix}",
		id = d.id,
		typ = d.issue_type,
		sev = d.severity,
		score = d.severity_score,
		status = d.status,
		repo = d.code_repo_name,
		cve = or_dash(&d.cve_id),
		pkg = or_dash(&d.affected_package),
		file = or_dash(&d.affected_file),
		reach = d.reachability_status,
		fix = or_dash(&d.how_to_fix),
	)
}

/// Dash placeholder for empty strings
pub(super) fn or_dash(s: &str) -> &str {
	if s.is_empty() { "—" } else { s }
}

fn container_suffix(name: &str) -> String {
	if name.is_empty() {
		String::new()
	} else {
		format!(" (container: {name})")
	}
}

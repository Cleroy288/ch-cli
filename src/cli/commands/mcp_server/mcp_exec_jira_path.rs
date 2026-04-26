use serde_json::Value;

use super::mcp_helpers::extract_string;
use super::mcp_helpers_bb::query_sep;

/// Substitute placeholders in Jira path
pub fn build_jira_path(
	template: &str,
	args: &Value,
) -> String {
	let mut path = template.to_string();
	let params =
		["issue_key", "project_key"];
	for key in &params {
		let placeholder =
			format!("{{{key}}}");
		if let Some(val) =
			extract_string(args, key)
		{
			path = path.replace(
				&placeholder, &val,
			);
		}
	}
	append_jira_params(&mut path, args);
	path
}

/// Append optional query params
pub fn append_jira_params(
	path: &mut String,
	args: &Value,
) {
	append_param(path, args, "fields");
	append_num(path, args, "max_results",
		"maxResults");
	append_num(path, args, "start_at",
		"startAt");
}

/// Append a string query param
fn append_param(
	path: &mut String,
	args: &Value,
	key: &str,
) {
	if let Some(val) = extract_string(args, key) {
		path.push(query_sep(path));
		path.push_str(
			&format!("{key}={val}"),
		);
	}
}

/// Append a numeric query param with rename
fn append_num(
	path: &mut String,
	args: &Value,
	key: &str,
	api_key: &str,
) {
	if let Some(num) =
		args.get(key).and_then(Value::as_u64)
	{
		path.push(query_sep(path));
		path.push_str(
			&format!("{api_key}={num}"),
		);
	}
}

/// Simple percent-encoding for JQL
pub fn urlencoded(input: &str) -> String {
	input
		.replace('%', "%25")
		.replace(' ', "%20")
		.replace('=', "%3D")
		.replace('&', "%26")
		.replace('+', "%2B")
		.replace('"', "%22")
}

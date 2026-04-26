use serde_json::{Value, json};

/// Tool: fetch open security issues
pub fn aikido_get_issues_def() -> Value {
	json!({
		"name": "aikido_get_issues",
		"description":
			"Fetch open security issues from \
			 Aikido, grouped by severity",
		"inputSchema": {
			"type": "object",
			"properties": {
				"repo_name": {
					"type": "string",
					"description":
						"Filter by code repo name",
				},
				"container_name": {
					"type": "string",
					"description":
						"Filter by container image",
				},
				"issue_type": {
					"type": "string",
					"description":
						"Filter by type (sast, \
						 open_source, \
						 docker_container)",
				},
				"severities": {
					"type": "string",
					"description":
						"Comma-separated severity \
						 filter (critical,high,\
						 medium,low)",
				},
				"format": {
					"type": "string",
					"description":
						"Output: grouped (default)\
						 , flat, or summary",
				},
			},
		},
	})
}

/// Tool: list monitored code repos
pub fn aikido_list_repos_def() -> Value {
	json!({
		"name": "aikido_list_repos",
		"description":
			"List code repositories monitored \
			 by Aikido Security",
		"inputSchema": {
			"type": "object",
			"properties": {},
		},
	})
}

/// Tool: list container image repos
pub fn aikido_list_containers_def() -> Value {
	json!({
		"name": "aikido_list_containers",
		"description":
			"List container image repositories \
			 monitored by Aikido Security",
		"inputSchema": {
			"type": "object",
			"properties": {},
		},
	})
}

/// Tool: get issue counts by severity
pub fn aikido_issue_counts_def() -> Value {
	json!({
		"name": "aikido_issue_counts",
		"description":
			"Get issue counts by severity, \
			 optionally filtered by repo",
		"inputSchema": {
			"type": "object",
			"properties": {
				"repo_name": {
					"type": "string",
					"description":
						"Filter by code repo name",
				},
				"container_repo_id": {
					"type": "number",
					"description":
						"Filter by container \
						 repo ID",
				},
			},
		},
	})
}

/// Tool: get single issue detail
pub fn aikido_get_issue_def() -> Value {
	json!({
		"name": "aikido_get_issue",
		"description":
			"Get detailed info about a specific \
			 Aikido issue by ID",
		"inputSchema": {
			"type": "object",
			"properties": {
				"issue_id": {
					"type": "number",
					"description": "The issue ID",
				},
			},
			"required": ["issue_id"],
		},
	})
}

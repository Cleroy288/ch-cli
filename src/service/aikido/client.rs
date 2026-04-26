use reqwest::blocking::Client;

use crate::domain::aikido::{
	ContainerRepo, Issue, IssueFilters, Repo,
};
use crate::domain::credentials
	::AikidoCredentials;
use crate::domain::errors::aikido::AikidoResult;

use super::client_auth::AuthManager;
use super::client_request::push_filter;

/// Blocking Aikido Security API client
pub struct AikidoClient {
	pub(super) http: Client,
	pub(super) auth: AuthManager,
}

impl AikidoClient {
	pub fn new(
		creds: AikidoCredentials,
	) -> Self {
		let http = Client::new();
		let auth = AuthManager::new(
			http.clone(),
			creds.client_id,
			creds.client_secret,
		);
		Self { http, auth }
	}

	/// Fetch open issues with filters
	pub fn fetch_issues(
		&self,
		filters: &IssueFilters,
	) -> AikidoResult<Vec<Issue>> {
		let mut params = vec![
			("filter_status", "open".into()),
		];
		append_issue_filters(
			&mut params, filters,
		);
		self.get_json(
			"/public/v1/issues/export",
			&params,
		)
	}

	/// List monitored code repositories
	pub fn fetch_repos(
		&self,
	) -> AikidoResult<Vec<Repo>> {
		self.paginate(
			"/public/v1/repositories/code",
		)
	}

	/// List container image repositories
	pub fn fetch_containers(
		&self,
	) -> AikidoResult<Vec<ContainerRepo>> {
		self.paginate("/public/v1/containers")
	}
}

/// Build query params from issue filters
fn append_issue_filters<'a>(
	params: &mut Vec<(&'a str, String)>,
	filters: &IssueFilters,
) {
	push_filter(
		params,
		"filter_code_repo_name",
		&filters.repo_name,
	);
	push_filter(
		params,
		"filter_container_repo_name",
		&filters.container_name,
	);
	push_filter(
		params,
		"filter_issue_type",
		&filters.issue_type,
	);
	if let Some(ref sevs) = filters.severities {
		params.push((
			"filter_severities",
			sevs.join(","),
		));
	}
}

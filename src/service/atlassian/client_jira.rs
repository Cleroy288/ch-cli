use reqwest::blocking::Client;

use crate::domain::errors::atlassian::{
	AtlassianError, AtlassianResult,
};

use super::client_jira_agile;
use super::types::JiraCredentials;

const JIRA_API_BASE: &str =
	"https://api.atlassian.com/ex/jira";
const JIRA_API_VERSION: &str = "/rest/api/3";

pub struct JiraClient {
	http: Client,
	pub(crate) creds: JiraCredentials,
}

impl JiraClient {
	pub fn new(creds: JiraCredentials) -> Self {
		Self {
			http: Client::new(),
			creds,
		}
	}

	pub fn get(
		&self,
		path: &str,
	) -> AtlassianResult<String> {
		let url = format!(
			"{JIRA_API_BASE}/{}{JIRA_API_VERSION}\
			{path}",
			self.creds.cloud_id,
		);
		self.do_get(&url, "Jira API")
	}

	/// Falls back to direct site URL on 401.
	pub fn get_agile(
		&self,
		path: &str,
	) -> AtlassianResult<String> {
		let gateway =
			client_jira_agile::try_agile_gateway(
				self, path,
			);
		if gateway.is_ok() {
			return gateway;
		}
		client_jira_agile::try_agile_direct(
			self, path,
		)
	}

	pub(crate) fn do_get(
		&self,
		url: &str,
		label: &str,
	) -> AtlassianResult<String> {
		let resp = self
			.http
			.get(url)
			.basic_auth(
				&self.creds.email,
				Some(&self.creds.token),
			)
			.send()
			.map_err(|e| {
				AtlassianError::Http(e.to_string())
			})?;
		let status = resp.status();
		if !status.is_success() {
			return Err(AtlassianError::Status(
				format!("{label}: {status}"),
			));
		}
		resp.text().map_err(|e| {
			AtlassianError::Http(e.to_string())
		})
	}
}

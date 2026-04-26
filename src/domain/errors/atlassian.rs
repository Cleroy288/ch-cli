/// Errors from Atlassian API calls (Jira + BB)
#[derive(Debug, thiserror::Error)]
pub enum AtlassianError {
	#[error("HTTP request: {0}")]
	Http(String),

	#[error("{0}")]
	Status(String),

	#[error("JSON: {0}")]
	Json(String),
}

pub type AtlassianResult<T> =
	Result<T, AtlassianError>;

impl From<serde_json::Error> for AtlassianError {
	fn from(err: serde_json::Error) -> Self {
		Self::Json(err.to_string())
	}
}


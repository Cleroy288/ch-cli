/// Aikido Security API error types
#[derive(Debug, thiserror::Error)]
pub enum AikidoError {
	#[error("HTTP request: {0}")]
	Http(String),

	#[error("{0}")]
	Status(String),

	#[error("JSON: {0}")]
	Json(String),

	#[error("Auth: {0}")]
	Auth(String),

	#[error("Parse: {0}")]
	Parse(String),
}

pub type AikidoResult<T> = Result<T, AikidoError>;

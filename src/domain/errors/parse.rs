#[derive(Debug, thiserror::Error)]
pub enum ParseError {
	#[error("{0}")]
	Io(#[from] std::io::Error),

	#[error("parser language setup: {0}")]
	Language(String),

	#[error("parse failed: {0}")]
	ParseFailed(String),

	#[error("query compilation: {0}")]
	Query(String),
}

pub type ParseResult<T> = Result<T, ParseError>;

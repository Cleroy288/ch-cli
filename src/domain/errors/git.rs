pub type GitResult<T> = Result<T, GitError>;

#[derive(Debug, thiserror::Error)]
pub enum GitError {
	#[error("git command failed: {0}")]
	Command(String),

	#[error("failed to parse git log: {0}")]
	Parse(String),

	#[error("no git repository found")]
	NoRepo,

	#[error("git I/O error: {0}")]
	Io(#[from] std::io::Error),
}

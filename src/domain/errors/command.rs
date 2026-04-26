use super::index::IndexError;
use super::memory::MemoryError;
use super::search::SearchError;

pub type CommandResult = Result<(), CommandError>;

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
	#[error("indexing unavailable: {0}")]
	IndexUnavailable(String),

	#[error("{0}")]
	Index(#[from] IndexError),

	#[error("{0}")]
	Search(#[from] SearchError),

	#[error("{0}")]
	Io(#[from] std::io::Error),

	#[error("symbol not found: {0}")]
	SymbolNotFound(String),

	#[error("invalid symbol kind: {0}")]
	InvalidKind(String),

	#[error("{0}")]
	Memory(#[from] MemoryError),
}

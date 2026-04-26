pub use crate::domain::errors::search::{
	SearchError, SearchResult,
};

impl From<tantivy::TantivyError> for SearchError {
	fn from(err: tantivy::TantivyError) -> Self {
		Self::Tantivy(err.to_string())
	}
}

impl From<tantivy::query::QueryParserError>
	for SearchError
{
	fn from(
		err: tantivy::query::QueryParserError,
	) -> Self {
		Self::QueryParse(err.to_string())
	}
}

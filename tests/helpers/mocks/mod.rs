//! Mock implementations for service traits.
//!
//! Each mock records calls and returns preset
//! values via RefCell<VecDeque>. Used for testing
//! handlers and services in isolation.

pub mod mock_index;
pub mod mock_search;
mod mock_search_trait;

pub use mock_index::MockIndexService;
pub use mock_search::MockSearchService;

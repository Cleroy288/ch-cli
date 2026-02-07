//! Core TieredQueryExpander Implementation
//!
//! Provides the main expander structure and constructors.

use crate::indexer::SemanticGraph;
use crate::retrieval::daemon::DaemonClient;

use super::super::super::fast_path::FastPathParser;
use super::super::config::TieredConfig;

/// Tiered query expander with fast-path optimization
pub struct TieredQueryExpander<'a> {
	/// daemon client for LLM expansion
	pub(crate) daemon: &'a DaemonClient,
	/// semantic graph for validation (optional)
	pub(crate) graph: Option<&'a SemanticGraph>,
	/// fast-path parser
	pub(crate) parser: FastPathParser,
	/// configuration
	pub(crate) config: TieredConfig,
}

impl<'a> TieredQueryExpander<'a> {
	/// Create a new tiered expander
	pub fn new(
		daemon: &'a DaemonClient,
		graph: Option<&'a SemanticGraph>,
	) -> Self {
		Self {
			daemon,
			graph,
			parser: FastPathParser::new(),
			config: TieredConfig::default(),
		}
	}

	/// Create with custom configuration
	pub fn with_config(
		daemon: &'a DaemonClient,
		graph: Option<&'a SemanticGraph>,
		config: TieredConfig,
	) -> Self {
		Self {
			daemon,
			graph,
			parser: FastPathParser::new(),
			config,
		}
	}
}

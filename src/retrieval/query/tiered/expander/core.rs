//! Core TieredQueryExpander Implementation
//!
//! Provides the main expander structure and constructors.

use crate::indexer::SemanticGraph;
use crate::retrieval::daemon::DaemonClient;

use super::super::super::fast_path::FastPathParser;
use super::super::config::TieredConfig;

/// Tiered query expander with fast-path optimization
pub struct TieredQueryExpander<'ctx> {
	/// daemon client for LLM expansion
	pub(crate) daemon: &'ctx DaemonClient,
	/// semantic graph for validation (optional)
	pub(crate) graph: Option<&'ctx SemanticGraph>,
	/// fast-path parser
	pub(crate) parser: FastPathParser,
	/// configuration
	pub(crate) config: TieredConfig,
}

impl<'ctx> TieredQueryExpander<'ctx> {
	/// Create a new tiered expander
	pub fn new(
		daemon: &'ctx DaemonClient,
		graph: Option<&'ctx SemanticGraph>,
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
		daemon: &'ctx DaemonClient,
		graph: Option<&'ctx SemanticGraph>,
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

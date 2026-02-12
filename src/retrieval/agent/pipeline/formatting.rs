//! Result formatting utilities.

use crate::indexer::Symbol;
use crate::retrieval::context::{ContextConfig, ContextExpander};
use crate::retrieval::hybrid::HybridSearchResult;
use crate::retrieval::{RetrievalError, RetrievalResult};

use super::core::RetrievalPipeline;

impl RetrievalPipeline {
	/// Expand context for search results
	pub(super) fn expand_context(
		&self,
		results: &[HybridSearchResult],
	) -> RetrievalResult<String> {
		let symbols: Vec<_> = results
			.iter()
			.map(|res| (*res.symbol).clone())
			.collect();

		if let Some(ref graph) = self.graph {
			let config = ContextConfig::default();
			let max_tokens = self.config.max_tokens;
			let expander =
				ContextExpander::with_config(graph, config, max_tokens);
			Ok(expander.expand_to_xml(&symbols))
		} else {
			Ok(self.format_simple(results))
		}
	}

	/// Simple formatting without context expansion
	pub(super) fn format_simple(
		&self,
		results: &[HybridSearchResult],
	) -> String {
		let mut output = String::from("<results>\n");

		for result in results {
			output.push_str(&format!(
				"  <symbol kind=\"{}\" name=\"{}\" \
				file=\"{}\" line=\"{}\"/>\n",
				result.symbol.kind,
				result.symbol.name,
				result.symbol.location.file.display(),
				result.symbol.location.line
			));
		}

		output.push_str("</results>");
		output
	}

	/// Get more context for specific symbols
	pub fn get_more_context(
		&self,
		symbol_names: &[String],
	) -> RetrievalResult<String> {
		let matching =
			self.find_matching_symbols(symbol_names)?;

		let graph = self.graph.as_ref().ok_or_else(|| {
			RetrievalError::Embedding(
				"No semantic graph".to_string(),
			)
		})?;

		Ok(expand_deep_context(
			graph,
			&matching,
			self.config.max_tokens * 2,
		))
	}

	/// Find symbols matching the given names
	fn find_matching_symbols(
		&self,
		symbol_names: &[String],
	) -> RetrievalResult<Vec<Symbol>> {
		let symbols = self.symbols.as_ref().ok_or_else(|| {
			RetrievalError::Embedding(
				"Pipeline not initialized".to_string(),
			)
		})?;

		Ok(symbols
			.iter()
			.filter(|sym| symbol_names.contains(&sym.name))
			.cloned()
			.collect())
	}
}

/// Build deep context expansion for matched symbols
fn expand_deep_context(
	graph: &crate::indexer::SemanticGraph,
	matching: &[Symbol],
	max_tokens: usize,
) -> String {
	let config = ContextConfig {
		max_callers: 10,
		max_callees: 10,
		max_usages_per_symbol: 30,
		context_lines_before: 5,
		context_lines_after: 20,
		include_parent: true,
		include_related_types: true,
	};
	let expander =
		ContextExpander::with_config(graph, config, max_tokens);
	expander.expand_to_xml(matching)
}

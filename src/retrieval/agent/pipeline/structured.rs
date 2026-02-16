//! Structured retrieval pipeline logic.

use std::io::Write;

use crate::indexer::triple_search::TripleLimits;
use crate::retrieval::{RetrievalError, RetrievalResult};

use super::super::output::StructuredOutput;
use super::super::RetrievalOutput;
use super::core::RetrievalPipeline;
use super::query_expansion::expand_for_structured;

impl RetrievalPipeline {
	/// Run the full retrieval pipeline
	pub fn retrieve(
		&mut self,
		query: &str,
	) -> RetrievalResult<RetrievalOutput> {
		if self.symbols.is_none() {
			self.initialize()?;
		}

		let search_spec =
			self.expand_for_retrieval(query);
		log_search_spec(&search_spec);

		let results =
			self.search_and_rerank(query, &search_spec)?;
		let xml_output =
			self.format_results(&results)?;

		let token_count = xml_output.len() / 4;
		Ok(RetrievalOutput {
			query: query.to_string(),
			search_spec,
			xml_output,
			result_count: results.len(),
			token_count,
			has_more: results.len()
				>= self.config.max_results,
		})
	}

	/// Search, filter, and rerank results
	fn search_and_rerank(
		&self,
		query: &str,
		search_spec: &crate::retrieval::daemon::protocol::SearchSpec,
	) -> RetrievalResult<Vec<crate::retrieval::hybrid::HybridSearchResult>> {
		let fetch_limit = if self.config.flags.rerank {
			self.config.max_results * 3
		} else {
			self.config.max_results
		};

		let mut results =
			self.search(search_spec, fetch_limit)?;
		let _ = writeln!(
			std::io::stderr().lock(),
			"[pipeline] Found {} candidates",
			results.len()
		);

		self.filter_doc_chunks(
			&search_spec.intent, &mut results,
		);
		self.apply_reranking(
			query, search_spec, &mut results,
		);
		Ok(results)
	}

	/// Apply reranking if enabled and truncate
	fn apply_reranking(
		&self,
		query: &str,
		spec: &crate::retrieval::daemon::protocol::SearchSpec,
		results: &mut Vec<crate::retrieval::hybrid::HybridSearchResult>,
	) {
		if !self.config.flags.rerank
			|| results.is_empty()
		{
			return;
		}
		let _ = writeln!(
			std::io::stderr().lock(),
			"[pipeline] Reranking..."
		);
		let taken = std::mem::take(results);
		*results = self.rerank_safe(
			query, spec, taken,
		);
		results.truncate(self.config.max_results);
	}

	/// Format results with optional context expansion
	fn format_results(
		&self,
		results: &[crate::retrieval::hybrid::HybridSearchResult],
	) -> RetrievalResult<String> {
		if self.config.flags.expand_context {
			let _ = writeln!(
				std::io::stderr().lock(),
				"[pipeline] Expanding context..."
			);
			self.expand_context(results)
		} else {
			Ok(self.format_simple(results))
		}
	}

	/// Run structured pipeline retrieval
	pub fn retrieve_structured(
		&mut self,
		query: &str,
	) -> RetrievalResult<StructuredOutput> {
		if self.symbols.is_none() {
			self.initialize()?;
		}

		let spec =
			expand_for_structured(self, query);
		let results =
			self.run_triple_search(&spec)?;

		let intent_str = format!("{:?}", spec.intent);
		let mut output = StructuredOutput::new(
			query.to_string(),
			intent_str,
		);
		self.populate_structured(
			&results, &mut output,
		);

		log_structured_counts(&output);
		Ok(output)
	}

	/// Execute triple hybrid search with limits
	fn run_triple_search(
		&self,
		spec: &crate::retrieval::daemon::protocol::SearchSpec,
	) -> RetrievalResult<crate::retrieval::hybrid::triple::TripleHybridResults> {
		let hybrid = self
			.triple_hybrid
			.as_ref()
			.ok_or_else(|| {
				RetrievalError::Embedding(
					"Structured pipeline not init"
						.to_string(),
				)
			})?;

		let limits = TripleLimits {
			code: self.config.max_code_results,
			doc: self.config.max_doc_results,
			notes: self.config.max_notes_results,
		};
		hybrid.search(
			&spec.original_query, &limits,
		)
	}

	/// Populate structured output from search results
	fn populate_structured(
		&self,
		results: &crate::retrieval::hybrid::triple::TripleHybridResults,
		output: &mut StructuredOutput,
	) {
		for code_hit in &results.code_results {
			if let Some(code_result) =
				self.build_code_result(&code_hit.symbol)
			{
				output.add_code(code_result);
			}
		}
		for doc_hit in &results.doc_results {
			output.add_doc(
				self.build_doc_result(&doc_hit.symbol),
			);
		}
		for note_hit in &results.notes_results {
			output.add_notes(
				self.build_notes_result(&note_hit.symbol),
			);
		}
	}

	/// Filter doc chunks for Understand intent
	fn filter_doc_chunks(
		&self,
		intent: &crate::retrieval::daemon::protocol::QueryIntent,
		results: &mut Vec<
			crate::retrieval::hybrid::HybridSearchResult,
		>,
	) {
		use crate::retrieval::daemon::protocol::QueryIntent;
		if !matches!(intent, QueryIntent::Understand) {
			return;
		}
		let before = results.len();
		results.retain(|res| {
			let is_doc = res.symbol.kind
				== crate::indexer::SymbolKind::DocumentChunk;
			let in_doc = res
				.symbol
				.location
				.file
				.to_string_lossy()
				.contains("/doc/");
			!(is_doc && in_doc)
		});
		if results.len() < before {
			let _ = writeln!(
				std::io::stderr().lock(),
				"[pipeline] Filtered {} doc chunks",
				before - results.len()
			);
		}
	}
}

/// Log structured output result counts
fn log_structured_counts(
	output: &StructuredOutput,
) {
	let _ = writeln!(
		std::io::stderr().lock(),
		"[pipeline] Results: {} code, {} doc, \
		{} notes",
		output.code_context.len(),
		output.doc_context.len(),
		output.notes_context.len()
	);
}

/// Log search spec details to stderr
fn log_search_spec(
	spec: &crate::retrieval::daemon::protocol::SearchSpec,
) {
	let _ = writeln!(
		std::io::stderr().lock(),
		"[pipeline] Search spec: {:?} symbols, \
		intent: {:?}",
		spec.symbol_names.len(),
		spec.intent
	);
}

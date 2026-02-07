//! Structured retrieval pipeline logic.

use crate::retrieval::query::{
	fallback_parse, TieredConfig, TieredQueryExpander,
};
use crate::retrieval::{RetrievalError, RetrievalResult};

use super::super::output::StructuredOutput;
use super::super::RetrievalOutput;
use super::core::RetrievalPipeline;

impl RetrievalPipeline {
	/// Run the full retrieval pipeline
	pub fn retrieve(
		&mut self,
		query: &str,
	) -> RetrievalResult<RetrievalOutput> {
		// Ensure initialized
		if self.symbols.is_none() {
			self.initialize()?;
		}

		// Step 1: Query Expansion (tiered or direct LLM)
		let search_spec = if self.config.expand_query {
			if self.config.tiered_expansion {
				// Use tiered expansion with fast-path
				let tiered_config = TieredConfig {
					confidence_threshold:
						self.config.fast_path_threshold,
					importance_threshold:
						self.config.importance_threshold,
					existence_threshold: 0.5,
				};
				let expander = TieredQueryExpander::with_config(
					&self.daemon,
					self.graph.as_ref(),
					tiered_config,
				);
				match expander.expand(query) {
					Ok(result) => {
						eprintln!(
							"[pipeline] Tiered expansion: \
							{:?} in {}ms",
							result.tier_used, result.time_ms
						);
						result.spec
					}
					Err(e) => {
						eprintln!(
							"[pipeline] Tiered expansion \
							failed: {}, using fallback",
							e
						);
						self.fallback_query_expansion(query)
					}
				}
			} else {
				// Direct LLM expansion with safe timeout
				eprintln!("[pipeline] Expanding query via LLM...");
				self.expand_query_safe(query)
			}
		} else {
			// Simple spec without expansion
			use crate::retrieval::daemon::protocol::QueryIntent;
			crate::retrieval::daemon::protocol::SearchSpec {
				original_query: query.to_string(),
				symbol_names: query
					.split_whitespace()
					.map(String::from)
					.collect(),
				intent: QueryIntent::Search,
				file_filters: Vec::new(),
				context_hints: Vec::new(),
			}
		};

		eprintln!(
			"[pipeline] Search spec: {:?} symbols, intent: {:?}",
			search_spec.symbol_names.len(),
			search_spec.intent
		);

		// Step 2: Hybrid Search
		let fetch_limit = if self.config.rerank {
			self.config.max_results * 3
		} else {
			self.config.max_results
		};

		let mut results = self.search(&search_spec, fetch_limit)?;
		eprintln!("[pipeline] Found {} candidates", results.len());

		// For Understand intent, filter out documentation
		use crate::retrieval::daemon::protocol::QueryIntent;
		if matches!(search_spec.intent, QueryIntent::Understand) {
			let before_count = results.len();
			results.retain(|r| {
				// Keep everything except DocumentChunk from doc/
				let is_doc_chunk = r.symbol.kind
					== crate::indexer::SymbolKind::DocumentChunk;
				let in_doc_dir = r
					.symbol
					.location
					.file
					.to_string_lossy()
					.contains("/doc/");
				!(is_doc_chunk && in_doc_dir)
			});
			if results.len() < before_count {
				eprintln!(
					"[pipeline] Filtered {} doc chunks for \
					Understand intent",
					before_count - results.len()
				);
			}
		}

		// Step 3: Reranking (with graceful degradation)
		if self.config.rerank && !results.is_empty() {
			eprintln!("[pipeline] Reranking...");
			results = self.rerank_safe(query, &search_spec, results);
			results.truncate(self.config.max_results);
		}

		// Step 4: Context Expansion
		let xml_output = if self.config.expand_context {
			eprintln!("[pipeline] Expanding context...");
			self.expand_context(&results)?
		} else {
			self.format_simple(&results)
		};

		let token_count = xml_output.len() / 4;

		Ok(RetrievalOutput {
			query: query.to_string(),
			search_spec,
			xml_output,
			result_count: results.len(),
			token_count,
			has_more: results.len() >= self.config.max_results,
		})
	}

	/// Run structured pipeline retrieval (code, doc, notes)
	/// Returns StructuredOutput with full file content
	pub fn retrieve_structured(
		&mut self,
		query: &str,
	) -> RetrievalResult<StructuredOutput> {
		if self.symbols.is_none() {
			self.initialize()?;
		}

		let search_spec = self.expand_query_for_structured(query);
		let intent_str = format!("{:?}", search_spec.intent);

		let hybrid = self.triple_hybrid.as_ref().ok_or_else(|| {
			RetrievalError::Embedding(
				"Structured pipeline not initialized".to_string(),
			)
		})?;

		let results = hybrid.search(
			&search_spec.original_query,
			self.config.max_code_results,
			self.config.max_doc_results,
			self.config.max_notes_results,
		)?;

		let mut output = StructuredOutput::new(
			query.to_string(),
			intent_str,
		);

		// Process code results with full file content
		for result in &results.code_results {
			if let Some(code_result) =
				self.build_code_result(&result.symbol)
			{
				output.add_code(code_result);
			}
		}

		// Process doc results
		for result in &results.doc_results {
			output.add_doc(self.build_doc_result(&result.symbol));
		}

		// Process notes results
		for result in &results.notes_results {
			output
				.add_notes(self.build_notes_result(&result.symbol));
		}

		eprintln!(
			"[pipeline] Results: {} code, {} doc, {} notes",
			output.code_context.len(),
			output.doc_context.len(),
			output.notes_context.len()
		);

		Ok(output)
	}

	/// Expand query for structured pipeline (simplified)
	fn expand_query_for_structured(
		&self,
		query: &str,
	) -> crate::retrieval::daemon::protocol::SearchSpec {
		if self.config.expand_query {
			self.expand_query_safe(query)
		} else {
			fallback_parse(query)
		}
	}
}

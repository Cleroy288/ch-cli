//! Retrieve command implementation (agentic pipeline).
//!
//! Thin handler that delegates to RetrievalService.

use std::time::Instant;

use crate::retrieval::RetrievalOutput;
use crate::service::retrieval::types::RetrievalRequest;
use crate::service::{
	DefaultRetrievalService, RetrievalService,
};

use super::error::{CommandError, CommandResult};

/// Execute the `retrieve` command
pub fn retrieve_command(
	query: &str,
	limit: usize,
	max_tokens: usize,
	no_expand: bool,
	no_rerank: bool,
	no_context: bool,
	xml: bool,
	structured: bool,
	threshold: f32,
	min_results: usize,
) -> CommandResult {
	let start = Instant::now(); // timing
	let service = DefaultRetrievalService::new();
	let req = RetrievalRequest {
		query: query.to_string(),
		limit,
		max_tokens,
		no_expand,
		no_rerank,
		no_context,
		xml,
		structured,
		threshold,
		min_results,
		project_path: ".".to_string(),
	};

	if structured {
		return format_structured(
			&service, &req, xml, start,
		);
	}

	if !xml {
		println!(
			"Initializing retrieval pipeline...",
		);
	}
	let output = service.retrieve(&req)?;
	format_output(&output, xml, start);

	Ok(())
}

/// Format and print retrieval output
fn format_output(
	output: &RetrievalOutput,
	xml: bool,
	start: Instant,
) {
	if xml {
		println!("{}", output.xml_output);
		return;
	}

	println!("\n{}", "=".repeat(60));
	println!("RETRIEVAL RESULTS");
	println!("{}", "=".repeat(60));
	println!("\nQuery: {}", output.query);
	println!("Intent: {:?}", output.search_spec.intent);

	if !output.search_spec.symbol_names.is_empty() {
		println!(
			"Symbols: {}",
			output.search_spec.symbol_names.join(", "),
		);
	}

	println!("\nResults: {} found", output.result_count);
	println!("Tokens: ~{}", output.token_count);

	if output.has_more {
		println!(
			"\n(More results available \
			- increase --limit to see more)",
		);
	}

	println!("\n{}", "-".repeat(60));
	println!("CONTEXT OUTPUT");
	println!("{}", "-".repeat(60));
	println!("{}", output.xml_output);
	println!(
		"\nRetrieval complete in {} ms",
		start.elapsed().as_millis(),
	);
}

/// Run structured retrieval and format output
fn format_structured(
	service: &DefaultRetrievalService,
	req: &RetrievalRequest,
	xml: bool,
	start: Instant,
) -> CommandResult {
	let output = service.retrieve_structured(req)?;

	if xml {
		println!("{}", output.to_xml());
		return Ok(());
	}

	match output.to_json() {
		Ok(json) => println!("{}", json),
		Err(e) => {
			return Err(CommandError::IndexError(
				format!(
					"JSON serialization error: {}",
					e,
				),
			))
		}
	}
	eprintln!(
		"\nRetrieval: {} code, {} doc, {} notes \
		in {}ms",
		output.code_context.len(),
		output.doc_context.len(),
		output.notes_context.len(),
		start.elapsed().as_millis(),
	);

	Ok(())
}

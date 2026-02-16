//! Retrieve command implementation
//! (agentic pipeline).
//!
//! Thin handler that delegates to
//! RetrievalService.

use std::io::Write;
use std::time::Instant;

use crate::retrieval::RetrievalOutput;
use crate::service::retrieval::types
	::RetrievalRequest;
use crate::service::{
	DefaultRetrievalService, RetrievalService,
};

use super::error::{CommandError, CommandResult};

#[allow(clippy::struct_excessive_bools)]
/// Boolean flags for the retrieve command
#[derive(Debug, Clone, Default)]
pub struct RetrieveFlags {
	/// disable query expansion
	pub no_expand: bool,
	/// disable reranking
	pub no_rerank: bool,
	/// disable context expansion
	pub no_context: bool,
	/// output as XML
	pub xml: bool,
	/// use structured output
	pub structured: bool,
}

/// Options for the retrieve command
#[derive(Default)]
pub struct RetrieveOptions {
	/// max number of results
	pub limit: usize,
	/// max output tokens
	pub max_tokens: usize,
	/// boolean flags
	pub flags: RetrieveFlags,
	/// RRF score threshold
	pub threshold: f32,
	/// minimum results per type
	pub min_results: usize,
}

/// Execute the `retrieve` command
pub fn retrieve_command(
	query: &str,
	opts: &RetrieveOptions,
) -> CommandResult {
	let start = Instant::now(); // timing
	let service = DefaultRetrievalService::new();
	let req = build_request(query, opts);
	let flags = &opts.flags;

	if flags.structured {
		return format_structured(
			&service, &req, flags.xml, start,
		);
	}
	run_retrieval(&service, &req, flags.xml, start)
}

/// Build a RetrievalRequest from options
fn build_request(
	query: &str,
	opts: &RetrieveOptions,
) -> RetrievalRequest {
	let flags = &opts.flags;
	RetrievalRequest {
		query: query.to_string(),
		limit: opts.limit,
		max_tokens: opts.max_tokens,
		flags: crate::service::retrieval::types
			::RetrievalFlags {
			no_expand: flags.no_expand,
			no_rerank: flags.no_rerank,
			no_context: flags.no_context,
			xml: flags.xml,
			structured: flags.structured,
		},
		threshold: opts.threshold,
		min_results: opts.min_results,
		project_path: ".".to_string(),
	}
}

/// Run retrieval and format output
fn run_retrieval(
	service: &DefaultRetrievalService,
	req: &RetrievalRequest,
	xml: bool,
	start: Instant,
) -> CommandResult {
	if !xml {
		let mut out = std::io::stdout().lock();
		writeln!(
			out,
			"Initializing retrieval pipeline...",
		)?;
	}
	let output = service.retrieve(req)?;
	format_output(&output, xml, start)?;
	Ok(())
}

/// Format and print retrieval output
fn format_output(
	output: &RetrievalOutput,
	xml: bool,
	start: Instant,
) -> std::io::Result<()> {
	let mut out = std::io::stdout().lock();
	if xml {
		writeln!(out, "{}", output.xml_output)?;
		return Ok(());
	}
	print_output_header(&mut out, output)?;
	print_output_body(&mut out, output, start)?;
	Ok(())
}

/// Print retrieval header (query, intent, counts)
fn print_output_header(
	out: &mut impl Write,
	output: &RetrievalOutput,
) -> std::io::Result<()> {
	writeln!(out, "\n{}", "=".repeat(60))?;
	writeln!(out, "RETRIEVAL RESULTS")?;
	writeln!(out, "{}", "=".repeat(60))?;
	writeln!(out, "\nQuery: {}", output.query)?;
	print_spec_details(out, output)?;
	Ok(())
}

/// Print search spec details and counts
fn print_spec_details(
	out: &mut impl Write,
	output: &RetrievalOutput,
) -> std::io::Result<()> {
	writeln!(
		out, "Intent: {:?}",
		output.search_spec.intent,
	)?;
	let names = &output.search_spec.symbol_names;
	if !names.is_empty() {
		writeln!(
			out, "Symbols: {}", names.join(", "),
		)?;
	}
	writeln!(
		out, "\nResults: {} found",
		output.result_count,
	)?;
	writeln!(
		out, "Tokens: ~{}", output.token_count
	)?;
	Ok(())
}

/// Print context output section and timing
fn print_output_body(
	out: &mut impl Write,
	output: &RetrievalOutput,
	start: Instant,
) -> std::io::Result<()> {
	if output.has_more {
		writeln!(
			out,
			"\n(More results available \
			- increase --limit to see more)",
		)?;
	}
	writeln!(out, "\n{}", "-".repeat(60))?;
	writeln!(out, "CONTEXT OUTPUT")?;
	writeln!(out, "{}", "-".repeat(60))?;
	writeln!(out, "{}", output.xml_output)?;
	writeln!(
		out,
		"\nRetrieval complete in {} ms",
		start.elapsed().as_millis(),
	)?;
	Ok(())
}

/// Run structured retrieval and format output
fn format_structured(
	service: &DefaultRetrievalService,
	req: &RetrievalRequest,
	xml: bool,
	start: Instant,
) -> CommandResult {
	let output =
		service.retrieve_structured(req)?;
	let mut out = std::io::stdout().lock();

	if xml {
		writeln!(out, "{}", output.to_xml())?;
		return Ok(());
	}
	let json = output.to_json().map_err(|err| {
		CommandError::IndexError(format!(
			"JSON serialization error: {}", err
		))
	})?;
	writeln!(out, "{}", json)?;
	print_structured_summary(&output, start)?;
	Ok(())
}

/// Print summary of structured retrieval
fn print_structured_summary(
	output: &crate::retrieval::agent
		::StructuredOutput,
	start: Instant,
) -> std::io::Result<()> {
	writeln!(
		std::io::stderr(),
		"\nRetrieval: {} code, {} doc, \
		{} notes in {}ms",
		output.code_context.len(),
		output.doc_context.len(),
		output.notes_context.len(),
		start.elapsed().as_millis(),
	)?;
	Ok(())
}

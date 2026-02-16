//! Iterative PRF expansion with bounded iterations.
//!
//! Wraps single-pass PRF expansion in a loop that
//! re-expands until convergence or max iterations.

use std::collections::HashSet;
use std::io::Write;

use super::{expand_with_feedback, PrfExpansion, PrfFeedback};

/// Maximum PRF refinement iterations
pub const MAX_PRF_ITERATIONS: usize = 3;

/// Overlap ratio above which PRF has converged
const CONVERGENCE_THRESHOLD: f64 = 0.9;

/// Expand query iteratively with PRF feedback.
///
/// Runs up to MAX_PRF_ITERATIONS passes of PRF
/// expansion. Stops early if the added terms
/// converge (overlap exceeds threshold).
/// Returns None if no useful terms are found.
pub fn expand_iterative(
	query: &str,
	feedback: &PrfFeedback,
) -> Option<PrfExpansion> {
	let mut current = expand_with_feedback(
		query, feedback,
	)?;
	let mut prev_terms = terms_set(&current);

	for iteration in 1..MAX_PRF_ITERATIONS {
		let next = expand_with_feedback(
			&current.expanded_text, feedback,
		);
		let Some(expanded) = next else {
			log_prf_converged(query, iteration);
			return Some(current);
		};
		let new_terms = terms_set(&expanded);
		if is_converged(&prev_terms, &new_terms) {
			log_prf_converged(query, iteration);
			return Some(current);
		}
		prev_terms = new_terms;
		current = expanded;
	}

	log_prf_capped(query, MAX_PRF_ITERATIONS);
	Some(current)
}

/// Collect added terms into a HashSet for comparison
fn terms_set(
	expansion: &PrfExpansion,
) -> HashSet<String> {
	expansion
		.added_terms
		.iter()
		.cloned()
		.collect()
}

/// Check if term sets have converged
fn is_converged(
	prev: &HashSet<String>,
	next: &HashSet<String>,
) -> bool {
	if prev.is_empty() && next.is_empty() {
		return true;
	}
	let union_size =
		prev.union(next).count().max(1);
	let intersect_size =
		prev.intersection(next).count();
	let overlap =
		intersect_size as f64 / union_size as f64;

	overlap >= CONVERGENCE_THRESHOLD
}

/// Log PRF convergence to stderr
fn log_prf_converged(query: &str, iteration: usize) {
	let _ = writeln!(
		std::io::stderr().lock(),
		"[query] PRF converged after {} iteration(s) \
		for \"{}\"",
		iteration, query
	);
}

/// Log PRF hitting max iterations to stderr
fn log_prf_capped(query: &str, max: usize) {
	let _ = writeln!(
		std::io::stderr().lock(),
		"[query] PRF capped at {} iterations \
		for \"{}\"",
		max, query
	);
}

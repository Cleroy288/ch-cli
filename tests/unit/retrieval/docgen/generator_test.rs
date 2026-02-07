//! Tests for retrieval::docgen::generator

use ch_cli::retrieval::docgen::DocGenerator;

#[test]
fn test_generator_not_ready() {
	let generator = DocGenerator::new();
	assert!(!generator.is_ready());
}

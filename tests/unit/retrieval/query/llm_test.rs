use ch_cli::retrieval::query::Phi3Model;

#[test]
#[ignore] // requires model download (~4GB)
fn test_generate() {
	let mut model = Phi3Model::new().unwrap();
	let output = model
		.generate("Hello, my name is", 20)
		.unwrap();
	assert!(!output.is_empty());
}

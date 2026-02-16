use rustean::retrieval::hybrid::embedding::{
	BgeEmbedder, EMBEDDING_DIM,
};

#[test]
#[ignore] // requires model download
fn test_embed_single() {
	let embedder = BgeEmbedder::new().unwrap();
	let embedding = embedder.embed_text("Hello, world!").unwrap();
	assert_eq!(embedding.len(), EMBEDDING_DIM);
}

#[test]
#[ignore] // requires model download
fn test_embed_batch() {
	let embedder = BgeEmbedder::new().unwrap();
	let texts = vec![
		"Hello, world!".to_string(),
		"How are you?".to_string(),
	];
	let embeddings = embedder.embed_batch(&texts).unwrap();
	assert_eq!(embeddings.len(), 2);
	assert_eq!(embeddings[0].len(), EMBEDDING_DIM);
}

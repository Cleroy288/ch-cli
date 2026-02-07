# Semantic Embeddings

## Summary

The ch-cli uses BGE (BAAI General Embedding) models to generate semantic embeddings for code symbols. These embeddings enable semantic search - finding code by meaning rather than just keywords.

## How It Works

```
                    ┌──────────────────┐
                    │   Code Symbol    │
                    │  "fn parse_user" │
                    └────────┬─────────┘
                             │
                    ┌────────▼─────────┐
                    │    Tokenizer     │
                    │  (WordPiece)     │
                    └────────┬─────────┘
                             │
                    ┌────────▼─────────┐
                    │   BERT Model     │
                    │ (bge-small-en)   │
                    └────────┬─────────┘
                             │
                    ┌────────▼─────────┐
                    │  Mean Pooling    │
                    │  + L2 Normalize  │
                    └────────┬─────────┘
                             │
                    ┌────────▼─────────┐
                    │ 384-dim Vector   │
                    │ [0.12, -0.34...] │
                    └──────────────────┘
```

1. **Tokenization**: Code is tokenized using the WordPiece tokenizer
2. **BERT Encoding**: Tokens pass through the BGE BERT model
3. **Mean Pooling**: Token embeddings are averaged (weighted by attention mask)
4. **Normalization**: L2 normalization for cosine similarity

## CLI Commands

### Generate Embeddings
```bash
# Index project and generate embeddings
ch-cli embed

# Force re-embedding all symbols
ch-cli embed --force

# Embed a specific path
ch-cli embed --path /path/to/project
```

### Semantic Search (coming in Phase 2)
```bash
# Search using semantic similarity
ch-cli search --semantic "parse user input"
```

## Model Details

| Property | Value |
|----------|-------|
| Model | BAAI/bge-small-en-v1.5 |
| Parameters | 33M |
| Embedding Dimension | 384 |
| Max Sequence Length | 512 tokens |
| Memory Usage | ~130MB |

## Vector Store

Embeddings are stored in an HNSW (Hierarchical Navigable Small World) index for fast approximate nearest neighbor search.

### Storage Location
```
.ch-index/
├── vectors.json      # Serialized vector points
└── vectors/          # (future) Binary format
```

### Performance
- Insert: O(log n)
- Search: O(log n)
- Memory: ~1.5KB per symbol (384 floats)

## Architecture

### Source Files

| File | Purpose |
|------|---------|
| `hybrid/embedding.rs` | BgeEmbedder struct and model inference |
| `hybrid/vector_store.rs` | VectorStore with HNSW index |
| `models/mod.rs` | Model loading from HuggingFace Hub |
| `models/cache.rs` | Model weight caching |

### Key Types

```rust
/// Embedding for a code symbol
pub struct VectorPoint {
    pub id: u64,
    pub vector: Vec<f32>,      // 384-dim
    pub file_path: PathBuf,
    pub line: usize,
    pub symbol_name: String,
    pub symbol_kind: String,
}

/// Search result with distance
pub struct SearchResult {
    pub point: VectorPoint,
    pub distance: f32,         // cosine distance (0 = identical)
}
```

## Configuration

Default model configuration in `RetrievalConfig`:

```rust
RetrievalConfig {
    embedding_model: "BAAI/bge-small-en-v1.5",
    model_cache: "~/.ch-cli/models",
    // ...
}
```

## First Run

On first use, the embedding model is automatically downloaded from HuggingFace Hub:

```
$ ch-cli embed
Indexing project at: .
Found 150 symbols
Connecting to daemon...
Daemon is ready
Generating embeddings...
  Batch 1: embedded 32 symbols (32/150)
  Batch 2: embedded 32 symbols (64/150)
  ...
Embedding complete: 150 symbols in 2340 ms
```

## GPU Acceleration

The embedder supports GPU acceleration through Candle:

```toml
# Cargo.toml features
[features]
default = ["cpu"]
metal = ["candle-core/metal"]   # macOS GPU
cuda = ["candle-core/cuda"]     # NVIDIA GPU
```

Build with Metal support:
```bash
cargo build --release --features metal
```

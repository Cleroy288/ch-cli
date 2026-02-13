# Model Loading

## Summary

The rustean retrieval system uses three ML models loaded via a background daemon. The model loading system supports both single-file and sharded model weights from HuggingFace Hub.

## Supported Models

| Model | Purpose | Architecture | Size |
|-------|---------|--------------|------|
| BAAI/bge-small-en-v1.5 | Embeddings | BERT | ~130MB |
| BAAI/bge-reranker-base | Reranking | XLM-RoBERTa | ~1.1GB |
| microsoft/Phi-3-mini-4k-instruct | Query Expansion | Phi-3 (sharded) | ~7.6GB |

**Total memory: ~5.3GB** (within 6GB budget)

## Architecture

### Model Loading Flow

```
download_model(model_id)
        │
        ▼
┌───────────────────────────────────┐
│  Try 1: model.safetensors         │
│  (single file, most common)       │
└───────────────┬───────────────────┘
                │ 404?
                ▼
┌───────────────────────────────────┐
│  Try 2: model.safetensors.index   │
│  (sharded model - parse index,    │
│   download all shards)            │
└───────────────┬───────────────────┘
                │ 404?
                ▼
┌───────────────────────────────────┐
│  Try 3: pytorch_model.bin         │
│  (legacy format fallback)         │
└───────────────────────────────────┘
```

### Sharded Model Support

Large models like Phi-3 are split into multiple safetensor files:

```
model.safetensors.index.json    # Index with weight_map
model-00001-of-00002.safetensors  # Shard 1 (~5GB)
model-00002-of-00002.safetensors  # Shard 2 (~2.7GB)
```

The `download_model()` function:
1. Downloads `model.safetensors.index.json`
2. Parses the `weight_map` to get unique shard filenames
3. Downloads each shard
4. Returns all paths for `VarBuilder::from_mmaped_safetensors()`

### XLM-RoBERTa Support

The BGE reranker uses XLM-RoBERTa architecture (not BERT). Key differences:

| Feature | BERT | XLM-RoBERTa |
|---------|------|-------------|
| Weight prefix | `bert.` | `roberta.` |
| Vocab size | 30,522 | 250,002 |
| Pad token | `[PAD]` (id=0) | `<pad>` (id=1) |
| Model class | `BertModel` | `XLMRobertaForSequenceClassification` |

## CLI Commands

```bash
# Start daemon (loads all models)
rustean daemon start

# Check loaded models
rustean daemon status

# Stop daemon
rustean daemon stop
```

## Configuration

Default model IDs in `RetrievalConfig`:

```rust
RetrievalConfig {
    embedding_model: "BAAI/bge-small-en-v1.5",
    reranker_model: "BAAI/bge-reranker-base",
    expansion_model: "microsoft/Phi-3-mini-4k-instruct",
    // ...
}
```

## Source Files

| File | Purpose |
|------|---------|
| `models/mod.rs` | `download_model()`, sharded loading |
| `models/cache.rs` | Model caching metadata |
| `hybrid/embedding.rs` | BGE embedder (BERT) |
| `rerank/cross_encoder.rs` | BGE reranker (XLM-RoBERTa) |
| `query/llm.rs` | Phi-3 LLM |
| `daemon/server.rs` | Model loading orchestration |

## First Run

On first daemon start, models are downloaded from HuggingFace Hub:

```
$ rustean daemon start
[daemon] Loading embedding model: BAAI/bge-small-en-v1.5
[models] Found single model.safetensors
[daemon] Embedding model loaded (dim=384)
[daemon] Loading reranker model: BAAI/bge-reranker-base
[models] Found single model.safetensors
[daemon] Reranker model loaded
[daemon] Loading LLM: microsoft/Phi-3-mini-4k-instruct
[models] Found sharded model, downloading shards...
[models] Downloading shard: model-00001-of-00002.safetensors
[models] Downloading shard: model-00002-of-00002.safetensors
[models] Downloaded 2 shards
[daemon] LLM loaded for query expansion
```

## Error Handling

The daemon runs in "degraded mode" if models fail to load:

| Model Failed | Fallback Behavior |
|--------------|-------------------|
| Embeddings | Keyword-only search |
| Reranker | Use RRF scores only |
| LLM | Heuristic query parsing |

## Performance

| Operation | Time (CPU) | Time (GPU) |
|-----------|------------|------------|
| Embedding (per batch) | ~50ms | ~10ms |
| Reranking (10 docs) | ~200ms | ~40ms |
| Query Expansion | ~5-8s | ~1-2s |
| **Full Pipeline** | **~9s** | **~2s** |

GPU acceleration (Metal on macOS, CUDA on Linux) significantly reduces inference time.

See `notes/benchmarks/rustean-vs-augment-mcp.md` for detailed benchmark results.

## Dependencies

```toml
# Cargo.toml
candle-core = "0.9"
candle-nn = "0.9"
candle-transformers = "0.9"  # Includes XLM-RoBERTa
hf-hub = "0.4"
tokenizers = "0.20"
```

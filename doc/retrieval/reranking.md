# Semantic Reranking

## Summary

Semantic reranking uses a cross-encoder model (BGE reranker) to rescore search results for better relevance. Unlike bi-encoders that compare embeddings, cross-encoders process query-document pairs together for more accurate scoring.

## How It Works

```
                    Hybrid Search Results
                           │
                           │ Top N candidates
                           ▼
              ┌────────────────────────────┐
              │     Cross-Encoder          │
              │  (BGE Reranker)            │
              │                            │
              │  Input: (query, document)  │
              │  Output: relevance score   │
              └────────────┬───────────────┘
                           │
                           │ Score each pair
                           ▼
              ┌────────────────────────────┐
              │       Sort by Score        │
              │                            │
              │  Higher score = more       │
              │  relevant to query         │
              └────────────┬───────────────┘
                           │
                           ▼
                    Reranked Results
```

## CLI Usage

```bash
# Hybrid search without reranking
ch-cli search --semantic "authentication"

# Hybrid search with reranking
ch-cli search --semantic --rerank "authentication"

# Full pipeline: semantic + rerank + context
ch-cli search --semantic --rerank --context "authentication"
```

## Output Format

```
Reranked search results for 'authentication':

  1. fn authenticate (auth.rs:45) [RRF:0.0322 K:1 S:3 R:0.892]
     pub fn authenticate(user: &User, password: &str) -> Result<Token>

  2. struct AuthService (service.rs:12) [RRF:0.0318 K:5 S:1 R:0.847]
     pub struct AuthService { ... }

  3. fn validate_token (token.rs:78) [RRF:0.0161 K:2 S:- R:0.756]
```

Legend:
- `RRF` = Hybrid search fusion score
- `K:n` = Keyword rank
- `S:n` = Semantic rank
- `R:x.xxx` = Rerank score (0-1, higher is better)

## Cross-Encoder vs Bi-Encoder

| Aspect | Bi-Encoder (Embeddings) | Cross-Encoder (Reranker) |
|--------|------------------------|-------------------------|
| **Input** | Query and doc separately | Query + doc together |
| **Speed** | Fast (precompute embeddings) | Slow (score each pair) |
| **Accuracy** | Good | Better |
| **Use case** | Initial retrieval | Final reranking |

## Architecture

### Model

The reranker uses `BAAI/bge-reranker-base`:
- 278M parameters
- ~1.1GB memory
- Input: tokenized (query, document) pair
- Output: relevance score (0-1 after sigmoid)

### Source Files

| File | Purpose |
|------|---------|
| `rerank/mod.rs` | Module exports and RerankedItem type |
| `rerank/cross_encoder.rs` | BgeReranker implementation |
| `daemon/server.rs` | Rerank request handling |

### Key Types

```rust
/// BGE Reranker for scoring query-document pairs
pub struct BgeReranker {
    model: BertModel,      // BERT encoder
    classifier: Linear,    // Classification head (hidden -> 1)
    tokenizer: Tokenizer,  // Pair tokenizer
    device: Device,        // CPU/GPU
}

/// Result of reranking
pub struct RerankedItem<T> {
    pub item: T,
    pub score: f32,        // Cross-encoder score
    pub original_rank: usize,
}
```

### Daemon Integration

The reranker is loaded in the daemon alongside the embedder:

```rust
// In daemon server
DaemonRequest::Rerank { query, documents } => {
    match &self.reranker {
        Some(reranker) => {
            let scores = reranker.score_batch(&query, &documents)?;
            DaemonResponse::Scores(scores)
        }
        None => {
            // Fallback: position-based scores
            DaemonResponse::Scores(fallback_scores)
        }
    }
}
```

## Pipeline Integration

When `--rerank` is enabled:

1. Hybrid search fetches 3x more candidates than `--limit`
2. Documents are formatted: `{kind} {name} {signature}`
3. Daemon scores each (query, document) pair
4. Results are sorted by rerank score
5. Top `--limit` results are returned

```rust
// Fetch more candidates for reranking
let fetch_limit = if rerank { limit * 3 } else { limit };
let results = hybrid.search(query, fetch_limit)?;

// Rerank and truncate
if rerank {
    results = rerank_results(query, results, limit)?;
}
```

## Performance

| Operation | Time |
|-----------|------|
| Model loading | ~3-5s (one-time) |
| Score per pair | ~10-15ms |
| Batch of 30 | ~50-80ms |

## When to Use Reranking

| Scenario | Recommendation |
|----------|----------------|
| Quick search | Skip reranking (`--semantic` only) |
| Important query | Use reranking (`--semantic --rerank`) |
| Natural language | Use reranking (better at understanding intent) |
| Exact symbol name | Skip reranking (keyword is enough) |

## Limitations

1. **Latency** - Each pair requires a forward pass
2. **Memory** - Reranker adds ~1.1GB to daemon
3. **Document length** - Truncated to 512 tokens

## Configuration

The reranker model is configured in `RetrievalConfig`:

```rust
RetrievalConfig {
    reranker_model: "BAAI/bge-reranker-base".to_string(),
    // ...
}
```

## References

- [BGE Reranker](https://huggingface.co/BAAI/bge-reranker-base)
- [Cross-Encoders for Reranking](https://www.sbert.net/examples/applications/cross-encoder/README.html)

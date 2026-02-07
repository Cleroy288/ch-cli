# Hybrid Search

## Summary

Hybrid search combines keyword-based search (Tantivy BM25) with semantic search (BGE embeddings) using Reciprocal Rank Fusion (RRF) to deliver more relevant results.

## How It Works

```
                        User Query
                            │
              ┌─────────────┴─────────────┐
              │                           │
              ▼                           ▼
    ┌─────────────────┐         ┌─────────────────┐
    │  Keyword Search │         │ Semantic Search │
    │    (Tantivy)    │         │  (Embeddings)   │
    │     BM25        │         │    Cosine       │
    └────────┬────────┘         └────────┬────────┘
             │                           │
             │   Ranked Results          │   Ranked Results
             │                           │
             └─────────────┬─────────────┘
                           │
                  ┌────────▼────────┐
                  │   RRF Fusion    │
                  │  score = Σ 1/   │
                  │      (k+rank)   │
                  └────────┬────────┘
                           │
                  ┌────────▼────────┐
                  │  Final Results  │
                  │ (Deduplicated)  │
                  └─────────────────┘
```

## CLI Usage

```bash
# Standard keyword search
ch-cli search "parse_user"

# Hybrid search (keyword + semantic)
ch-cli search --semantic "parse user input"

# Fuzzy keyword search
ch-cli search --fuzzy "pars_usr"
```

## RRF Algorithm

Reciprocal Rank Fusion combines results from multiple rankers:

```
RRF_score(d) = Σ 1/(k + rank_r(d))
```

Where:
- `k = 60` (standard constant from the RRF paper)
- `rank_r(d)` = position of document d in ranker r (1-indexed)

### Example

| Document | Keyword Rank | Semantic Rank | RRF Score |
|----------|--------------|---------------|-----------|
| A | 1 | 3 | 1/61 + 1/63 = 0.0322 |
| B | 5 | 1 | 1/65 + 1/61 = 0.0318 |
| C | 2 | - | 1/62 = 0.0161 |
| D | - | 2 | 1/62 = 0.0161 |

Result: A > B > C = D

### Why RRF?

- **Simple**: No need to normalize scores between rankers
- **Effective**: Consistently performs well across domains
- **Robust**: Handles missing results gracefully (not in top-N)

## Configuration

```rust
HybridSearchConfig {
    // Weight for keyword search (default 1.0)
    keyword_weight: 1.0,

    // Weight for semantic search (default 1.0)
    semantic_weight: 1.0,

    // Candidates from each source before fusion
    candidates_per_source: 50,

    // RRF k constant (default 60)
    rrf_k: 60.0,
}
```

### Adjusting Weights

```rust
// Prioritize exact matches
config.keyword_weight = 2.0;
config.semantic_weight = 1.0;

// Prioritize semantic similarity
config.keyword_weight = 1.0;
config.semantic_weight = 2.0;
```

## Output Format

```
Hybrid search results for 'authentication':

  1. fn authenticate (auth.rs:45) [RRF:0.0322 K:1 S:3]
     pub fn authenticate(user: &User, password: &str) -> Result<Token>

  2. struct AuthService (service.rs:12) [RRF:0.0318 K:5 S:1]
     pub struct AuthService { ... }

  3. fn validate_token (token.rs:78) [RRF:0.0161 K:2 S:-]
```

Legend:
- `RRF` = Combined fusion score
- `K:n` = Keyword rank (position in BM25 results)
- `S:n` = Semantic rank (position in embedding results)
- `-` = Not in top-N for that ranker

## Architecture

### Source Files

| File | Purpose |
|------|---------|
| `hybrid/mod.rs` | HybridSearch orchestrator |
| `hybrid/fusion.rs` | RRF algorithm |
| `hybrid/embedding.rs` | BGE embeddings |
| `hybrid/vector_store.rs` | HNSW index |

### Key Types

```rust
/// Hybrid search result
pub struct HybridSearchResult {
    pub symbol: Symbol,
    pub rrf_score: f32,
    pub keyword_rank: Option<usize>,
    pub semantic_rank: Option<usize>,
    pub keyword_score: Option<f32>,
    pub semantic_distance: Option<f32>,
}

/// Configuration
pub struct HybridSearchConfig {
    pub keyword_weight: f32,
    pub semantic_weight: f32,
    pub candidates_per_source: usize,
    pub rrf_k: f32,
}
```

## When to Use Hybrid Search

| Scenario | Recommended |
|----------|-------------|
| Exact symbol name known | Keyword (`ch-cli search`) |
| Fuzzy/typo search | Keyword + fuzzy (`--fuzzy`) |
| Natural language query | Hybrid (`--semantic`) |
| Concept search | Hybrid (`--semantic`) |

### Examples

```bash
# You know the exact name
ch-cli search "IndexManager"

# You have typos
ch-cli search --fuzzy "indx_mngr"

# You're describing what you want
ch-cli search --semantic "function that parses user credentials"

# Conceptual search
ch-cli search --semantic "error handling for file operations"
```

## Performance

| Operation | Time |
|-----------|------|
| Keyword search | ~1ms |
| Embedding generation | ~15ms per query |
| HNSW search | ~1ms |
| RRF fusion | <1ms |
| **Total (hybrid)** | **~20ms** |

## Limitations

1. **First query is slow**: Daemon must load models (~5-10s)
2. **Embedding quality**: Depends on model understanding of code
3. **Index size**: Embeddings add ~1.5KB per symbol

## References

- [Reciprocal Rank Fusion paper](https://dl.acm.org/doi/10.1145/1571941.1572114)
- [BGE Embedding Models](https://huggingface.co/BAAI/bge-small-en-v1.5)

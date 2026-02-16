# Semantic Search Quality Report — 2026-02-10

## Summary

Three fixes were applied to improve semantic search precision:
enriched embedding text, hub function penalty, and CSLS
post-processing. The **primary goal** (eliminate `embed_text`/
`embed_batch` from dominating all results) is **fully achieved**.
However, **new hub functions emerged** — the semantic channel
still concentrates on a small set of generic results.

**Overall verdict: Partial improvement. Keyword channel now
dominates useful results. Semantic channel needs deeper work.**

---

## Test Setup

- **Model**: jinaai/jina-embeddings-v2-base-code (Metal GPU)
- **Index**: Full ch-cli codebase (~660 symbols)
- **Queries**: 10 diverse natural-language queries
- **Mode**: `--semantic` (hybrid: keyword + semantic + RRF)
- **Cache**: Cleared and re-embedded with version 2 format

---

## Results: Hub Function Domination

### Before (pre-fix behavior)

`embed_text` and `embed_batch` appeared in top-5 for
**every query** regardless of topic. ~15% precision.

### After (post-fix)

| Check | Result |
|-------|--------|
| `embed_text` in top-5 across 10 queries | **0/10** |
| `embed_batch` in top-5 across 10 queries | **0/10** |

**Primary goal achieved.**

### New Hub Functions

| Symbol | Appears in top-5 | Source |
|--------|------------------|--------|
| `index_symbols` (triple_indexing.rs) | 7/10 queries | Semantic only |
| `insert_batch` (vector_store_ops.rs) | 6/10 | Semantic only |
| `distance` (vector_store.rs) | 6/10 | Semantic only |
| `hybrid_search` mod (mod.rs) | 4/10 | Semantic only |

These are all **semantic-channel results** (S:rank only, no
K:rank). The keyword channel does NOT produce these — it
returns diverse, relevant results.

---

## Per-Query Precision Analysis

Scoring: Relevant if the result directly relates to the query
topic. Top-5, P@5 = relevant/5.

| # | Query | Top-1 Relevant? | P@5 | Notes |
|---|-------|-----------------|-----|-------|
| 1 | "error handling" | No (index_symbols) | 1/5 | Keyword found `error_docs` at #10 |
| 2 | "file watching and auto-reindex" | Yes (file_name) | 4/5 | Keyword dominates, good |
| 3 | "daemon load ML models" | Partial (models) | 2/5 | Semantic noise from index_symbols |
| 4 | "search ranking and scoring" | Yes (search) | 4/5 | Multiple search fns found |
| 5 | "tree-sitter parsing" | No (index_symbols) | 0/5 | Semantic hubs dominate |
| 6 | "generate docs for symbols" | Yes (index_symbols) | 5/5 | Best query — keyword found generator_batch |
| 7 | "persistent caching" | Partial (from_path) | 2/5 | Semantic noise |
| 8 | "RRF reciprocal rank fusion" | Yes (build_rrf_scores) | 2/5 | Top-1 excellent |
| 9 | "CLI argument parsing" | Partial (daemon_restart) | 2/5 | Semantic hubs in slots 3-5 |
| 10 | "cosine similarity" | Yes (distance) | 4/5 | Excellent — CSLS doc also found |

### Aggregate Precision

| Metric | Value |
|--------|-------|
| **Mean P@5** | **~2.6/5 = 52%** |
| **Top-1 relevance** | **6/10 = 60%** |
| **Queries with ≥3 relevant in top-5** | **4/10** |

---

## Channel Analysis

The hybrid search combines keyword (K) and semantic (S)
results via RRF fusion. Analysis of where good results come
from:

| Channel | Contribution to relevant top-5 results |
|---------|---------------------------------------|
| **Keyword only** (K:rank, no S:rank) | ~70% of relevant results |
| **Semantic only** (S:rank, no K:rank) | ~5% of relevant results |
| **Both** (K + S fused) | ~25% of relevant results |

**The keyword channel carries the search quality.** The
semantic channel mostly contributes noise (hub functions
ranking high due to embedding space hubness).

---

## Why New Hubs Still Appear

1. **ref_count=0 everywhere**: Hub penalty passes `ref_count=0`
   to all scoring paths because the reference count is not
   available at scoring time. The `ref_count_penalty` has zero
   effect.

2. **Embedding text similarity**: Despite enrichment,
   `"method index symbols in retrieval hybrid triple indexing"`
   is still semantically close to many NL queries because
   "index" and "symbols" are high-frequency terms in the
   codebase domain.

3. **CSLS K=10 too high**: With ~660 symbols, K=10 for mean
   neighbor distance might not discriminate well enough.
   Hub points in a small index have similar neighborhood
   densities.

4. **path_penalty too narrow**: Only matches exact substrings
   like `"hybrid/embedding"`. `triple_indexing.rs` doesn't
   match, so it escapes penalty.

---

## What Worked

| Fix | Impact |
|-----|--------|
| **Enriched embedding text** | Eliminated old hubs (embed_text/embed_batch). New text is more discriminative. |
| **Hub name penalty** | Blocks `new`, `default`, `clone` etc. Works well for those names. |
| **CSLS reranking** | Marginal effect on small index. Would help more at scale (>5000 symbols). |
| **Cache invalidation** | Works correctly. Version file written. Stale vectors cleaned. |

## What Didn't Work (Enough)

| Issue | Root Cause |
|-------|-----------|
| New hubs (index_symbols, distance) | Generic domain terms in embedding text; ref_count not wired |
| Semantic channel low precision | Embedding model (jina-v2-base-code) struggles with NL→code mapping for Rust |
| CSLS low impact | Small corpus (~660 points), hub detection needs larger neighborhood |

---

## Recommendations (Next Steps)

1. **Wire ref_count into hub_penalty** — look up actual
   reference counts from SemanticGraph at scoring time.
   `index_symbols` with 10+ callers would get 0.7x penalty.

2. **Expand path_penalty patterns** — add `"triple_indexing"`,
   `"vector_store_ops"`, `"triple_vector"` to infra paths.

3. **Lower semantic_weight in RRF** — since keyword channel
   is more accurate, reduce semantic contribution (e.g. 0.3
   semantic vs 0.7 keyword). Currently appears to be equal.

4. **IDF-weighted embedding text** — prepend terms that are
   rare in the corpus (high IDF) to boost discriminativeness.

5. **Consider larger embedding model** — jina-v2-base may
   lack capacity for 768-dim Rust code understanding.
   jina-v3 or code-specific models could help.

---

## Raw Data: 10 Queries × Top-10 Results

### Q1: "error handling"
```
1. method index_symbols (triple_indexing.rs:17) [0.0454 - S:46]
2. fn distance (vector_store.rs:32) [0.0362 - S:15]
3. method insert_batch (vector_store_ops.rs:24) [0.0317 - S:36]
4. fn hybrid_init_docs (hybrid_init.rs:1) [0.0276 - S:16]
5. mod hybrid_search (mod.rs:29) [0.0305 - S:4]
```

### Q4: "search ranking and scoring"
```
1. method search (querying.rs:19) [0.0994 K:26 -]
2. method search (vector_store_persist.rs:22) [0.0482 K:46 -]
3. fn search (default.rs:38) [0.0475 K:29 -]
4. fn search (hybrid_search.rs:15) [0.0459 K:32 -]
5. method index_symbols (triple_indexing.rs:17) [0.0447 - S:48]
```

### Q6: "generate documentation for symbols"
```
1. method index_symbols (triple_indexing.rs:17) [0.1250 K:48 S:44]
2. method index_symbols_with_docs (triple_indexing.rs:25) [0.0889 K:36 -]
3. method generate_for_ids (generator_batch.rs:38) [0.0613 K:9 -]
4. method generate_batch (generator_batch.rs:16) [0.0535 K:19 -]
5. method index_symbols (index.rs:62) [0.0515 K:22 -]
```

### Q8: "RRF reciprocal rank fusion"
```
1. method build_rrf_scores (triple_fusion_impl.rs:47) [0.1069 K:38 S:46]
2. fn empty_result (triple_fusion_impl.rs:122) [0.0384 K:50 -]
3. fn distance (vector_store.rs:32) [0.0355 - S:17]
4. fn rrf_score (fusion.rs:40) [0.0309 K:8 -]
5. method insert_batch (vector_store_ops.rs:24) [0.0305 - S:45]
```

### Q10: "cosine similarity and vector distance"
```
1. method distance (vector_store.rs:32) [0.1171 K:4 S:14]
2. method vector_dim (vector_store_persist.rs:85) [0.0517 K:39 -]
3. method vector_store (hybrid_search.rs:71) [0.0503 K:24 -]
4. method index_symbols (triple_indexing.rs:17) [0.0458 - S:44]
5. method vector_dim (triple_vector_query.rs:45) [0.0444 K:35 -]
```

# Relevance-Based Filtering Implementation

## Summary

Added RRF score threshold filtering to the structured pipeline. Results below the threshold are filtered out, with minimum guarantees to ensure some results always return.

## Problem

The structured pipeline returned **fixed limits** (10 code + 5 doc + 3 notes) regardless of actual relevance, resulting in noise when queries didn't have enough relevant results.

## Solution

Implemented **Option B: Min/Max with Threshold**:
- Filter results where `rrf_score < threshold` (default: 0.015)
- Keep minimum results per type (default: 1)
- Respect maximum limits (10 code, 5 doc, 3 notes)

## Key Changes

| File | Change |
|------|--------|
| `src/retrieval/hybrid/mod.rs` | Added `rrf_score_threshold`, `min_results_per_type` to `HybridSearchConfig` |
| `src/retrieval/hybrid/triple.rs` | Added `filter_by_relevance()` method, `with_config()` builder |
| `src/retrieval/agent/pipeline.rs` | Wired threshold config to `TripleHybridSearch` |
| `src/cli/mod.rs` | Added `--threshold` and `--min-results` CLI flags |
| `src/cli/commands.rs` | Wired CLI flags to pipeline config |

## RRF Score Distribution

With k=60 (default):

| Scenario | RRF Score |
|----------|-----------|
| Rank 1 only (one list) | 0.0164 |
| Rank 1 + Rank 1 (both lists) | 0.0328 |
| Rank 10 only | 0.0143 |
| Rank 50 only | 0.0091 |

**Default threshold: 0.015** filters rank >40 single-list hits, keeps dual-list hits.

## Usage

```bash
# Default filtering (threshold=0.015, min=1)
rustean retrieve "query" --structured

# Stricter filtering
rustean retrieve "query" --structured --threshold 0.02

# Allow empty sections
rustean retrieve "query" --structured --threshold 0.02 --min-results 0

# More lenient filtering
rustean retrieve "query" --structured --threshold 0.01
```

## Debug Output

When filtering occurs, the pipeline logs:
```
[pipeline] doc: 5 results, 6 filtered (score < 0.015)
```

## Tests

Added 5 unit tests for filtering logic:
- `test_filter_keeps_high_scores` - Above threshold kept
- `test_filter_respects_minimum` - Min guarantee honored
- `test_filter_respects_maximum` - Max limit enforced
- `test_filter_empty_input` - Empty input returns empty
- `test_filter_all_below_threshold_min_zero` - min=0 allows empty

## Date

2026-02-04

## Sources

- [Elasticsearch Weighted RRF](https://www.elastic.co/search-labs/blog/weighted-reciprocal-rank-fusion-rrf)
- [Azure Hybrid Search Scoring](https://learn.microsoft.com/en-us/azure/search/hybrid-search-ranking)
- [OpenSearch RRF](https://opensearch.org/blog/introducing-reciprocal-rank-fusion-hybrid-search/)

# Fast-Path Implementation Notes

## Date: 2026-02-01

## Objective
Implement hybrid fast-path architecture to reduce retrieval time from ~4.4s to <200ms for explicit symbol queries.

## Implementation Summary

### Files Created
1. `src/retrieval/query/fast_path.rs` (~180 lines)
   - FastPathParser with compiled regex patterns
   - SymbolCandidate struct for extracted symbols
   - FastPathResult with confidence and intent
   - Stop-word filtering
   - Conceptual query detection

2. `src/retrieval/query/validator.rs` (~150 lines)
   - SymbolValidator for SemanticGraph validation
   - Importance scoring with weighted factors
   - ValidatedSymbol and ValidationResult structs

3. `src/retrieval/query/tiered.rs` (~200 lines)
   - TieredQueryExpander orchestrating the 3-tier logic
   - TieredConfig for threshold configuration
   - TierUsed enum for tracking which tier was used
   - TieredResult with spec, tier, and timing

### Files Modified
1. `src/retrieval/query/mod.rs`
   - Added module declarations
   - Added public exports
   - Made fallback_parse public

2. `src/retrieval/agent/pipeline.rs`
   - Added tiered_expansion config fields
   - Integrated TieredQueryExpander in retrieve()
   - Added timing logs for tier selection

3. `src/cli/commands.rs`
   - Updated PipelineConfig initialization with new fields

4. `Cargo.toml`
   - Added `regex = "1.10"` dependency

## Design Decisions

### 1. Pattern Priority
- CamelCase (0.95): Highest confidence, most likely to be actual types
- SCREAMING_CASE (0.90): Constants are explicit
- snake_case (0.85): Functions/variables, slightly lower confidence

### 2. Stop Words
Comprehensive list to avoid false positives:
- Question words: how, where, what, when, why, which, who
- Articles: the, a, an
- Verbs: is, are, does, find, show, get, search
- Common terms: defined, used, called, work, implement

### 3. Conceptual Query Patterns
Patterns that require LLM understanding:
- "why does/is" - Requires reasoning
- "how does/do" - Requires explanation
- "explain" - Requires elaboration
- "what happens" - Requires process understanding

### 4. Threshold Defaults
- Confidence: 0.7 (70% confident symbols are real)
- Importance: 0.5 (symbols should have some usage)
- Existence: 0.5 (at least half symbols should exist)

## Test Results

All 80 tests pass:
```
test retrieval::query::fast_path::tests::test_extract_camel_case ... ok
test retrieval::query::fast_path::tests::test_extract_snake_case ... ok
test retrieval::query::fast_path::tests::test_extract_screaming_case ... ok
test retrieval::query::fast_path::tests::test_conceptual_query ... ok
test retrieval::query::fast_path::tests::test_mixed_query ... ok
test retrieval::query::fast_path::tests::test_multiple_symbols ... ok
test retrieval::query::fast_path::tests::test_stop_word_filter ... ok
test retrieval::query::validator::tests::test_validate_existing_symbol ... ok
test retrieval::query::validator::tests::test_validate_nonexistent_symbol ... ok
test retrieval::query::validator::tests::test_validate_multiple_symbols ... ok
test retrieval::query::validator::tests::test_importance_ranking ... ok
test retrieval::query::tiered::tests::test_fast_path_parser_extracts_symbols ... ok
test retrieval::query::tiered::tests::test_conceptual_query_detection ... ok
test retrieval::query::tiered::tests::test_config_default ... ok
test retrieval::query::tiered::tests::test_detect_intent ... ok
test retrieval::query::tiered::tests::test_validation_with_graph ... ok
```

## Future Improvements

1. **Fuzzy Matching**: Allow partial symbol matches
2. **Learning Thresholds**: Adapt thresholds based on query success
3. **Cache Validated Symbols**: Avoid re-validation for repeated queries
4. **Pattern Expansion**: Add more language-specific patterns (e.g., `__dunder__` for Python)

## References

- Plan document: `/Users/charlesleroy/.claude/projects/...`
- Research sources in original plan:
  - Augment Code quantized vector search
  - Hybrid BM25 + Embeddings
  - ZeroEntropy Architecture

# Retrieval Ranking Observations

## Summary

Manual testing of rustean retrieval revealed an issue with result ranking: planning documents are returned instead of actual source code implementations.

## Test Case

**Query**: "Where is DocStore defined?"

**Expected**: `src/retrieval/docgen/store.rs` (actual implementation)

**Returned**: `notes/plans/doc-generator-plan.md` (planning document)

## Retrieval Output

```
Query: Where is DocStore defined?
Intent: FindDefinition
Symbols: DocStore

Results: 3 found
Tokens: ~3939

<context-block>
  <symbol kind="doc" name="DocStore (NEW)"
    file="notes/plans/doc-generator-plan.md" line="162">
```

## Analysis

1. **Intent Detection**: Correct (FindDefinition)
2. **Symbol Extraction**: Correct (DocStore)
3. **Candidate Finding**: Found 9 candidates
4. **Ranking Issue**: Planning doc ranked higher than actual source

## Probable Causes

1. **No file type weighting**: `.rs` files not prioritized over `.md` files for code queries
2. **Content matching**: Planning doc has "DocStore" mentioned more times with more context
3. **Reranker bias**: May favor longer, more descriptive text

## Suggested Improvements

1. **File type boost**: Add score multiplier for source files (`.rs`, `.py`, etc.) when intent is FindDefinition
2. **Path filtering**: Option to exclude `notes/`, `docs/`, `*.md` from code searches
3. **Source vs doc distinction**: Detect if result is actual code vs documentation

## Performance Notes

- First run: ~139 seconds (model loading)
- Second run: Near instant (models cached)
- Daemon socket fix working: no connection errors

## Comparison with Augment MCP

| Aspect | rustean | Augment MCP |
|--------|--------|-------------|
| Availability | Requires daemon startup | Always available |
| Speed (cold) | ~139s first query | ~1-2s |
| Speed (warm) | <1s | ~1-2s |
| Accuracy | Found related, not exact | Better for definitions |
| Local/Private | Yes | Cloud-based |

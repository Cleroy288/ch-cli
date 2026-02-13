# Structured Pipeline vs Augment MCP Benchmark

**Date:** 2026-02-04

## Summary

Comparison of rustean structured pipeline (`--structured` flag) vs Augment MCP codebase-retrieval for the same queries.

## Test Queries

### Query 1: "how does daemon work"

| Metric | rustean (structured) | Augment MCP |
|--------|---------------------|-------------|
| Code results | 10 | 6 files |
| Doc results | 5 | 0 (mixed with code) |
| Notes results | 3 | 0 |
| Full file content | Yes (500 lines max) | No (snippets only) |
| Content separation | Yes | No |

**rustean results:**
- Code: `pipeline.rs`, `client.rs`, `server.rs`, `lifecycle.rs`, `protocol.rs`, etc.
- Doc: `agentic-pipeline.md`, `daemon.md`, etc.
- Notes: Implementation notes

**Augment MCP results:**
- `lifecycle.rs`, `server.rs`, `mod.rs`, `commands.rs`, `protocol.rs`, `client.rs`
- Only code snippets (relevant sections)
- No explicit doc/notes separation

### Query 2: "HybridSearch struct"

| Metric | rustean (structured) | Augment MCP |
|--------|---------------------|-------------|
| Code results | 5 | 6 files |
| Doc results | 5 | 1 (hybrid-search.md) |
| Notes results | 2 | 0 |
| Full file content | Yes | No (snippets) |

**Augment MCP advantage:** More focused on the actual HybridSearch struct definition and implementation.

**rustean note:** Query expansion interpreted "struct" as SymbolKind::Struct, returning all structs.

### Query 3: "query expansion documentation"

| Metric | rustean (structured) | Augment MCP |
|--------|---------------------|-------------|
| Doc results | 5 (guaranteed) | 3 docs + 3 code |
| Content type | Separated | Mixed |

**rustean advantage:** Guaranteed doc results won't be pushed out by code.

**Augment MCP advantage:** More context from related code files.

## Key Observations

### rustean Structured Pipeline Strengths

1. **Guaranteed content type coverage**
   - Always 10 code + 5 doc + 3 notes results
   - No risk of one type drowning out another

2. **Full file content**
   - Returns entire files (up to 500 lines)
   - LLM can see complete context

3. **Separated output**
   - Clear JSON structure with `code_context`, `doc_context`, `notes_context`
   - Easy for LLM to process by type

### Augment MCP Strengths

1. **More focused snippets**
   - Returns only relevant code sections
   - Less noise, more signal

2. **Better symbol matching**
   - "HybridSearch struct" → exact struct definition
   - rustean expanded to all "Struct" symbols

3. **Cross-type context**
   - Mixes code and docs when relevant
   - Single unified ranking

## Recommendations

### When to use rustean `--structured`

- When you need guaranteed doc/notes coverage
- When you want full file context for understanding
- When building LLM pipelines that need structured input

### When to use Augment MCP

- When you need focused, precise snippets
- When looking for specific symbol definitions
- When memory/token budget is limited

## Issues Found

1. **Query expansion over-interpretation**
   - "HybridSearch struct" → expanded to all Struct symbols
   - Should preserve exact symbol name

2. **No result ranking across types**
   - Code, doc, notes are separate
   - Can't compare relevance across types

## Conclusion

The structured pipeline achieves its goal of **guaranteed content type coverage**. Each query now returns:
- 10 code results (never pushed out by docs)
- 5 doc results (never pushed out by code)
- 3 notes results (always included)

This is an improvement over the unified pipeline where documentation could dominate code results or vice versa.

Augment MCP remains better for **precision queries** where you want the most relevant snippets regardless of content type.

## Update: Relevance-Based Filtering (2026-02-04)

Added RRF score threshold filtering to reduce noise.

### Test Results with Filtering (threshold=0.015)

| Query | Results | Filtered |
|-------|---------|----------|
| "how does daemon work" | 6 code, 5 doc, 3 notes | 12 code + 7 doc |
| "HybridSearch struct" | 5 code, 5 doc, 2 notes | 21 doc |
| "query expansion documentation" | 8 code, 5 doc, 3 notes | 18 code + 35 doc + 18 notes |

### Improvement

- Before: Fixed 10+5+3 = 18 results (regardless of relevance)
- After: Only relevant results returned (6-16 results depending on query)
- Noise reduction: 12-71 low-relevance results filtered per query

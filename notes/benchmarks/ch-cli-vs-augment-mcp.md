# Benchmark: rustean vs Augment MCP Code Retrieval

**Date:** 2026-01-31
**Codebase:** rustean (1622 symbols, ~15k LOC)
**Daemon Status:** Running with all 3 models loaded (~5.3GB)

---

## Summary

| Metric | rustean | Augment MCP | Winner |
|--------|--------|-------------|--------|
| **Speed** | ~9s per query | <1s per query | Augment MCP |
| **Code Accuracy** | Variable (LLM-dependent) | High | Augment MCP |
| **Context Quality** | Structured XML with code | Raw code + docs | rustean |
| **Documentation** | Includes docs if found | Strong doc retrieval | Augment MCP |
| **First Query** | Fast (daemon pre-loaded) | Fast | Tie |

---

## Speed Benchmark

| Query | rustean Time | Augment MCP Time |
|-------|-------------|------------------|
| "Where is BgeEmbedder defined?" | 9.091s | <1s |
| "How does RRF fusion combine scores?" | 9.230s | <1s |
| "How does daemon load ML models?" | 9.253s | <1s |
| "What is the architecture of the retrieval pipeline?" | 9.096s | <1s |
| **Average** | **9.17s** | **<1s** |

**Analysis:**
- rustean spends ~8s on ML inference (query expansion, embedding, reranking)
- Augment MCP uses proprietary retrieval without per-query ML inference
- rustean would be faster with GPU acceleration (Metal/CUDA)

---

## Code Retrieval Quality

### Query 1: "Where is BgeEmbedder defined?"

**rustean Results:**
- ✅ Found `hybrid/embedding.rs:25` - struct definition
- ✅ Found `hybrid/embedding.rs:36` - impl block
- ❌ Also returned unrelated results (BgeReranker, location fields)

**Augment MCP Results:**
- ✅ Found implementation notes (retrieval-embeddings.txt)
- ✅ Found documentation (doc/retrieval/embeddings.md)
- ✅ Found cross_encoder.rs (related reranker)
- ✅ Found daemon/server.rs (usage context)

**Winner:** Augment MCP (broader context, less noise)

---

### Query 2: "How does RRF fusion combine keyword and semantic search scores?"

**rustean Results:**
- ✅ Found `hybrid/mod.rs:50` - rrf_score field
- ❌ Missed the actual RRF algorithm in fusion.rs

**Augment MCP Results:**
- ✅ Found `hybrid/fusion.rs` - full RRF algorithm with formula
- ✅ Found `doc/retrieval/hybrid-search.md` - documentation with examples
- ✅ Found `agent/pipeline.rs` - usage in pipeline
- ✅ Found `indexer/search.rs` - keyword search integration

**Winner:** Augment MCP (found the actual algorithm)

---

### Query 3: "How does the daemon load and serve ML models?"

**rustean Results:**
- ❌ Found `query/llm.rs:26` - Phi3Model field (not the daemon)
- ❌ Found `mod.rs:41` - ModelLoading error variant (tangential)

**Augment MCP Results:**
- ✅ Found `daemon/server.rs` - full ModelDaemon with load_models()
- ✅ Found `daemon/mod.rs` - module overview
- ✅ Found model loading notes and documentation

**Winner:** Augment MCP (found the correct daemon code)

---

### Query 4: "What is the architecture of the retrieval pipeline?"

**rustean Results:**
- ❌ Found `watcher.rs:52` - unrelated file watcher code
- ❌ Did not find pipeline.rs or architecture docs

**Augment MCP Results:**
- ✅ Found `agent/pipeline.rs` - full RetrievalPipeline struct
- ✅ Found `doc/retrieval/agentic-pipeline.md` - architecture diagram
- ✅ Found `agent/mod.rs` - module overview with 5-step description
- ✅ Found `hybrid/fusion.rs` - fusion algorithm
- ✅ Found `context/mod.rs` - context expansion

**Winner:** Augment MCP (comprehensive architecture view)

---

## Context Quality Comparison

### rustean Context Format
```xml
<context>
<context-block>
  <symbol kind="struct" name="BgeEmbedder" file="embedding.rs" line="25">
    <code>
  24 | /// BGE Embedder for generating sentence embeddings
  25 | pub struct BgeEmbedder {
  26 |     model: BertModel,
  ...
    </code>
  </symbol>
</context-block>
</context>
```

**Pros:**
- Structured XML format (LLM-friendly)
- Line numbers included
- Symbol metadata (kind, name, file)
- Code extracted with context

**Cons:**
- Limited to 10 results by default
- May include irrelevant results after reranking

---

### Augment MCP Context Format
```
Path: src/retrieval/hybrid/fusion.rs
     1	//! Reciprocal Rank Fusion (RRF)
     2	//!
     3	//! Combines results from multiple search sources using the RRF algorithm.
...
```

**Pros:**
- Full file paths
- Includes documentation and notes
- Cross-references between files
- Broader context coverage

**Cons:**
- Raw text format (less structured)
- May include too much content

---

## Root Cause Analysis: Why rustean Underperforms

### 1. Query Expansion Issues
The Phi-3 LLM extracts poor symbol names from queries:
- "Where is BgeEmbedder defined?" → symbols: ["Where", "BgeEmbedder"]
- "How does RRF fusion..." → symbols: ["How", "RRF"]

The word "How" and "Where" are being extracted as symbol names, which pollutes search results.

### 2. Reranking Limitations
The cross-encoder scores (query, document) pairs, but documents are short:
```
"struct BgeEmbedder fn new"
```
This lacks semantic richness for accurate relevance scoring.

### 3. Index Granularity
rustean indexes symbols (functions, structs, methods), while Augment MCP appears to index file chunks and documentation, providing broader context.

---

## Recommendations for rustean Improvement

1. **Fix Query Expansion**
   - Filter common words ("How", "Where", "What") from symbol extraction
   - Use intent detection to weight different search strategies

2. **Improve Reranking Documents**
   - Include more context in reranking (docstrings, surrounding code)
   - Consider reranking full file snippets, not just symbol names

3. **Add Documentation Search**
   - Index markdown files in `doc/` and `notes/`
   - Combine code + doc results for comprehensive answers

4. **GPU Acceleration**
   - Enable Metal support for macOS
   - Would reduce query time from ~9s to ~1-2s

5. **Hybrid Weight Tuning**
   - Current: 50% keyword, 50% semantic
   - Consider boosting semantic for natural language queries

---

## Conclusion

**Augment MCP** significantly outperforms **rustean** in:
- Speed: 9x faster
- Accuracy: Finds correct files more consistently
- Coverage: Includes documentation alongside code

**rustean** has advantages in:
- Structured XML output (better for LLM consumption)
- Local execution (no external service dependency)
- Customizable pipeline (can tune for specific use cases)

**Recommendation:** For production use, Augment MCP is superior. However, rustean's pipeline provides a solid foundation that can be improved with the fixes outlined above.

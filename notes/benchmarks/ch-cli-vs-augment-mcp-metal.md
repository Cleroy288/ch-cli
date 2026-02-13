# Benchmark: rustean (Metal) vs Augment MCP Code Retrieval

**Date:** 2026-01-31
**Codebase:** rustean (1658 symbols)
**Device:** Apple Silicon GPU (Metal, 25769 MB)
**Previous Benchmark:** rustean CPU vs Augment MCP (2026-01-31)

---

## Summary

| Metric | rustean (CPU) | rustean (Metal) | Augment MCP | Winner |
|--------|--------------|----------------|-------------|--------|
| **Speed** | ~9.2s | ~4.4s | <1s | Augment MCP |
| **Speedup vs CPU** | - | **2.1x faster** | - | rustean Metal |
| **Code Accuracy** | Variable | Variable | High | Augment MCP |
| **Context Quality** | Structured XML | Structured XML | Raw code + docs | rustean |
| **Local Execution** | Yes | Yes | No (cloud) | rustean |

---

## Speed Benchmark

### rustean with Metal Acceleration

| Query | CPU Time | Metal Time | Speedup |
|-------|----------|------------|---------|
| "Where is BgeEmbedder defined?" | 9.091s | 4.288s | **2.1x** |
| "How does RRF fusion combine scores?" | 9.230s | 4.533s | **2.0x** |
| "How does daemon load ML models?" | 9.253s | 4.348s | **2.1x** |
| "What is the architecture of the retrieval pipeline?" | 9.096s | 4.255s | **2.1x** |
| **Average** | **9.17s** | **4.36s** | **2.1x** |

**Note:** First query after daemon restart was ~11s due to Metal GPU warmup. Subsequent queries stabilized at ~4.3s.

### Augment MCP

All 4 queries completed in <1s (instant response).

---

## Metal Acceleration Details

### Build Configuration
```bash
cargo build --release --features metal
```

### Daemon Status with Metal
```
Daemon Status: Running
Device:        Metal (Apple Silicon GPU)
GPU Memory:    25769 MB

Loaded Models:
  - BAAI/bge-small-en-v1.5 (embeddings)
  - BAAI/bge-reranker-base (reranker)
  - microsoft/phi-3-mini-4k-instruct (query expansion)
```

### Frameworks Linked
```
/System/Library/Frameworks/Metal.framework/Versions/A/Metal
```

---

## Performance Analysis

### Where Time is Spent (Metal)

| Operation | Estimated Time |
|-----------|---------------|
| Query Expansion (Phi-3 LLM) | ~3.0s |
| Embedding Generation (BGE) | ~0.3s |
| Reranking (Cross-encoder) | ~0.5s |
| Hybrid Search | ~0.2s |
| Context Expansion | ~0.2s |
| **Total** | **~4.2s** |

**Bottleneck:** Query expansion with Phi-3 LLM dominates (~70% of time).

### CPU vs Metal Comparison

The 2.1x speedup comes from Metal acceleration of:
1. **BGE Embedder** (BERT model) - embedding generation
2. **BGE Reranker** (XLM-RoBERTa) - cross-encoder scoring
3. **Phi-3 LLM** - query expansion

All three models now run on Apple Silicon GPU instead of CPU.

---

## Retrieval Quality (Unchanged)

Metal acceleration does not affect retrieval quality - the same models produce identical results, just faster.

### Query 1: "Where is BgeEmbedder defined?"
- Found: `hybrid/embedding.rs:36` (impl block)
- Still extracts "Where" as symbol name (query expansion issue)

### Query 2: "How does RRF fusion combine scores?"
- Found: `hybrid/fusion.rs:28` (rrf_score field)
- Missed the actual algorithm in fusion.rs

### Query 3: "How does daemon load ML models?"
- Found: `mod.rs:41` (ModelLoading error variant)
- Missed `daemon/server.rs:84` (load_models function)

### Query 4: "What is the architecture of the retrieval pipeline?"
- Found: `file_ref.rs:79` (unrelated)
- Missed `agent/pipeline.rs` (the actual pipeline)

---

## Comparison: rustean vs Augment MCP

### Augment MCP Advantages
1. **Speed**: ~10x faster than rustean Metal
2. **Accuracy**: Finds correct files consistently
3. **Documentation**: Includes docs and notes alongside code
4. **Coverage**: Broader context per query

### rustean Advantages
1. **Local Execution**: No network dependency
2. **Privacy**: Code never leaves machine
3. **Structured Output**: XML format optimized for LLM consumption
4. **Customizable**: Pipeline can be tuned for specific use cases
5. **Metal Support**: Runs on Apple Silicon GPU

---

## Recommendations

### Short-term Improvements
1. **Fix Query Expansion**: Filter stop words ("How", "Where", "What") from symbol extraction
2. **Optimize Phi-3**: Use quantized model (4-bit) to reduce LLM time
3. **Cache Queries**: Add query result caching for repeated queries

### Medium-term Improvements
1. **Smaller LLM**: Consider Phi-2 or distilled model for expansion
2. **Batch GPU Operations**: Better GPU utilization with batching
3. **Index Documentation**: Add markdown files to search index

### Long-term Goals
1. **Match Augment MCP Speed**: Target <1s per query
2. **Improve Accuracy**: Better query understanding and result ranking

---

## Conclusion

**Metal acceleration delivers a 2.1x speedup**, reducing rustean query time from ~9.2s to ~4.4s. This is a significant improvement, but rustean remains ~4x slower than Augment MCP.

The main bottleneck is now the Phi-3 LLM for query expansion (~70% of query time). Optimizing or replacing this component would yield the largest performance gains.

**For production use:**
- Use **Augment MCP** when speed and accuracy are critical
- Use **rustean** when local execution and privacy are required

**Current Status:** Metal acceleration is working and provides measurable benefits. Further optimization needed to match cloud-based solutions.

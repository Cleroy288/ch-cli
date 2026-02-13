# Testing the Code Retrieval System

## Summary

This document explains how to test and benchmark the rustean code retrieval system, including comparisons with Augment MCP.

---

## Prerequisites

- Rust toolchain installed
- rustean project cloned
- (Optional) Augment MCP server configured in Claude Code

---

## 1. Build with GPU Acceleration

### macOS (Metal)
```bash
cargo build --release --features metal
```

### Linux (CUDA)
```bash
cargo build --release --features cuda
```

### CPU Only
```bash
cargo build --release
```

### Verify Metal is Linked (macOS)
```bash
otool -L ./target/release/rustean | grep -i metal
# Should show: /System/Library/Frameworks/Metal.framework/...
```

---

## 2. Daemon Management

### Start Daemon
```bash
./target/release/rustean daemon start
```

Expected output with Metal:
```
[daemon] Using device: Metal (Apple Silicon GPU)
[daemon] GPU memory: 25769 MB
[daemon] Loading embedding model: BAAI/bge-small-en-v1.5
[daemon] Embedding model loaded (dim=384)
[daemon] Loading reranker model: BAAI/bge-reranker-base
Daemon started (PID: 12345)
```

### Check Status
```bash
./target/release/rustean daemon status
```

Expected output:
```
Daemon Status: Running
PID:           12345
Device:        Metal (Apple Silicon GPU)
GPU Memory:    25769 MB
Uptime:        60 seconds

Loaded Models:
  - BAAI/bge-small-en-v1.5 (embeddings)
  - BAAI/bge-reranker-base (reranker)
  - microsoft/phi-3-mini-4k-instruct (query expansion)
```

### Stop Daemon
```bash
./target/release/rustean daemon stop
```

### Restart Daemon
```bash
./target/release/rustean daemon stop && sleep 2 && ./target/release/rustean daemon start
```

### Force CPU Mode (for comparison)
```bash
CH_FORCE_CPU=1 ./target/release/rustean daemon start
```

---

## 3. Running Retrieval Queries

### Basic Query
```bash
./target/release/rustean retrieve "your query here"
```

### Timed Query
```bash
time ./target/release/rustean retrieve "Where is BgeEmbedder defined?"
```

### Query with More Results
```bash
./target/release/rustean retrieve "your query" --limit 20
```

### Example Output
```
Initializing retrieval pipeline...
[pipeline] Indexing project: .
[pipeline] Found 1658 symbols
[pipeline] Building hybrid search index...
[pipeline] Expanding query...
[pipeline] Search spec: 2 symbols, intent: Search
[pipeline] Found 30 candidates
[pipeline] Reranking...
[pipeline] Expanding context...

============================================================
RETRIEVAL RESULTS
============================================================

Query: Where is BgeEmbedder defined?
Intent: Search
Symbols: Where, BgeEmbedder

Results: 10 found
Tokens: ~3682
```

---

## 4. Benchmark Queries

Use these standard queries for consistent benchmarking:

```bash
# Query 1: Symbol location
time ./target/release/rustean retrieve "Where is BgeEmbedder defined?"

# Query 2: Algorithm understanding
time ./target/release/rustean retrieve "How does RRF fusion combine keyword and semantic search scores?"

# Query 3: System behavior
time ./target/release/rustean retrieve "How does the daemon load and serve ML models?"

# Query 4: Architecture
time ./target/release/rustean retrieve "What is the architecture of the retrieval pipeline?"
```

### Run All Benchmarks (Script)
```bash
#!/bin/bash
queries=(
    "Where is BgeEmbedder defined?"
    "How does RRF fusion combine keyword and semantic search scores?"
    "How does the daemon load and serve ML models?"
    "What is the architecture of the retrieval pipeline?"
)

for query in "${queries[@]}"; do
    echo "=== Query: $query ==="
    time (./target/release/rustean retrieve "$query" 2>&1 | head -20)
    echo ""
done
```

---

## 5. Comparing with Augment MCP

### Using Claude Code

In Claude Code, use the Augment MCP codebase-retrieval tool:

```
Use codebase-retrieval to find: "Where is BgeEmbedder defined?"
Directory: /path/to/rustean
```

### Expected Behavior

| Metric | rustean (Metal) | Augment MCP |
|--------|----------------|-------------|
| Speed | ~4-5s | <1s |
| Results | Structured XML | Raw code blocks |
| Docs | Code only | Code + docs + notes |

---

## 6. Performance Expectations

### Query Times by Device

| Device | Expected Time |
|--------|---------------|
| CPU (M1/M2) | ~9-10s |
| Metal (M1/M2) | ~4-5s |
| CUDA (RTX 3080) | ~2-3s (estimated) |

### First Query Warmup

The first query after daemon restart may be slower (~10-12s) due to GPU warmup. Subsequent queries stabilize.

---

## 7. Troubleshooting

### Daemon Not Responding
```bash
# Check if daemon is running
./target/release/rustean daemon status

# Force restart
./target/release/rustean daemon stop
sleep 2
./target/release/rustean daemon start
```

### Metal Not Detected
```bash
# Check if binary has Metal linked
otool -L ./target/release/rustean | grep metal

# If not, rebuild with metal feature
cargo build --release --features metal
```

### Out of GPU Memory
```bash
# Force CPU mode
CH_FORCE_CPU=1 ./target/release/rustean daemon start
```

### Socket Error
```bash
# Remove stale socket
rm -f ~/Library/Application\ Support/com.rustean.rustean/ml.sock
./target/release/rustean daemon start
```

---

## 8. Recording Benchmark Results

Save benchmark results in `notes/benchmarks/` with format:

```
notes/benchmarks/YYYY-MM-DD-description.md
```

Include:
- Date and device info
- Build configuration
- Query times (all 4 standard queries)
- Comparison with previous results
- Any issues encountered

---

## Quick Reference

```bash
# Full benchmark workflow
cargo build --release --features metal
./target/release/rustean daemon stop 2>/dev/null
./target/release/rustean daemon start
sleep 5
./target/release/rustean daemon status
time ./target/release/rustean retrieve "Where is BgeEmbedder defined?"
time ./target/release/rustean retrieve "How does RRF fusion combine scores?"
time ./target/release/rustean retrieve "How does daemon load ML models?"
time ./target/release/rustean retrieve "What is the retrieval pipeline architecture?"
```

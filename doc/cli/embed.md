# Embed Command

Generate semantic embeddings for indexed code symbols to enable semantic search.

## Syntax

```bash
ch-cli embed [OPTIONS]
```

## Options

| Option | Description |
|--------|-------------|
| `--path <PATH>` | Path to project root (default: `.`) |
| `--force` | Force re-embedding all symbols (removes existing index) |
| `-h, --help` | Print help information |

## Description

The `embed` command generates vector embeddings for all code symbols in your project. These embeddings enable **semantic search** - finding code by meaning rather than just keywords.

### What Are Embeddings?

Embeddings are numerical representations (384-dimensional vectors) of code symbols. Similar code concepts have similar embeddings, enabling:
- **Semantic search** - Find "parse user input" even if the function is named `extract_credentials`
- **Concept matching** - Search by intent, not exact names
- **Fuzzy relationships** - Discover related code across the codebase

### The Embedding Process

```
Code Symbol           Tokenizer          BERT Model         Final Vector
"fn parse_user"  -->  [tokens]   -->   [hidden states]  -->  [0.12, -0.34, ...]
                    (WordPiece)      (bge-small-en)        (384 dimensions)
```

1. **Tokenization** - Code is tokenized using WordPiece
2. **BERT Encoding** - Tokens pass through the BGE model
3. **Mean Pooling** - Token embeddings are averaged
4. **Normalization** - L2 normalization for cosine similarity

## Usage Examples

### Basic Embedding
```bash
# Generate embeddings for current project
ch-cli embed

# Embed a specific project directory
ch-cli embed --path /path/to/project
```

### Force Re-Embedding
```bash
# Force complete re-embedding (removes existing index first)
ch-cli embed --force

# Force re-embed a specific path
ch-cli embed --path /path/to/project --force
```

## What Gets Embedded

Each symbol is converted to a text representation combining:
- **Symbol kind** - function, struct, enum, etc.
- **Symbol name** - The identifier
- **Signature** - Function parameters and return type (if available)

Example text sent to the embedding model:
```
function parse_user fn parse_user(input: &str) -> Result<User, Error>
```

### Supported Symbol Types
- Functions and methods
- Structs and enums
- Traits and trait implementations
- Constants and static variables
- Type aliases and modules
- Macros and enum variants

## Prerequisites

Before running `embed`, ensure:

### 1. Index Must Exist
The project must be indexed first. The embed command automatically runs indexing, but you can also run it separately:
```bash
ch-cli index --semantic
```

### 2. Daemon Must Be Running
The embedding model runs in a background daemon process. The embed command auto-starts the daemon if needed, but you can start it manually:
```bash
ch-cli daemon start
```

To verify daemon status:
```bash
ch-cli daemon status
```

## When to Use --force

Use the `--force` flag to completely re-generate all embeddings:

### Model Updates
```bash
# After upgrading to a new embedding model version
ch-cli embed --force
```

### Index Corruption
```bash
# If semantic search returns unexpected results
ch-cli embed --force
```

### Configuration Changes
```bash
# After changing embedding-related settings
ch-cli embed --force
```

### Full Cleanup
```bash
# Force removes the .ch-index/ directory and rebuilds everything
ch-cli embed --force
```

## Performance

### Typical Embedding Times

| Project Size | Symbols | Time |
|--------------|---------|------|
| Small (< 100 files) | ~150 symbols | ~2-3 seconds |
| Medium (100-500 files) | ~500 symbols | ~5-10 seconds |
| Large (500+ files) | ~2000 symbols | ~30-60 seconds |

### Performance Factors
- **Batch size** - Symbols are batched by 32 for optimal throughput
- **Daemon warm-up** - First embedding request may be slower (model loading)
- **GPU acceleration** - Metal (macOS) or CUDA (NVIDIA) can speed up embedding

### Incremental vs Full Embedding
- **Normal mode** - Re-uses existing index, only embeds new/changed symbols
- **Force mode** - Removes all data and re-embeds everything from scratch

## Model Details

| Property | Value |
|----------|-------|
| Model | BAAI/bge-small-en-v1.5 |
| Parameters | 33M |
| Embedding Dimension | 384 |
| Max Sequence Length | 512 tokens |
| Memory Usage | ~130MB |

### First Run

On first use, the model is automatically downloaded from HuggingFace Hub:
```
$ ch-cli embed
Indexing project at: .
Found 150 symbols
Connecting to daemon...
Daemon is ready
Generating embeddings...
  Batch 1: embedded 32 symbols (32/150)
  Batch 2: embedded 32 symbols (64/150)
  ...
Embedding complete: 150 symbols in 2340 ms
```

## Typical Workflow

The recommended workflow for semantic code search:

### 1. Index the Project
```bash
ch-cli index --semantic
```

### 2. Start the Daemon
```bash
ch-cli daemon start
```

### 3. Generate Embeddings
```bash
ch-cli embed
```

### 4. Search Semantically
```bash
ch-cli search --semantic "parse user input"
```

### One-Shot Workflow
The embed command can do steps 1-3 automatically:
```bash
# This indexes, starts daemon (if needed), and embeds
ch-cli embed
```

## Storage

Embeddings are stored in the `.ch-index/` directory:
```
.ch-index/
├── index.state        # Index metadata
├── tantivy/           # Keyword search index
└── vectors.json       # Embedded vectors (HNSW index)
```

Memory usage per symbol: ~1.5KB (384 floats at 4 bytes each)

## GPU Acceleration

Build with GPU support for faster embedding:

### macOS (Metal)
```bash
cargo build --release --features metal
```

### NVIDIA (CUDA)
```bash
cargo build --release --features cuda
```

## Troubleshooting

### Daemon Not Running
```
Error: Could not connect to daemon
Run 'ch-cli daemon start' first and wait for models to load
```

**Solution:**
```bash
ch-cli daemon start
# Wait a few seconds for model to load
ch-cli embed
```

### Slow First Embedding
The first embedding request may be slow due to:
- Model download from HuggingFace (~50MB)
- Model loading into memory (~130MB)

Subsequent requests are much faster.

### Unexpected Search Results
If semantic search returns poor results:
```bash
# Force re-embed to rebuild the vector index
ch-cli embed --force
```

## See Also

- [Index Command](./index.md) - Build the code index
- [Search Command](./search.md) - Search for symbols
- [Semantic Embeddings](../retrieval/embeddings.md) - Technical details on embeddings

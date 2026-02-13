# Daemon Command

Manage the ML model daemon required for semantic search and embeddings.

## Summary

The daemon is a background process that keeps ML models loaded in memory. It provides:
- **Embeddings** for semantic code search
- **Reranking** for result quality
- **Query expansion** for natural language queries

Without the daemon, each command would need to load models (~5-10 seconds). With the daemon running, requests complete in milliseconds.

## Syntax

```bash
rustean daemon <COMMAND>
```

## Subcommands

| Command | Description |
|---------|-------------|
| `start` | Start the model daemon in background |
| `stop` | Stop the running daemon |
| `status` | Show daemon status and loaded models |
| `restart` | Stop and start the daemon |
| `run` | Run daemon in foreground (internal use) |

## Subcommand Details

### start

Start the daemon in background mode.

```bash
rustean daemon start
```

**Behavior:**
- Loads all ML models into memory
- Creates Unix socket for IPC at `~/.rustean/ml.sock`
- Writes PID file at `~/.rustean/daemon.pid`
- Returns immediately (daemon runs in background)

**Startup time:** ~5-10 seconds for model loading

### stop

Gracefully stop the running daemon.

```bash
rustean daemon stop
```

**Behavior:**
- Sends shutdown signal to daemon
- Unloads all models from memory
- Removes socket and PID files
- Frees ~6GB of memory

### status

Show current daemon status.

```bash
rustean daemon status
```

**Output includes:**
- Running state (running/stopped)
- Process ID
- Loaded models and their memory usage
- Uptime
- Socket path

**Example output:**
```
Daemon Status: Running

Process ID: 12345
Uptime: 2h 15m 30s
Socket: ~/.rustean/ml.sock

Loaded Models:
  - bge-small-en-v1.5 (embeddings): 130MB
  - bge-reranker-base (reranking): 1.1GB
  - phi-3-mini (query expansion): 4GB

Total Memory: ~6GB
```

### restart

Stop and start the daemon.

```bash
rustean daemon restart
```

**Use cases:**
- Apply configuration changes
- Recover from errors
- Update to new model versions

### run

Run the daemon in foreground (internal use).

```bash
rustean daemon run
```

**Note:** This is used internally by `start`. Not intended for direct use.

## Usage Examples

### Basic Operations

```bash
# Start the daemon
rustean daemon start

# Check if running
rustean daemon status

# Stop when done
rustean daemon stop
```

### Before Semantic Search

```bash
# Start daemon first
rustean daemon start

# Wait for models to load (~5-10 seconds)
# Then run semantic search
rustean retrieve "how does authentication work"

# Or use semantic embed
rustean embed --text "authentication handler"
```

### Quick Status Check

```bash
# Check if daemon is running
rustean daemon status

# If not running, start it
rustean daemon start
```

### Restart After Issues

```bash
# If daemon becomes unresponsive
rustean daemon restart

# Or stop and start manually
rustean daemon stop
rustean daemon start
```

## When Daemon is Required

The daemon must be running for these commands:

| Command | Requires Daemon | Feature |
|---------|-----------------|---------|
| `rustean retrieve` | Yes | Semantic code retrieval |
| `rustean embed` | Yes | Text embedding generation |
| `rustean search --semantic` | Yes | Semantic symbol search |

**Commands that work without daemon:**
- `rustean index` - Basic indexing
- `rustean search` - Symbol name search (non-semantic)
- `rustean goto` - Go to definition
- `rustean refs` - Find references
- `rustean info` - Detailed symbol info (code, callers, refs)
- `rustean symbols` - List symbols
- `rustean stats` - Index statistics

## Auto-Start Behavior

When you run commands requiring the daemon:
1. **First request:** Daemon starts automatically (~5-10 seconds)
2. **Subsequent requests:** Instant (~10-100 milliseconds)

The daemon stays running until explicitly stopped.

## Resource Usage

### Memory Requirements

| Model | Purpose | Memory |
|-------|---------|--------|
| bge-small-en-v1.5 | Embeddings | ~130MB |
| bge-reranker-base | Reranking | ~1.1GB |
| phi-3-mini | Query expansion | ~4GB |
| **Total** | | **~6GB** |

### Startup Time

- Initial model download: ~2-5 minutes (first time only)
- Model loading: ~5-10 seconds
- After loading: <100ms per request

### File Locations

| File | Purpose |
|------|---------|
| `~/.rustean/ml.sock` | Unix socket for IPC |
| `~/.rustean/daemon.pid` | Process ID file |
| `~/.rustean/models/` | Cached model weights |

## Troubleshooting

### Daemon Won't Start

**Symptom:** `rustean daemon start` fails or hangs

**Solutions:**

1. Check for stale files:
```bash
ls -la ~/.rustean/ml.sock ~/.rustean/daemon.pid
```

2. Remove stale files and retry:
```bash
rm -f ~/.rustean/ml.sock ~/.rustean/daemon.pid
rustean daemon start
```

3. Check if port/socket is in use:
```bash
lsof ~/.rustean/ml.sock
```

### Daemon Not Responding

**Symptom:** Commands hang or return "connection refused"

**Solutions:**

1. Check daemon status:
```bash
rustean daemon status
```

2. If status shows running but commands fail, restart:
```bash
rustean daemon restart
```

3. Force cleanup and restart:
```bash
rustean daemon stop
rm -f ~/.rustean/ml.sock ~/.rustean/daemon.pid
rustean daemon start
```

### High Memory Usage

**Symptom:** System slow or out of memory

**Solutions:**

1. Stop daemon when not needed:
```bash
rustean daemon stop
```

2. Check current memory usage:
```bash
rustean daemon status
```

3. The daemon uses ~6GB. Ensure sufficient RAM available.

### Slow First Request

**Symptom:** First command takes 5-10 seconds

**Explanation:** This is normal. Models are loading on first request.

**Optimization:**
```bash
# Pre-start daemon before working
rustean daemon start

# Wait for status to show "Running" with all models loaded
rustean daemon status
```

### Models Not Downloading

**Symptom:** Daemon starts but fails to load models

**Solutions:**

1. Check internet connection
2. Check disk space in `~/.rustean/models/`
3. Remove partial downloads:
```bash
rm -rf ~/.rustean/models/*
rustean daemon restart
```

## Common Workflows

### Workflow 1: Starting Semantic Search Session

```bash
# 1. Start daemon
rustean daemon start

# 2. Verify it's running
rustean daemon status

# 3. Run semantic searches
rustean retrieve "how does the parser work"
rustean retrieve "error handling patterns"

# 4. Stop when done (optional)
rustean daemon stop
```

### Workflow 2: Checking if Daemon is Running

```bash
# Quick check
rustean daemon status

# If not running:
# - Status shows "Daemon Status: Stopped"
# - Start it: rustean daemon start

# If running:
# - Status shows "Daemon Status: Running"
# - Shows loaded models and memory
```

### Workflow 3: Recovering from Issues

```bash
# 1. Try restart first
rustean daemon restart

# 2. If restart fails, force cleanup
rustean daemon stop
rm -f ~/.rustean/ml.sock ~/.rustean/daemon.pid

# 3. Start fresh
rustean daemon start

# 4. Verify recovery
rustean daemon status
```

### Workflow 4: Minimal Memory Mode

```bash
# When not using semantic features, stop daemon to free memory
rustean daemon stop

# Use non-semantic commands
rustean search MyStruct
rustean goto MyStruct
rustean refs my_function

# Start daemon only when needed
rustean daemon start
rustean retrieve "authentication flow"
rustean daemon stop
```

## Architecture

The daemon uses a client-server architecture with Unix sockets:

```
+----------------------------------+
|        Model Daemon              |
|  +----------+  +-----------+     |
|  | Embedder |  | Reranker  |     |
|  +----------+  +-----------+     |
|        +------------+            |
|        | LLM (Phi-3)|            |
|        +------------+            |
|              |                   |
|     +--------+--------+          |
|     |  Unix Socket    |          |
|     | ~/.rustean/ml.sock          |
+-----|----------------+-----------+
      |
      +---------------------------+
      |            |              |
 +----+----+ +-----+-----+ +------+------+
 | retrieve| |  embed    | |search --sem |
 +---------+ +-----------+ +-------------+
```

## See Also

- [Search Command](./search.md) - Symbol search with semantic option
- [Index Command](./index.md) - Building code indexes
- [Retrieval Documentation](../retrieval/daemon.md) - Technical daemon details
- [Getting Started](../getting-started/) - First time setup

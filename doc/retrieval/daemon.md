# Model Daemon

## Summary

The model daemon is a background process that keeps ML models loaded in memory to avoid reload latency on each request. Instead of loading models (which takes 2-5 seconds) every time you run a command, the daemon loads them once and handles all subsequent requests instantly.

## How It Works

```
┌─────────────────────────────────────────────────────────┐
│                    Model Daemon                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐  │
│  │ BGE Embedder│  │ Phi-3 LLM   │  │ BGE Reranker    │  │
│  │ (~130MB)    │  │ (~4GB)      │  │ (~1.1GB)        │  │
│  └─────────────┘  └─────────────┘  └─────────────────┘  │
│                          │                               │
│                   ┌──────┴──────┐                        │
│                   │ Unix Socket │                        │
│                   │ ~/.ch-cli/  │                        │
│                   │  ml.sock    │                        │
│                   └──────┬──────┘                        │
└──────────────────────────┼──────────────────────────────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
   ┌────┴────┐       ┌─────┴─────┐      ┌─────┴─────┐
   │ch-cli   │       │ch-cli     │      │ch-cli     │
   │embed    │       │search     │      │retrieve   │
   └─────────┘       └───────────┘      └───────────┘
```

## CLI Commands

### Start the daemon
```bash
ch-cli daemon start
```
Starts the daemon in the background. Models are loaded on startup (~5-10 seconds).

### Stop the daemon
```bash
ch-cli daemon stop
```
Gracefully stops the daemon and unloads all models.

### Check status
```bash
ch-cli daemon status
```
Shows:
- Whether daemon is running
- Process ID
- Loaded models
- Memory usage
- Uptime

### Restart the daemon
```bash
ch-cli daemon restart
```
Stops and starts the daemon to reload models.

## Auto-Start Behavior

When you run commands like `ch-cli embed` or `ch-cli search --semantic`, the daemon is automatically started if it's not running. This means:

1. First command: ~5-10 seconds (daemon startup + model loading)
2. Subsequent commands: ~10-100 milliseconds (instant)

## File Locations

| File | Purpose |
|------|---------|
| `~/.ch-cli/ml.sock` | Unix socket for IPC |
| `~/.ch-cli/daemon.pid` | Process ID file |
| `~/.ch-cli/models/` | Cached model weights |

## IPC Protocol

The daemon uses JSON over Unix sockets for communication:

### Request Types
- `Embed`: Generate embeddings for text chunks
- `Rerank`: Rerank documents given a query
- `Expand`: Expand a natural language query
- `Status`: Get daemon status
- `Shutdown`: Stop the daemon
- `Ping`: Health check

### Response Types
- `Embeddings`: Vector embeddings (384-dim for BGE-small)
- `Scores`: Relevance scores for reranking
- `SearchSpec`: Expanded search specification
- `Status`: Daemon status information
- `Ok`: Operation succeeded
- `Pong`: Response to ping
- `Error`: Error message

## Memory Usage

| Model | Purpose | Memory |
|-------|---------|--------|
| bge-small-en-v1.5 | Embeddings | ~130MB |
| bge-reranker-base | Reranking | ~1.1GB |
| phi-3-mini | Query expansion | ~4GB |
| **Total** | | **~6GB** |

## Configuration

Default configuration in `RetrievalConfig`:

```rust
RetrievalConfig {
    socket_path: "~/.ch-cli/ml.sock",
    pid_file: "~/.ch-cli/daemon.pid",
    model_cache: "~/.ch-cli/models",
    embedding_model: "BAAI/bge-small-en-v1.5",
    reranker_model: "BAAI/bge-reranker-base",
    expansion_model: "microsoft/phi-3-mini-4k-instruct",
}
```

## Troubleshooting

### Daemon won't start
1. Check if another process is using the socket: `ls -la ~/.ch-cli/ml.sock`
2. Remove stale files: `rm ~/.ch-cli/ml.sock ~/.ch-cli/daemon.pid`
3. Try starting again: `ch-cli daemon start`

### High memory usage
- The daemon loads all models on startup (~6GB total)
- Stop the daemon when not needed: `ch-cli daemon stop`

### Connection refused
1. Check if daemon is running: `ch-cli daemon status`
2. If stale PID file, clean up and restart:
   ```bash
   ch-cli daemon stop
   ch-cli daemon start
   ```

## Architecture

### Components

| File | Purpose |
|------|---------|
| `daemon/mod.rs` | Module exports |
| `daemon/protocol.rs` | IPC message types |
| `daemon/server.rs` | Background server |
| `daemon/client.rs` | Client API |
| `daemon/lifecycle.rs` | Start/stop/status |

### Source Files

- `src/retrieval/daemon/protocol.rs`: DaemonRequest, DaemonResponse enums
- `src/retrieval/daemon/server.rs`: ModelDaemon struct
- `src/retrieval/daemon/client.rs`: DaemonClient struct
- `src/retrieval/daemon/lifecycle.rs`: Process management functions

# Daemon Socket Error: Resource Temporarily Unavailable (OS Error 35)

**Date**: 2026-02-06
**Status**: Open
**Severity**: Medium (workaround available)

---

## Error Description

When running `rustean docs generate`, the command fails with:

```
[daemon-client] Retry 1/3 after 100ms
[daemon-client] Retry 2/3 after 200ms
[daemon-client] Retry 3/3 after 400ms
Error starting doc generation: IO error: Resource temporarily unavailable (os error 35)
Make sure the daemon is running: rustean daemon start
```

## Root Cause

**OS Error 35 = EAGAIN (Resource temporarily unavailable)**

This is a Unix socket error that occurs when:
1. The daemon's socket listen backlog is full
2. The daemon is busy processing other requests (model loading)
3. Too many concurrent connection attempts

The daemon uses a Unix domain socket at:
```
/Users/charlesleroy/Library/Application Support/com.rustean.rustean/ml.sock
```

When the daemon is loading models (embeddings, reranker, Phi-3), it can't accept new connections fast enough, causing EAGAIN.

## Current Retry Logic

Location: `src/retrieval/daemon/client/retry.rs`

```rust
pub const DEFAULT_RETRIES: u32 = 3;
pub const DEFAULT_INITIAL_DELAY_MS: u64 = 100;
pub const DEFAULT_MAX_DELAY_MS: u64 = 2000;
```

The retry delays are: 100ms → 200ms → 400ms (total ~700ms)

This is insufficient when the daemon is loading the Phi-3 model (~10+ seconds).

## Affected Commands

| Command | Status | Notes |
|---------|--------|-------|
| `docs generate` | **FAILS** | Requires daemon for LLM generation |
| `docs status` | Works | Falls back to local store |
| `docs show` | Works | Falls back to local store |
| `docs search` | Works | Falls back to local store |

## Workaround

Use `rustean info <symbol>` instead. It provides:
- Doc comments from source
- Source code snippets
- Callers (who calls this symbol)
- Callees (what this symbol calls)
- All references

Example:
```bash
rustean info search_command --all
```

## Potential Fixes

### Option 1: Increase Retry Timeout
```rust
// In retry.rs
pub const DEFAULT_RETRIES: u32 = 10;
pub const DEFAULT_INITIAL_DELAY_MS: u64 = 500;
pub const DEFAULT_MAX_DELAY_MS: u64 = 10000;
```

### Option 2: Wait for Daemon Ready
Before sending doc generation request, poll daemon status until models are loaded:
```rust
while !client.ping().is_ok() {
    std::thread::sleep(Duration::from_secs(1));
}
```

### Option 3: Queue Requests Server-Side
Modify daemon to queue incoming requests instead of rejecting with EAGAIN:
- Increase socket backlog: `listener.set_nonblocking(false)`
- Add request queue with bounded channel

### Option 4: Connection Pooling
Reuse socket connections instead of creating new ones per request:
```rust
struct ConnectionPool {
    connections: Vec<UnixStream>,
    max_size: usize,
}
```

## Files Involved

- `src/retrieval/daemon/client/connection.rs` - Socket connection logic
- `src/retrieval/daemon/client/retry.rs` - Retry configuration
- `src/retrieval/daemon/server/lifecycle.rs` - Server socket setup
- `src/retrieval/daemon/server/client_handler.rs` - Request handling

## Test to Reproduce

```bash
# 1. Kill any existing daemon
rustean daemon stop

# 2. Start daemon (will load models in background)
rustean daemon start

# 3. Immediately try docs generate (while models loading)
rustean docs generate

# Expected: Error 35
# This is because daemon socket is busy during model loading
```

## Related Issues

- Daemon model loading takes ~10-30 seconds
- Socket backlog may be too small for concurrent requests
- No connection pooling implemented

---

## Appendix: Error Codes

| Error Code | Name | Description |
|------------|------|-------------|
| 35 | EAGAIN | Resource temporarily unavailable |
| 54 | ECONNRESET | Connection reset by peer |
| 61 | ECONNREFUSED | Connection refused |

## References

- Unix socket programming: `man 2 accept`
- Rust UnixStream: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html

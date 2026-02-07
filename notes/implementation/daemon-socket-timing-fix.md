# Daemon Socket Timing Fix

## Summary

Fixed a critical issue where daemon clients would get "connection refused" errors during startup because the Unix socket was only created after all ML models finished loading (~60-80 seconds).

## Problem

**Symptom**: `ch-cli daemon status` showed "Running but unreachable" during startup.

**Root Cause**: In `server.rs`, the `run()` method loaded all ML models before creating the Unix socket listener:

```rust
// OLD (broken) sequence:
pub fn run(&mut self) -> RetrievalResult<()> {
    self.load_models()?;           // 60-80 seconds
    let listener = UnixListener::bind(&self.socket_path)?;  // socket created AFTER
    // ...
}
```

This meant clients couldn't connect for the entire model loading duration.

## Solution

Create the socket BEFORE loading models, and handle limited requests (ping/status) during loading:

```rust
// NEW (fixed) sequence:
pub fn run(&mut self) -> RetrievalResult<()> {
    let listener = UnixListener::bind(&self.socket_path)?;  // socket created FIRST
    listener.set_nonblocking(true)?;
    self.load_models_with_listener(&listener)?;  // handles ping/status during load
    // ...
}
```

## Changes Made

### 1. Added `handle_status()` method
Extracted status handling for reuse in both full and limited modes.

### 2. Added `load_models_with_listener()`
Loads models while periodically checking for and handling ping/status requests.

### 3. Added `handle_client_limited()`
Only handles ping/status/shutdown requests during model loading phase. Other requests get "Daemon is still loading models" error.

### 4. Modified `run()` method
- Creates socket before model loading
- Sets socket to non-blocking during load phase
- Uses new `load_models_with_listener()` instead of `load_models()`

### 5. Removed unused `load_models()`
Replaced by the new function.

## Behavior After Fix

| Phase | Duration | Available Commands |
|-------|----------|-------------------|
| Socket creation | Instant | - |
| Model loading | ~60-80s | ping, status, shutdown |
| Fully loaded | Ongoing | All commands |

## Testing

- All 129 unit tests pass
- All 13 integration tests pass
- Manual testing confirms socket available immediately

## Files Modified

- `src/retrieval/daemon/server.rs`

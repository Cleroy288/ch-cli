# First-Launch Setup - Implementation Notes

## Date: 2026-02-06

## Status Summary

| Feature | Code | Tests | Manual Test |
|---------|------|-------|-------------|
| Non-blocking doc gen (daemon) | DONE | PASS (284/284) | PASS |
| Blocking socket after model load | DONE | PASS | PASS |
| Retry config (8/500ms/5000ms) | DONE | PASS | PASS |
| Protocol `in_progress` field | DONE | PASS | PASS |
| `docs status` shows bg state | DONE | PASS | PASS |
| First-launch 3-step wizard | DONE | compiles | NOT TESTED |
| Background incremental update | DONE | compiles | NOT TESTED |
| Daemon auto-start in wizard | NOT DONE | - | - |
| `sync_with_index()` for stale docs | NOT DONE | - | - |
| Unit tests for new functions | NOT DONE | - | - |
| Background thread cancellation | NOT DONE | - | - |

## Changes Made

### Phase 1: Fix Daemon

1. **`src/retrieval/daemon/server/mod.rs`**
   - Added `DocGenProgress` struct (total, completed, failed, is_running)
   - Changed `doc_stores` from `HashMap` to `Arc<Mutex<HashMap>>`
   - Changed `doc_generator` from `Option<DocGenerator>` to `Arc<Mutex<Option<DocGenerator>>>`
   - Added `doc_gen_progress: Arc<Mutex<DocGenProgress>>`
   - Added `doc_gen_thread: Option<JoinHandle<()>>`

2. **`src/retrieval/daemon/server/doc_handlers.rs`** (full rewrite)
   - `handle_start_doc_gen()`: checks if running, populates store, spawns bg thread, returns immediately
   - `run_doc_gen_background()`: per-entry lock strategy (extract -> LLM no lock -> write back)
   - `handle_doc_gen_status()`: reads `doc_gen_progress` when bg running (no store lock contention)
   - `handle_get_doc()`, `handle_search_docs()`: updated for mutex access

3. **`src/retrieval/daemon/server/lifecycle.rs`**
   - Added `listener.set_nonblocking(false)` after model loading
   - Simplified `run_main_loop()` to blocking accept (removed WouldBlock polling)

4. **`src/retrieval/daemon/client/retry.rs`**
   - max_retries: 3 -> 8
   - initial_delay_ms: 100 -> 500
   - max_delay_ms: 2000 -> 5000

5. **`src/retrieval/daemon/protocol.rs`**
   - Added `in_progress: bool` to `DocGenStatus` response variant

6. **`src/retrieval/daemon/client/doc_requests.rs`**
   - `start_doc_gen` return type: `()` -> `DocGenStatus`
   - Added `in_progress` field to client `DocGenStatus`

### Phase 2: First-Launch Setup

7. **`src/startup/mod.rs`**
   - Added `IndexAndGenerateDocs` variant to `StartupAction`
   - Split `run_startup()` into `handle_existing_index()` and `handle_first_launch()`
   - Added `spawn_background_doc_update()` for subsequent launches

8. **`src/startup/progress_docgen.rs`** (NEW - 5 functions)
   - `generate_docs_with_progress()` - main entry point
   - `display_daemon_wait()` - shows "Step 2/3" message
   - `wait_for_daemon()` - polls ping with 120s timeout
   - `display_docgen_header()` - shows "Step 3/3" message
   - `poll_docgen_progress()` - progress bar with Ctrl+C cancellation
   - `display_docgen_complete()` - shows "Setup complete!" message

9. **`src/startup/prompts_analysis.rs`**
   - Updated prompt: mentions doc generation, shows estimated time

### Phase 3: Background Updates

10. `spawn_background_doc_update()` in `handle_existing_index()` - sends `StartDocGen` to daemon in detached thread after incremental index

### Other

11. **`src/indexer/semantic/mod.rs`** - Added `#[derive(Clone)]` to `SemanticGraph`
12. **`src/cli/commands/docs.rs`** - Updated status display for `in_progress` state
13. **`tests/retrieval/daemon_client_tests.rs`** - Updated retry config assertions

## Key Fix: Lock Contention (discovered during manual testing)

**Problem**: First implementation held `stores` and `generator` mutex during LLM inference (~1s per entry). Main thread blocked on `doc_stores.lock()` in `handle_doc_gen_status()`.

**Fix**:
1. Per-entry processing: brief lock extract -> LLM with no lock held -> brief lock write back
2. `handle_doc_gen_status` reads `doc_gen_progress` (never contended) when bg gen is running
3. Doc generator model init (~30s) moved into background thread

**Result**: `docs generate` returns in 6ms. `docs status` responds in 6ms during generation.

## Manual Test Results

```
docs generate:          6ms   (was blocking for minutes)
docs generate --force:  6ms   (second run, store populated)
docs status:            6ms   while gen running, shows progress
docs show <symbol>:     7ms   while gen running
daemon status:          6ms   while gen running
info <symbol> --all:    443ms works concurrently (includes local indexing)
cargo test:             284 pass, 0 fail
```

## Known Gaps

1. **No daemon auto-start**: `generate_docs_with_progress()` waits for daemon but doesn't start it
2. **No `sync_with_index()`**: Changed functions keep old docs (only new symbols get docs)
3. **No unit tests**: For `DocGenProgress`, `run_doc_gen_background`, `progress_docgen.rs`, startup flow
4. **No bg thread cancellation**: Thread runs until done or process exits
5. **First-launch flow not interactively tested**: Requires deleting `.rustean-index/` and running TUI

# First-Launch Setup + Background Doc Updates

## Summary

On first launch, rustean guides the user through a 3-step setup (index, daemon, doc gen) with progress bars. On subsequent launches, changes are detected and docs are updated in the background without blocking the user. The daemon no longer blocks during doc generation.

---

## What Is Implemented

### 1. Non-Blocking Doc Generation (Daemon) - DONE, TESTED

The daemon's `handle_start_doc_gen()` now spawns a background thread and returns immediately. Shared state (`doc_stores`, `doc_generator`, `doc_gen_progress`) is wrapped in `Arc<Mutex<>>`. Locks are held briefly (extract entry -> LLM with no lock -> write back) so the main thread can serve other requests concurrently.

| Metric | Before | After |
|--------|--------|-------|
| `docs generate` return time | blocked for minutes | 6ms |
| `docs status` while gen runs | OS Error 35 (timeout) | 6ms |
| `docs show <symbol>` while gen runs | OS Error 35 | 7ms |
| `daemon status` while gen runs | OS Error 35 | 6ms |

**Files**:
- `src/retrieval/daemon/server/mod.rs` - `DocGenProgress` struct, `Arc<Mutex<>>` shared fields
- `src/retrieval/daemon/server/doc_handlers.rs` - Background thread, lock-free status polling

### 2. Blocking Socket After Model Loading - DONE, TESTED

After `load_models_with_listener()`, socket switches to blocking mode. Main loop uses `accept()` without the old `WouldBlock` polling sleep.

**File**: `src/retrieval/daemon/server/lifecycle.rs`

### 3. Retry Config - DONE, TESTED

| Parameter | Before | After |
|-----------|--------|-------|
| `max_retries` | 3 | 8 |
| `initial_delay_ms` | 100 | 500 |
| `max_delay_ms` | 2000 | 5000 |
| Total retry window | ~700ms | ~30s |

**File**: `src/retrieval/daemon/client/retry.rs`

### 4. Protocol Update - DONE, TESTED

Added `in_progress: bool` to `DaemonResponse::DocGenStatus`. `start_doc_gen` client function now returns `DocGenStatus` instead of `()`.

**Files**:
- `src/retrieval/daemon/protocol.rs`
- `src/retrieval/daemon/client/doc_requests.rs`
- `src/retrieval/daemon/client/mod.rs`

### 5. First-Launch Setup Flow - DONE, NOT MANUALLY TESTED

Code is written and compiles. The flow is:
```
prompt (mentions doc gen + estimated time)
  -> [1/3] Index codebase (existing index_with_progress)
  -> [2/3] Wait for daemon readiness (polls ping, 120s timeout)
  -> [3/3] Poll DocGenStatus with progress bar (Ctrl+C to skip)
  -> "Setup complete!" message
  -> TUI
```

**Not manually tested** because testing requires deleting `.rustean-index/` and running the TUI interactively. The code compiles, the individual pieces (daemon polling, doc gen, progress display) are tested separately.

**Files**:
- `src/startup/mod.rs` - `handle_first_launch()` orchestration
- `src/startup/progress_docgen.rs` - NEW, progress bar with daemon polling
- `src/startup/prompts_analysis.rs` - Updated prompt text

### 6. Background Incremental Updates - DONE, NOT MANUALLY TESTED

On subsequent launches with changes, after incremental index, `spawn_background_doc_update()` fires a `StartDocGen` to the daemon in a detached thread. User enters TUI immediately.

**Not manually tested** because it requires running the TUI interactively. The background thread itself is the same `start_doc_gen` path that is tested.

**File**: `src/startup/mod.rs`

### 7. `docs status` CLI - DONE, TESTED

Updated to show `in_progress` state: "Generation running in background..." vs "Generation not started".

**File**: `src/cli/commands/docs.rs`

### 8. SemanticGraph Clone - DONE, TESTED

Added `#[derive(Clone)]` to `SemanticGraph` so the background thread can own a copy for cross-reference building.

**File**: `src/indexer/semantic/mod.rs`

---

## What Is NOT Implemented

### 1. Daemon Auto-Start in First-Launch Flow

The `progress_docgen.rs` calls `wait_for_daemon()` which polls `ping()` for up to 120s, but **does not start the daemon itself**. If no daemon is running when the first-launch wizard reaches step 2/3, it will time out and skip doc generation.

**Current behavior**: Relies on the daemon being pre-warmed by `prewarm_daemon_if_needed()` in `main.rs` (which spawns a background start for TUI launches). If the daemon takes longer than the indexing step to start, the wizard will wait. If no daemon starts at all, it shows "Daemon not available. Skipping doc generation." and continues to TUI.

**What would be needed**: Explicitly call `ensure_daemon_ready()` or `prewarm_daemon()` at the start of `generate_docs_with_progress()`.

### 2. DocStore `sync_with_index()` in Incremental Path

The plan mentions: "Ensure `sync_with_index()` correctly marks only changed entries as Pending". The `spawn_background_doc_update()` sends `StartDocGen` with `force: false`, which calls `populate_from_symbols()` on the daemon side. But it does **not call `sync_with_index()`** to mark stale entries as Pending based on file modification times.

**Current behavior**: Only new symbols get docs generated. If a function body changes (same name, same line), its existing doc won't be regenerated.

**What would be needed**: Call `store.sync_with_index(&index_state)` before `populate_from_symbols()` in `handle_start_doc_gen()`, or pass the index state to the daemon.

### 3. Unit Tests for New Code

No new unit tests were written for:
- `DocGenProgress` struct
- `run_doc_gen_background()` function
- `generate_docs_with_progress()` function
- `spawn_background_doc_update()` function
- `handle_first_launch()` / `handle_existing_index()` functions

Existing tests (284 total) all pass. The retry config test was updated.

### 4. Cancellation / Graceful Shutdown of Background Doc Gen

The background doc gen thread has no way to be cancelled. If the daemon receives a `Shutdown` request, it exits but the background thread may still be running (it will terminate when the process dies). There is no explicit signal to stop the background thread mid-generation.

### 5. Multiple Concurrent Doc Gen Requests

If `StartDocGen` is called while a previous generation is already running, it returns the current progress. But the old `doc_gen_thread` `JoinHandle` is silently dropped (overwritten by `daemon.doc_gen_thread = Some(handle)`). This is safe (the old thread continues to run) but means only the latest thread handle is tracked.

---

## Architecture

```
First launch (no .rustean-index/):
  prompt -> [1/3] Index -> [2/3] Daemon Ready -> [3/3] Doc Gen Progress -> TUI

Subsequent launch (index exists, changes detected):
  prompt -> Incremental Index (blocking) -> Background Doc Update (async) -> TUI

Subsequent launch (no changes):
  -> TUI directly
```

## Data Flow

```
Client                    Daemon
  |-- StartDocGen -------->|
  |<--- DocGenStatus ------|  (in_progress: true, returns in 6ms)
  |                        |  [background thread: init model -> process entries]
  |-- DocGenStatus ------->|
  |<--- DocGenStatus ------|  (completed: N, total: M, in_progress: true)
  |          ...           |
  |-- DocGenStatus ------->|
  |<--- DocGenStatus ------|  (is_ready: true, in_progress: false)
```

## Lock Strategy

```
Background thread (per entry):
  1. stores.lock()    -> clone entry     -> drop lock
  2. generator.lock() -> LLM inference   -> drop lock   (main thread can serve requests here)
  3. stores.lock()    -> upsert result   -> drop lock
  4. progress.lock()  -> update counters -> drop lock

Main thread (status query):
  1. progress.lock()  -> read counters   -> drop lock   (never contended)
  2. If not running: stores.lock() -> read stats -> drop lock
```

## Files Modified

| File | Change |
|------|--------|
| `src/retrieval/daemon/server/mod.rs` | `DocGenProgress`, `Arc<Mutex<>>` shared fields |
| `src/retrieval/daemon/server/doc_handlers.rs` | Background thread, lock-free status |
| `src/retrieval/daemon/server/lifecycle.rs` | Blocking socket after model load |
| `src/retrieval/daemon/client/retry.rs` | 8 retries, 500ms initial |
| `src/retrieval/daemon/client/doc_requests.rs` | Return `DocGenStatus` from start |
| `src/retrieval/daemon/client/mod.rs` | Updated return type |
| `src/retrieval/daemon/protocol.rs` | Added `in_progress` field |
| `src/startup/mod.rs` | Startup flow orchestration |
| `src/startup/progress_docgen.rs` | NEW - doc gen progress bar |
| `src/startup/prompts_analysis.rs` | Updated prompt text |
| `src/indexer/semantic/mod.rs` | `#[derive(Clone)]` on `SemanticGraph` |
| `src/cli/commands/docs.rs` | Updated status display |
| `tests/retrieval/daemon_client_tests.rs` | Updated retry assertions |

## Conclusion

The core problem (daemon blocking during doc gen) is fully solved and manually verified. The first-launch wizard and background incremental updates are implemented but need interactive manual testing to verify the full UX flow. See "What Is NOT Implemented" for remaining gaps.

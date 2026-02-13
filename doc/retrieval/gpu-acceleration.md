# GPU Acceleration

## Summary

rustean supports GPU acceleration for ML models via Metal (macOS) and CUDA (Linux).
Runtime detection automatically selects the best available device with fallback to CPU.

## Supported Backends

| Backend | Platform | Hardware |
|---------|----------|----------|
| Metal | macOS | Apple Silicon (M1/M2/M3) |
| CUDA | Linux/Windows | NVIDIA GPUs |
| CPU | All | Any x86/ARM CPU |

## Build Configuration

### CPU Only (Default)

```bash
cargo build --release
```

### Metal (macOS Apple Silicon)

```bash
cargo build --release --features metal
```

### CUDA (Linux with NVIDIA GPU)

```bash
cargo build --release --features cuda
```

## Runtime Device Detection

The daemon automatically detects available GPU at startup:

1. Checks `CH_FORCE_CPU` environment variable
2. If `metal` feature enabled: tries Metal GPU
3. If `cuda` feature enabled: tries CUDA GPU
4. Falls back to CPU on any failure

### Detection Flow

```
Start
  |
  v
CH_FORCE_CPU=1? --yes--> CPU
  |no
  v
Metal feature? --yes--> Try Metal
  |no                      |
  |                        v
  |                   Metal OK? --yes--> Metal GPU
  |                        |no
  |                        v
  |                      CPU (fallback)
  v
CUDA feature? --yes--> Try CUDA
  |no                     |
  |                       v
  |                   CUDA OK? --yes--> CUDA GPU
  |                       |no
  |                       v
  |                     CPU (fallback)
  v
CPU
```

## Environment Variables

| Variable | Description | Values |
|----------|-------------|--------|
| `CH_FORCE_CPU` | Force CPU mode (disable GPU) | `1`, `true` |

### Example: Force CPU Mode

```bash
CH_FORCE_CPU=1 ./rustean daemon start
```

## Daemon Status

View current device in daemon status:

```bash
./rustean daemon status
```

Output:

```
Daemon Status: Running
PID:           54449
Device:        Metal (Apple Silicon GPU)
GPU Memory:    16384 MB
Uptime:        120 seconds

Loaded Models:
  - BAAI/bge-small-en-v1.5 (embeddings)
  - BAAI/bge-reranker-base (reranker)
  - microsoft/phi-3-mini-4k-instruct (query expansion)
```

## Performance

Expected speedups with GPU acceleration:

| Operation | CPU | GPU | Speedup |
|-----------|-----|-----|---------|
| Embedding batch | ~50ms | ~10ms | 5x |
| Reranking (10 docs) | ~200ms | ~40ms | 5x |
| Query Expansion | ~5-8s | ~1-2s | 4x |
| **Full Pipeline** | **~9s** | **~2s** | **4-5x** |

## API Reference

### DeviceType Enum

```rust
pub enum DeviceType {
    Cpu,    // CPU computation
    Metal,  // Apple Metal GPU
    Cuda,   // NVIDIA CUDA GPU
}
```

### DeviceInfo Struct

```rust
pub struct DeviceInfo {
    pub device_type: DeviceType,
    pub device_name: String,
    pub memory_mb: Option<u64>,
}
```

### Functions

- `get_device()` - Returns best available Device
- `get_device_info()` - Returns DeviceInfo with details
- `is_force_cpu_mode()` - Checks CH_FORCE_CPU env var

## Troubleshooting

### Metal Not Detected

1. Ensure built with `--features metal`
2. Check macOS version (10.15+ required)
3. Verify Apple Silicon or AMD GPU

### CUDA Not Detected

1. Ensure built with `--features cuda`
2. Install NVIDIA drivers
3. Install CUDA toolkit
4. Set `LD_LIBRARY_PATH` to CUDA libs

### Forcing CPU for Testing

```bash
CH_FORCE_CPU=1 ./rustean daemon restart
```

## Implementation Details

### Files Created

| File | Description | Lines |
|------|-------------|-------|
| `src/retrieval/models/device.rs` | Device detection module with DeviceType enum, DeviceInfo struct, and detection functions | ~200 |
| `doc/retrieval/gpu-acceleration.md` | This documentation file | ~200 |
| `notes/implementation/gpu-acceleration.txt` | Implementation notes | ~50 |

### Files Modified

| File | Changes |
|------|---------|
| `src/retrieval/models/mod.rs` | Added `pub mod device`, exported `DeviceInfo` and `DeviceType`, updated `get_device()` to use device module, added `get_device_info()` |
| `src/retrieval/daemon/protocol.rs` | Added `DeviceStatus` struct, updated `DaemonStatus` to use `device` field instead of `memory_bytes` |
| `src/retrieval/daemon/server.rs` | Added `device_info` field, device logging on startup, `DeviceStatus` in status response |
| `src/retrieval/daemon/lifecycle.rs` | Updated `DaemonStatus` construction to use `DeviceStatus::default()` |
| `src/cli/commands.rs` | Updated `daemon_status_command()` to display device type, name, and GPU memory |

### Unit Tests

Located in `src/retrieval/models/device.rs`:

| Test | Description |
|------|-------------|
| `test_device_detection` | Verifies `detect_device()` returns valid DeviceInfo |
| `test_force_cpu_env_var` | Verifies CH_FORCE_CPU=1 and CH_FORCE_CPU=true handling |
| `test_fallback_to_cpu` | Verifies forced CPU mode returns CPU device |
| `test_device_type_display` | Verifies Display trait for DeviceType |

Run tests:

```bash
cargo test device
```

### Build Verification

```bash
# Check compilation
cargo check

# Run device tests
cargo test device

# Build with Metal
cargo build --release --features metal

# Build with CUDA
cargo build --release --features cuda
```

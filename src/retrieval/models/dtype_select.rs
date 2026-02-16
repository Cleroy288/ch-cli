//! Device-Aware DType Selection
//!
//! Selects optimal DType for embedding and reranker models
//! based on the detected compute device. Metal/CUDA use F16
//! for native hardware support (~2x speedup), CPU stays F32.

use candle_core::DType;

use super::device::DeviceType;

/// Select optimal DType for embedding/reranker models
/// Metal/CUDA -> F16 (native hw, ~2x speedup)
/// CPU -> F32 (F16 is emulated on CPU, slower)
pub fn embedding_dtype(device_type: DeviceType) -> DType {
	match device_type {
		DeviceType::Metal | DeviceType::Cuda => DType::F16,
		DeviceType::Cpu => DType::F32,
	}
}

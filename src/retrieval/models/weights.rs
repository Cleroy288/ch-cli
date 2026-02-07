//! Weight Loading Functions
//!
//! Utilities for loading model weights from safetensors files.

use std::path::PathBuf;

use candle_core::{DType, Device};
use candle_nn::VarBuilder;

use super::{ModelError, ModelResult};

/// Create a VarBuilder from downloaded model weights (single file)
pub fn load_weights(
	weights_path: &PathBuf,
	device: &Device,
) -> ModelResult<VarBuilder<'static>> {
	load_weights_multi(&[weights_path.clone()], device)
}

/// Create a VarBuilder from multiple weight files (for sharded models)
pub fn load_weights_multi(
	weights_paths: &[PathBuf],
	device: &Device,
) -> ModelResult<VarBuilder<'static>> {
	let vb = unsafe {
		VarBuilder::from_mmaped_safetensors(
			weights_paths, DType::F32, device,
		)
		.map_err(|e| ModelError::WeightLoad(e.to_string()))?
	};
	Ok(vb)
}

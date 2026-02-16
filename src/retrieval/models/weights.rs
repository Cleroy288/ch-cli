//! Weight Loading Functions
//!
//! Utilities for loading model weights from safetensors files.

use std::path::{Path, PathBuf};

use candle_core::{DType, Device};
use candle_nn::VarBuilder;

use super::{ModelError, ModelResult};

/// Create a VarBuilder from downloaded model weights (single file)
pub fn load_weights(
	weights_path: &Path,
	device: &Device,
) -> ModelResult<VarBuilder<'static>> {
	load_weights_multi(
		&[weights_path.to_path_buf()],
		device,
	)
}

/// Create a VarBuilder from multiple weight files (for sharded models)
pub fn load_weights_multi(
	weights_paths: &[PathBuf],
	device: &Device,
) -> ModelResult<VarBuilder<'static>> {
	let var_builder = unsafe {
		VarBuilder::from_mmaped_safetensors(
			weights_paths,
			DType::F32,
			device,
		)
		.map_err(|err| {
			ModelError::WeightLoad(err.to_string())
		})?
	};
	Ok(var_builder)
}

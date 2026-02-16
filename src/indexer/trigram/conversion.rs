//! Conversion functions for trigram serialization.

use crate::indexer::trigram::types::Trigram;

/// Convert trigram to hex string for serialization
pub fn trigram_to_string(tri: &Trigram) -> String {
	format!("{:02x}{:02x}{:02x}", tri[0], tri[1], tri[2])
}

/// Convert hex string back to trigram
pub fn string_to_trigram(hex: &str) -> Option<Trigram> {
	if hex.len() != 6 {
		return None;
	}

	let byte0 = u8::from_str_radix(&hex[0..2], 16).ok()?;
	let byte1 = u8::from_str_radix(&hex[2..4], 16).ok()?;
	let byte2 = u8::from_str_radix(&hex[4..6], 16).ok()?;

	Some([byte0, byte1, byte2])
}

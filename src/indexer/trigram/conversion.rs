//! Conversion functions for trigram serialization.

use crate::indexer::trigram::types::Trigram;

/// Convert trigram to hex string for serialization
pub fn trigram_to_string(t: &Trigram) -> String {
	format!("{:02x}{:02x}{:02x}", t[0], t[1], t[2])
}

/// Convert hex string back to trigram
pub fn string_to_trigram(s: &str) -> Option<Trigram> {
	if s.len() != 6 {
		return None;
	}

	let b0 = u8::from_str_radix(&s[0..2], 16).ok()?; // first byte
	let b1 = u8::from_str_radix(&s[2..4], 16).ok()?; // second byte
	let b2 = u8::from_str_radix(&s[4..6], 16).ok()?; // third byte

	Some([b0, b1, b2])
}

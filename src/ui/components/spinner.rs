use std::time::SystemTime;

/// Braille spinner frames
const FRAMES: [char; 10] = [
	'\u{280B}', '\u{2819}', '\u{2839}', '\u{2838}',
	'\u{283C}', '\u{2834}', '\u{2826}', '\u{2827}',
	'\u{2807}', '\u{280F}',
];

/// Frame duration in milliseconds
const FRAME_MS: u128 = 100;

///
/// Derives frame from system time for smooth
/// animation without external state.
pub fn spinner_char() -> char {
	let ms = SystemTime::now()
		.duration_since(SystemTime::UNIX_EPOCH)
		.unwrap_or_default()
		.as_millis();
	let idx = (ms / FRAME_MS) as usize % FRAMES.len();
	FRAMES[idx]
}

///
/// Returns e.g. "Thinking..."
pub fn thinking_label() -> String {
	format!("{} Thinking...", spinner_char())
}

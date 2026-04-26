use rustean::ui::markdown::code_metrics::{
	LineCount, SatU16,
};

// -- SatU16 trait -----------------------------------------

#[test]
fn sat_u16_zero_returns_zero() {
	assert_eq!(0_usize.sat_u16(), 0_u16);
}

#[test]
fn sat_u16_normal_value() {
	assert_eq!(42_usize.sat_u16(), 42_u16);
}

#[test]
fn sat_u16_at_max_boundary() {
	let max = u16::MAX as usize;
	assert_eq!(max.sat_u16(), u16::MAX);
}

#[test]
fn sat_u16_above_max_saturates() {
	let above = u16::MAX as usize + 1;
	assert_eq!(above.sat_u16(), u16::MAX);
}

#[test]
fn sat_u16_usize_max_saturates() {
	assert_eq!(usize::MAX.sat_u16(), u16::MAX);
}

// -- LineCount trait --------------------------------------

#[test]
fn line_count_empty_returns_one() {
	assert_eq!("".line_count(), 1);
}

#[test]
fn line_count_single_line() {
	assert_eq!("hello".line_count(), 1);
}

#[test]
fn line_count_multiple_lines() {
	assert_eq!("a\nb\nc".line_count(), 3);
}

#[test]
fn line_count_trailing_newline() {
	// str::lines() ignores trailing newline
	assert_eq!("a\nb\n".line_count(), 2);
}

const GUTTER_FIXED: u16 = 7;
const BORDER_COL: u16 = 1;

#[must_use]
pub const fn digit_width(
	count: usize,
) -> usize {
	if count == 0 { return 1; }
	(count.ilog10() as usize) + 1
}

pub trait SatU16 {
	fn sat_u16(self) -> u16;
}

impl SatU16 for usize {
	fn sat_u16(self) -> u16 {
		if self > u16::MAX as usize {
			u16::MAX
		} else {
			self as u16
		}
	}
}

pub trait LineCount {
	/// Minimum 1 even for empty strings
	fn line_count(&self) -> u16;
}

impl LineCount for str {
	fn line_count(&self) -> u16 {
		let n = self.lines().count();
		if n == 0 { 1 } else { n.sat_u16() }
	}
}

#[must_use]
pub fn cursor_x_offset(
	line_count: usize,
) -> u16 {
	BORDER_COL
		+ GUTTER_FIXED
		+ digit_width(line_count) as u16
}

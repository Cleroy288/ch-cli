#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockIdx(pub usize);

impl BlockIdx {
	pub fn val(self) -> usize {
		self.0
	}
}

#[derive(
	Debug, Clone, Copy, PartialEq, Eq, Default,
)]
pub struct ScrollOffset(pub u16);

impl ScrollOffset {
	pub fn val(self) -> u16 {
		self.0
	}
}

#[derive(Debug, Clone)]
pub struct ReviewBlock {
	pub index: BlockIdx,
	pub lang: String,
	pub original: String,
	pub edited: String,
	pub file_path: Option<String>,
	/// Agent model that produced this block
	pub agent_source: Option<String>,
}

impl ReviewBlock {
	pub fn new(
		index: usize,
		lang: String,
		code: String,
		file_path: Option<String>,
	) -> Self {
		Self {
			index: BlockIdx(index),
			lang,
			original: code.clone(),
			edited: code,
			file_path,
			agent_source: None,
		}
	}

	pub fn is_modified(&self) -> bool {
		self.original != self.edited
	}
}

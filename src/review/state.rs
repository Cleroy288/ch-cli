/// Inline editable code blocks — core state.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use ratatui::layout::Rect;
use ratatui::text::Line;
use tui_textarea::TextArea;

use crate::domain::claude::ResponseIntent;
use crate::domain::review::{
	BlockIdx, ReviewBlock, ScrollOffset,
};
use crate::ui::markdown::code_block;
use crate::ui::markdown::sections::{
	parse_sections, ContentSection,
};

/// Keyboard focus target within inline blocks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlineFocus {
	CodeBlock(BlockIdx),
	Question(BlockIdx),
}

/// Whether a block is editable or frozen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockMode {
	Editable,
	Frozen,
}

pub struct InlineBlock {
	pub index: BlockIdx,
	pub lang: String,
	pub original: String,
	pub edited: String,
	pub file_path: Option<String>,
	/// Agent model that produced this block
	pub agent_source: Option<String>,
	pub editor: TextArea<'static>,
	pub question: String,
	pub answer: Option<String>,
	pub render_rect: Cell<Rect>,
	/// Cached highlighted + guttered lines
	highlight_cache:
		RefCell<Option<Rc<[Line<'static>]>>>,
}

pub struct InlineBlocks {
	pub(super) blocks: Vec<InlineBlock>,
	pub(super) focus: Option<InlineFocus>,
	pub(super) mode: BlockMode,
	pub(super) content_area: Cell<Rect>,
	pub(super) impl_step: Option<BlockIdx>,
	pub(super) scroll_offset: Cell<ScrollOffset>,
	/// Last known max scroll from render pass
	pub(super) max_scroll: Cell<u16>,
	/// Parsed sections — frozen after init
	pub(super) cached_sections:
		Box<[ContentSection]>,
	/// Whether this response is implementation
	pub(super) intent: ResponseIntent,
}

impl InlineBlocks {
	pub fn activate(
		review_blocks: Vec<ReviewBlock>,
		full_response: String,
	) -> Self {
		Self::activate_with_intent(
			review_blocks,
			full_response,
			ResponseIntent::Question,
		)
	}

	pub fn activate_with_intent(
		review_blocks: Vec<ReviewBlock>,
		full_response: String,
		intent: ResponseIntent,
	) -> Self {
		let sections =
			parse_sections(&full_response);
		let blocks = review_blocks
			.into_iter()
			.map(InlineBlock::from_review)
			.collect();
		Self {
			blocks,
			focus: None,
			mode: BlockMode::Editable,
			content_area: Cell::new(Rect::default()),
			impl_step: None,
			scroll_offset: Cell::new(
				ScrollOffset::default(),
			),
			max_scroll: Cell::new(0),
			cached_sections: sections
				.into_boxed_slice(),
			intent,
		}
	}

	pub fn freeze(&mut self) {
		self.mode = BlockMode::Frozen;
		self.focus = None;
		self.impl_step = None;
		for block in &mut self.blocks {
			block.sync_edited_from_editor();
			block.editor = TextArea::default();
		}
	}
}

impl InlineBlock {
	fn from_review(rb: ReviewBlock) -> Self {
		let lines: Vec<String> = rb
			.edited
			.lines()
			.map(String::from)
			.collect();
		Self {
			index: rb.index,
			lang: rb.lang,
			original: rb.original,
			edited: rb.edited,
			file_path: rb.file_path,
			agent_source: rb.agent_source,
			editor: TextArea::new(lines),
			question: String::new(),
			answer: None,
			render_rect: Cell::new(Rect::default()),
			highlight_cache: RefCell::new(None),
		}
	}

	pub fn sync_edited_from_editor(&mut self) {
		self.edited =
			self.editor.lines().join("\n");
		*self.highlight_cache.borrow_mut() = None;
	}

	pub fn rendered_code_lines(
		&self,
	) -> Rc<[Line<'static>]> {
		if let Some(ref c) =
			*self.highlight_cache.borrow()
		{
			return Rc::clone(c);
		}
		let src: Vec<&str> =
			self.edited.split('\n').collect();
		let rc: Rc<[Line<'static>]> =
			code_block::render_lines(
				&self.lang, &src,
			)
			.into();
		*self.highlight_cache.borrow_mut() =
			Some(Rc::clone(&rc));
		rc
	}
}

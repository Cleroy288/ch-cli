use crate::app::App;
use crate::ui::wrap::pre_wrap;

use crate::ui::components::debug_render::{
    build_claude_response_lines,
    build_message_history_lines,
};

const SCROLL_STEP: u16 = 3;

impl App {
    pub(crate) fn scroll_up(&mut self) {
        self.scroll_offset =
            self.scroll_offset
                .saturating_sub(SCROLL_STEP);
    }

    pub(crate) fn scroll_down(&mut self) {
        self.scroll_offset =
            self.scroll_offset
                .saturating_add(SCROLL_STEP);
        self.clamp_debug_scroll();
    }

    /// Prevent scroll_offset from growing unbounded.
    fn clamp_debug_scroll(&mut self) {
        let area = self.output_area.get();
        if area.height == 0 { return; }
        let h = build_message_history_lines(
            self.history(),
        );
        let r = build_claude_response_lines(
            self.last_claude_response(),
        );
        let total = pre_wrap(h, area.width).len()
            + pre_wrap(r, area.width).len();
        let max = total.saturating_sub(
            area.height as usize,
        );
        if self.scroll_offset as usize > max {
            self.scroll_offset = max as u16;
        }
    }

    /// Routes to inline panel when active,
    /// else output panel.
    pub(crate) fn handle_scroll_up(&mut self) {
        if let Some(ib) = &mut self.inline_blocks {
            ib.scroll_up();
        } else {
            self.scroll_up();
        }
    }

    /// Routes to inline panel when active,
    /// else output panel.
    pub(crate) fn handle_scroll_down(
        &mut self,
    ) {
        if let Some(ib) = &mut self.inline_blocks {
            ib.scroll_down();
        } else {
            self.scroll_down();
        }
    }
}

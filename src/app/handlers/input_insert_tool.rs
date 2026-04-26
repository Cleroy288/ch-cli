use crate::app::App;
use crate::domain::tool_ref::{
    ToolItem, ToolKind, ToolReference, format_tool_ref,
};

impl App {
    pub(crate) fn insert_tool_reference(
        &mut self,
        kind: ToolKind,
        item: &ToolItem,
    ) {
        self.remove_trigger_char();
        let ref_text =
            format_tool_ref(kind, &item.display, None);
        let start = self.cursor_position.get();
        self.insert_text_at_cursor(&ref_text);
        let end = self.cursor_position.get();
        let tool_ref = ToolReference {
            start,
            end,
            kind,
            key: item.key.clone(),
            display: item.display.clone(),
        };
        self.tool_references.push(tool_ref);
        self.picker.deactivate();
    }

}

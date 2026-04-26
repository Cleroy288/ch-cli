use crate::app::App;
use crate::domain::{
    FileName, FilePath, FileReference, InputSpan,
};

impl App {
    pub(crate) fn insert_selected_path(
        &mut self,
        full_path: String,
        name_only: String,
        is_dir: bool,
    ) {
        self.remove_trigger_char();

        let start = self.cursor_position.get();
        self.insert_text_at_cursor(&name_only);
        let end = self.cursor_position.get();

        let span = InputSpan { start, end };
        let file_ref = FileReference::new(
            span,
            FilePath::from(full_path),
            FileName::from(name_only),
            is_dir,
        );
        self.file_references.push(file_ref);
    }
}

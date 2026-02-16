//! Path insertion handlers for inserting selected
//! file/folder paths into the input buffer.

use crate::app::App;
use crate::domain::{
    FileName, FilePath, FileReference, InputSpan,
};

impl App {
    /// Insert a selected file/folder path at trigger.
    pub(crate) fn insert_selected_path(
        &mut self,
        full_path_str: String,
        name_only: String,
        is_dir: bool,
    ) {
        let trigger_pos =
            self.picker.trigger_position();
        self.remove_at_trigger_char(trigger_pos);

        let start_pos = self.cursor_position.get();
        self.insert_name_text(&name_only);
        let end_pos = self.cursor_position.get();

        let span = InputSpan {
            start: start_pos,
            end: end_pos,
        };
        let info = PathInfo {
            full_path: full_path_str,
            name: name_only,
            is_dir,
        };
        let file_ref =
            build_file_ref_from_parts(span, info);
        self.file_references.push(file_ref);
    }

    /// Remove the @ trigger character
    fn remove_at_trigger_char(
        &mut self,
        trigger_pos: usize,
    ) {
        let trigger_char =
            self.input.chars().nth(trigger_pos);
        if trigger_pos < self.input.len()
            && trigger_char == Some('@')
        {
            self.input.remove(trigger_pos);
            self.cursor_position.set(trigger_pos);
        }
    }

    /// Insert the name text character by character
    fn insert_name_text(
        &mut self,
        name_only: &str,
    ) {
        for chr in name_only.chars() {
            let pos = self.cursor_position.get();
            self.input.insert(pos, chr);
            self.cursor_position.move_right(
                self.input.len(),
            );
        }
    }
}

/// Path information for building a FileReference
struct PathInfo {
    full_path: String,
    name: String,
    is_dir: bool,
}

/// Build a FileReference from parts (pure function)
fn build_file_ref_from_parts(
    span: InputSpan,
    info: PathInfo,
) -> FileReference {
    FileReference::new(
        span,
        FilePath::from(info.full_path),
        FileName::from(info.name),
        info.is_dir,
    )
}

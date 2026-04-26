use crate::domain::file_name::FileName;
use crate::domain::file_path::FilePath;

#[derive(Debug, Clone, Copy)]
pub struct InputSpan {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct FileReference {
    pub start: usize,
    pub end: usize,
    pub full_path: FilePath,
    pub display_name: FileName,
    pub is_dir: bool,
}

impl FileReference {
    pub fn new(
        span: InputSpan,
        full_path: FilePath,
        display_name: FileName,
        is_dir: bool,
    ) -> Self {
        Self {
            start: span.start,
            end: span.end,
            full_path,
            display_name,
            is_dir,
        }
    }
}

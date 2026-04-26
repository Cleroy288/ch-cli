use std::path::PathBuf;

use crate::domain::{DIR_SYMBOL, FILE_SYMBOL};

#[derive(Debug, Clone)]
pub struct FsEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
}

impl FsEntry {
    pub fn new(path: PathBuf, is_dir: bool) -> Self {
        let name = path
            .file_name()
            .and_then(|os_name| os_name.to_str())
            .unwrap_or("")
            .to_string();

        Self { path, name, is_dir }
    }

    pub fn display_name(&self) -> String {
        if self.is_dir {
            format!("{} {}", DIR_SYMBOL, self.name)
        } else {
            format!("{} {}", FILE_SYMBOL, self.name)
        }
    }

    pub fn path_string(&self) -> String {
        self.path.to_str().unwrap_or("").to_string()
    }

    pub fn name_only(&self) -> String {
        self.name.clone()
    }
}


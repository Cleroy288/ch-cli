use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

/// Max entries kept in history
const MAX_ENTRIES: usize = 200;

/// Prompt history scoped to a project.
/// Pure data -- I/O lives in service layer.
#[derive(Debug, Serialize, Deserialize)]
pub struct PromptHistory {
    entries: VecDeque<String>,
}

impl PromptHistory {
    /// Create an empty history.
    pub fn empty() -> Self {
        Self { entries: VecDeque::new() }
    }

    /// Add an entry (dedup + cap, no I/O).
    pub fn add(&mut self, text: &str) {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return;
        }
        self.dedup_last(trimmed);
        self.entries.push_back(trimmed.to_owned());
        self.enforce_cap();
    }

    /// Entry at `idx` from the end (0 = most recent).
    pub fn get_from_end(
        &self,
        idx: usize,
    ) -> Option<&str> {
        let len = self.entries.len();
        if idx >= len {
            return None;
        }
        self.entries
            .get(len - 1 - idx)
            .map(String::as_str)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl PromptHistory {
    /// Skip duplicate if last entry matches.
    fn dedup_last(&mut self, text: &str) {
        if self.entries.back().map(String::as_str)
            == Some(text)
        {
            self.entries.pop_back();
        }
    }

    fn enforce_cap(&mut self) {
        while self.entries.len() > MAX_ENTRIES {
            self.entries.pop_front();
        }
    }
}

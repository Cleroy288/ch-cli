use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileName(pub(crate) String);

impl FileName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FileName {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(&self.0)
    }
}

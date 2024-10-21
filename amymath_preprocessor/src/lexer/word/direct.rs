use crate::{processor::DefKind, to_tex::ToTex};

/// A word token whose TeX is identical to the word.
#[derive(Debug, Clone, Copy)]
pub struct DirectWordToken<'doc> {
    /// The word being represented.
    pub name: &'doc str,
    /// A slot for identifying what kind of word this token is.
    pub kind: Option<DefKind>,
}

impl<'doc> DirectWordToken<'doc> {
    /// Construct a direct word token from the word it represents.
    pub fn new(name: &'doc str) -> Self {
        Self { name, kind: None }
    }
}

impl<'doc> ToTex for DirectWordToken<'doc> {
    fn to_tex(self) -> String {
        if let Some(kind) = self.kind {
            format!("{}{{{}}}", kind.to_tex(), self.name)
        } else {
            format!("{{{}}}", self.name)
        }
    }
}

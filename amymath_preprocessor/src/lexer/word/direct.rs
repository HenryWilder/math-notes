use crate::{processor::DefKind, to_tex::ToTex};

#[derive(Debug, Clone, Copy)]
pub struct DirectWordToken<'doc> {
    pub name: &'doc str,
    pub kind: Option<DefKind>,
}

impl<'doc> DirectWordToken<'doc> {
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

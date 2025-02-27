/// Builtin word tokens.
pub mod builtin;
/// Direct word tokens.
pub mod direct;

use builtin::*;
use direct::*;

/// A token representing a variable, constant, or function.
#[derive(Debug, Clone, Copy)]
pub enum WordToken<'doc> {
    /// LaTeX is identical to the name
    Direct(DirectWordToken<'doc>),

    /// LaTeX is an associated command
    Builtin(BuiltinWordToken),
}

impl<'doc> From<&'doc str> for WordToken<'doc> {
    fn from(value: &'doc str) -> Self {
        if let Some(bw_token) = BuiltinWordToken::try_from(&value) {
            Self::Builtin(bw_token)
        } else {
            Self::Direct(DirectWordToken::new(value))
        }
    }
}

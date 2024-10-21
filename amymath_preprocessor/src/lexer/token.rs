use crate::to_tex::ToTex;

use super::operator::*;
use super::group_ctrl::*;
use super::word::*;

/// A single token from the source document.
#[derive(Clone, Copy)]
pub enum Token<'doc> {
    /// The name of a variable, constant, or function
    Word(WordToken<'doc>),

    /// A literal number (excluding mathematical constants)
    Number(&'doc str),

    /// A mathematical operator which may look at nodes to its left or right
    Operator(OperatorToken),

    /// A delimiter indicating the start or end of a subexpression
    GroupCtrl(GroupCtrlToken),
}

impl<'doc> std::fmt::Debug for Token<'doc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // force regular debug even if using "pretty" debug
        match self {
            Self::Word(arg0)
                => write!(f, "Word({arg0:?})"),
            Self::Number(arg0)
                => write!(f, "Number({arg0:?})"),
            Self::Operator(arg0)
                => write!(f, "Operator({arg0:?})"),
            Self::GroupCtrl(arg0)
                => write!(f, "GroupCtrl({arg0:?})"),
        }
    }
}

impl<'doc> ToTex for Token<'doc> {
    fn to_tex(self) -> String {
        match self {
            Self::Number(token)
                => format!(r"\lit{{{token}}}"),

            Self::Word(WordToken::Direct(dw_token))
                => dw_token.to_tex(),

            Self::Word(WordToken::Builtin(bw_token))
                => bw_token.to_tex(),

            Self::Operator(op_token)
                => op_token.to_tex(),

            Self::GroupCtrl(gc_token)
                => gc_token.to_tex(),
        }
    }
}

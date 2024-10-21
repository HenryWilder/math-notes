use std::error::Error;
use crate::lexer::operator::OperatorToken;
use super::{BracketKind, GroupCtrlToken};

/// An error that occurs while parsing.
#[derive(Debug)]
pub enum ParseError {
    /// The global scope was popped.
    TooManyCloseBrackets,
    /// The document ended with excess scopes.
    NotEnoughCloseBrackets,
    /// An operator in the source has fewer arguments than are valid for that operator.
    OperatorMissingArguments{
        /// The number of items available to the left of the operator.
        num_lhs: usize,
        /// The operator.
        op_token: OperatorToken,
        /// The number of items available to the right of the operator.
        num_rhs: usize,
    },
    /// A group was closed with a bracket that isn't compatible with the bracket it was opened with.
    BracketMismatch{
        /// The bracket that opened the group in the source document.
        opened_with: BracketKind,
        /// The bracket that closed the group in the source document.
        closed_with: BracketKind,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::TooManyCloseBrackets
                => write!(f, "More bracket/brace/parentheses groups were closed than opened"),
            ParseError::NotEnoughCloseBrackets
                => write!(f, "More bracket/brace/parentheses groups were opened than closed"),
            ParseError::OperatorMissingArguments { num_lhs, op_token, num_rhs }
                => write!(f, "No version of `{op_token:?}` operator takes {num_lhs} left-hand arguments and {num_rhs} right-hand arguments."),
            ParseError::BracketMismatch { opened_with, closed_with }
                => write!(f, "Mismatched bracket pair: \"{}\" is incompatible with \"{}\"",
                    GroupCtrlToken::open(*opened_with).source_str(),
                    GroupCtrlToken::close(*closed_with).source_str(),
                ),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

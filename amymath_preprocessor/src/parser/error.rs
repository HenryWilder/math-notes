use std::error::Error;

use crate::lexer::operator::OperatorToken;

use super::BracketKind;

#[derive(Debug)]
pub enum ParseError {
    TooManyCloseBrackets,
    NotEnoughCloseBrackets,
    OperatorMissingArguments{
        lhs_exists: bool,
        op_token: OperatorToken,
        rhs_exists: bool,
    },
    BracketMismatch{
        opened_with: BracketKind,
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
            ParseError::OperatorMissingArguments { lhs_exists, op_token, rhs_exists }
                => write!(f, "No version of `{op_token:?}` operator takes {} left-hand argument and {} right-hand argument.",
                    if *lhs_exists { "1" } else { "no" },
                    if *rhs_exists { "1" } else { "no" },
                ),
            ParseError::BracketMismatch { opened_with, closed_with }
                => write!(f, "Mismatched bracket pair: \"{}\" is incompatible with \"{}\"",
                    match opened_with {
                        BracketKind::Paren => "(",
                        BracketKind::Brack => "[",
                        BracketKind::Brace => "{",
                        BracketKind::VVert => "||(",
                        BracketKind::Vert  => "|(",
                    },
                    match closed_with {
                        BracketKind::Paren => ")",
                        BracketKind::Brack => "]",
                        BracketKind::Brace => "}",
                        BracketKind::VVert => ")||",
                        BracketKind::Vert  => ")|",
                    },
                ),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

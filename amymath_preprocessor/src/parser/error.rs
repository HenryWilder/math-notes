use std::error::Error;

use crate::lexer::operator::OperatorToken;

#[derive(Debug)]
pub enum ParseError {
    TooManyCloseBrackets,
    NotEnoughCloseBrackets,
    OperatorMissingArguments{
        is_lhs_nonnull: bool,
        op_token: OperatorToken,
        is_rhs_nonnull: bool,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::TooManyCloseBrackets
                => write!(f, "More bracket/brace/parentheses groups were closed than opened"),
            ParseError::NotEnoughCloseBrackets
                => write!(f, "More bracket/brace/parentheses groups were opened than closed"),
            ParseError::OperatorMissingArguments { is_lhs_nonnull, op_token, is_rhs_nonnull }
                => write!(f, "No version of `{op_token:?}` operator takes {} left-hand argument and {} right-hand argument.",
                    if *is_lhs_nonnull { "1" } else { "no" },
                    if *is_rhs_nonnull { "1" } else { "no" },
                ),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

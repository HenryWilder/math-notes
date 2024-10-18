use std::error::Error;

use crate::lexer::operator::OperatorToken;

#[derive(Debug)]
pub enum ParseError {
    TooManyCloseBrackets,
    NotEnoughCloseBrackets,
    OperatorMissingArguments{
        num_provided: usize,
        op_token: OperatorToken,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::TooManyCloseBrackets
                => write!(f, "More bracket/brace/parentheses groups were closed than opened"),
            ParseError::NotEnoughCloseBrackets
                => write!(f, "More bracket/brace/parentheses groups were opened than closed"),
            ParseError::OperatorMissingArguments { num_provided, op_token }
                => write!(f, "No version of {op_token:?} operator takes {num_provided} arguments."),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

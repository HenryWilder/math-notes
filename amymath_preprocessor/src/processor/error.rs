use std::error::Error;

use crate::parser::error::ParseError;
use crate::lexer::error::LexerError;

#[derive(Debug)]
pub enum AmymathError {
    LexerError{
        line_number: usize,
        error: LexerError,
    },
    ParseError{
        line_number: usize,
        error: ParseError,
    },
    InvalidHeading{
        line_number: usize,
    },
    TemplateMissingContent,
}

impl std::fmt::Display for AmymathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AmymathError::LexerError { line_number, error }
                => write!(f, "At line {line_number}: Tokenization error: {error}"),
            AmymathError::ParseError { line_number, error }
                => write!(f, "At line {line_number}: Parse error: {error}"),
            AmymathError::InvalidHeading { line_number }
                => write!(f, "At line {line_number}: Headings must start with 1-4 '#'s followed by a space and then text."),
            AmymathError::TemplateMissingContent
                => write!(f, "Template is missing a content anchor, I don't know where the content should be inserted."),
        }
    }
}

impl Error for AmymathError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AmymathError::LexerError { line_number: _, error } => Some(error),
            AmymathError::ParseError { line_number: _, error } => Some(error),
            _ => None,
        }
    }
}

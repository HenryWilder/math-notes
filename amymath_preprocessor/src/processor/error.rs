use std::error::Error;

use crate::parser::error::ParseError;
use crate::lexer::error::LexerError;

#[derive(Debug)]
pub enum LineErrorKind {
    LexerError(LexerError),
    ParseError(ParseError),
    InvalidMetaItem,
    InvalidHeading,
}

#[derive(Debug)]
pub struct LineError {
    pub line_number: usize,
    pub kind: LineErrorKind,
}

impl std::fmt::Display for LineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "At line {}: {}",
            self.line_number,
            match &self.kind {
                LineErrorKind::LexerError(error)
                    => error.to_string(),
                LineErrorKind::ParseError(error)
                    => error.to_string(),
                LineErrorKind::InvalidMetaItem
                    => "Meta items must start with '@' followed by the meta item key and then the value.".to_string(),
                LineErrorKind::InvalidHeading
                    => "Headings must start with 1-4 '#'s followed by a space and then text.".to_string(),
            }
        )
    }
}

impl Error for LineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            LineErrorKind::LexerError(error) => Some(error),
            LineErrorKind::ParseError(error) => Some(error),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum PreprocError {
    LineError(LineError),
    TemplateMissingContent,
}

impl PreprocError {
    pub fn line_error(line_number: usize, kind: LineErrorKind) -> Self {
        Self::LineError(LineError { line_number, kind })
    }

    pub fn lexer_error(line_number: usize, error: LexerError) -> Self {
        Self::LineError(LineError { line_number, kind: LineErrorKind::LexerError(error) })
    }

    pub fn parse_error(line_number: usize, error: ParseError) -> Self {
        Self::LineError(LineError { line_number, kind: LineErrorKind::ParseError(error) })
    }
}

impl std::fmt::Display for PreprocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Preprocessor Error: {}", 
            match self {
                PreprocError::LineError(error)
                    => error.to_string(),
                PreprocError::TemplateMissingContent
                    => "Template is missing a content anchor, I don't know where the content should be inserted.".to_string(),
            }
        )
    }
}

impl Error for PreprocError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PreprocError::LineError(error) => Some(error),
            _ => None,
        }
    }
}

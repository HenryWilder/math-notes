use std::error::Error;

use crate::parser::error::ParseError;
use crate::lexer::error::LexerError;

/// A preprocessor error that can be narrowed down to a particular line.
#[derive(Debug)]
pub enum LineErrorKind {
    /// The error occurred in the tokenizer.
    LexerError(LexerError),
    /// The error occurred in the parser.
    ParseError(ParseError),
    /// A meta item was detected but malformed.
    InvalidMetaItem,
    /// A heading was detected but malformed.
    InvalidHeading,
}

/// A [`LineErrorKind`] with line number.
#[derive(Debug)]
pub struct LineError {
    /// The line the error occurred on.
    pub line_number: usize,
    /// The specific error.
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

/// An error that occurs during preprocessing.
#[derive(Debug)]
pub enum PreprocError {
    /// An error on a particular line.
    LineError(LineError),
    /// The template document has no content anchor
    TemplateMissingContent,
}

impl PreprocError {
    /// Shorthand for constructing a line error.
    pub fn line_error(line_number: usize, kind: LineErrorKind) -> Self {
        Self::LineError(LineError { line_number, kind })
    }

    /// Shorthand for constructing a lexer line error.
    pub fn lexer_error(line_number: usize, error: LexerError) -> Self {
        Self::LineError(LineError { line_number, kind: LineErrorKind::LexerError(error) })
    }

    /// Shorthand for constructing a parser line error.
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

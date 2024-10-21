use std::error::Error;

/// An error that occurs while tokenizing a document.
#[derive(Debug)]
pub enum LexerError {
    /// A token that cannot be categoized was found.
    UnknownToken{
        /// The token from the source document.
        token: String,
    },
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnknownToken{ token }
                => write!(f, "Unrecognized token: `{token}`"),
        }
    }
}

impl Error for LexerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

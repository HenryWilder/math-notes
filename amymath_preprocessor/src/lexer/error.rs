use std::error::Error;

#[derive(Debug)]
pub enum LexerError {
    UnknownToken{ token: String },
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

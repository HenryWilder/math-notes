use std::error::Error;

#[derive(Debug)]
pub enum ArgsError {
    IOError(std::io::Error),
    MissingSource,
    MissingKVPValue { key: &'static str },
    UnknownArg(String),
}
 
impl std::fmt::Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgsError::IOError(e) => e.fmt(f),
            ArgsError::MissingSource => write!(f, "Missing argument for source file"),
            ArgsError::MissingKVPValue { key } => write!(f, "Missing value for {key} argument"),
            ArgsError::UnknownArg(arg) => write!(f, "Unrecognized argument: \"{arg}\""),
        }
    }
}

impl From<std::io::Error> for ArgsError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl Error for ArgsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ArgsError::IOError(error) => Some(error),
            _ => None,
        }
    }
}

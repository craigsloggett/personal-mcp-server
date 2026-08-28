use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TransportError {
    InvalidMessage(String),

    Stdio(StdioError),

    Io(std::io::Error),
}

#[derive(Debug)]
pub enum StdioError {
    EmptyMessage,
    ContainsNewline,
    Io(std::io::Error),
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransportError::InvalidMessage(msg) => {
                write!(f, "invalid message: {}", msg)
            }
            TransportError::Stdio(e) => {
                write!(f, "stdio error: {}", e)
            }
            TransportError::Io(e) => {
                write!(f, "io error: {}", e)
            }
        }
    }
}

impl fmt::Display for StdioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StdioError::EmptyMessage => {
                write!(f, "message cannot be empty")
            }
            StdioError::ContainsNewline => {
                write!(f, "message contains newline character")
            }
            StdioError::Io(e) => {
                write!(f, "io error: {}", e)
            }
        }
    }
}

impl Error for TransportError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TransportError::Stdio(e) => Some(e),
            TransportError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl Error for StdioError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            StdioError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<StdioError> for TransportError {
    fn from(error: StdioError) -> Self {
        TransportError::Stdio(error)
    }
}

impl From<std::io::Error> for TransportError {
    fn from(error: std::io::Error) -> Self {
        TransportError::Io(error)
    }
}

impl From<std::io::Error> for StdioError {
    fn from(error: std::io::Error) -> Self {
        StdioError::Io(error)
    }
}

pub type TransportResult<T> = std::result::Result<T, TransportError>;
pub type StdioResult<T> = std::result::Result<T, StdioError>;

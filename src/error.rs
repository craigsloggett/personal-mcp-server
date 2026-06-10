use std::error::Error;
use std::fmt;

pub use crate::transport::TransportError;

#[derive(Debug)]
pub enum ServerError {
    Transport(TransportError),

    Io(std::io::Error),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::Transport(e) => {
                write!(f, "transport error: {}", e)
            }
            ServerError::Io(e) => {
                write!(f, "io error: {}", e)
            }
        }
    }
}

impl Error for ServerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ServerError::Transport(e) => Some(e),
            ServerError::Io(e) => Some(e),
        }
    }
}

impl From<TransportError> for ServerError {
    fn from(error: TransportError) -> Self {
        ServerError::Transport(error)
    }
}

impl From<std::io::Error> for ServerError {
    fn from(error: std::io::Error) -> Self {
        ServerError::Io(error)
    }
}

pub type ServerResult<T> = std::result::Result<T, ServerError>;

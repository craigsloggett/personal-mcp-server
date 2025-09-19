use std::error::Error;
use std::fmt;

pub use crate::transport::TransportError;

#[derive(Debug)]
pub enum ServerError {
    InvalidConfiguration(String),

    Transport(TransportError),
    Config(ConfigError),

    Io(std::io::Error),
}

#[derive(Debug)]
pub enum ConfigError {
    MissingField(String),
    InvalidValue(String),
    Io(std::io::Error),
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::InvalidConfiguration(msg) => {
                write!(f, "invalid configuration: {}", msg)
            }
            ServerError::Transport(e) => {
                write!(f, "transport error: {}", e)
            }
            ServerError::Config(e) => {
                write!(f, "configuration error: {}", e)
            }
            ServerError::Io(e) => {
                write!(f, "io error: {}", e)
            }
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingField(field) => {
                write!(f, "missing required field: {}", field)
            }
            ConfigError::InvalidValue(msg) => {
                write!(f, "invalid configuration value: {}", msg)
            }
            ConfigError::Io(e) => {
                write!(f, "io error: {}", e)
            }
        }
    }
}

impl Error for ServerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ServerError::Transport(e) => Some(e),
            ServerError::Config(e) => Some(e),
            ServerError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<TransportError> for ServerError {
    fn from(error: TransportError) -> Self {
        ServerError::Transport(error)
    }
}

impl From<ConfigError> for ServerError {
    fn from(error: ConfigError) -> Self {
        ServerError::Config(error)
    }
}

impl From<std::io::Error> for ServerError {
    fn from(error: std::io::Error) -> Self {
        ServerError::Io(error)
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError::Io(error)
    }
}

pub type ServerResult<T> = std::result::Result<T, ServerError>;
pub type ConfigResult<T> = std::result::Result<T, ConfigError>;

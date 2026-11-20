use thiserror::Error;

/// Errors produced while initializing structured logging.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum LoggingError {
    /// The configured log level cannot be parsed.
    #[error("invalid log level: {0}")]
    InvalidLevel(String),
    /// The log directory or file cannot be created.
    #[error("log file operation failed: {0}")]
    Io(String),
    /// The tracing subscriber could not be built / installed globally.
    #[error("cannot initialise tracing subscriber: {0}")]
    Subscriber(String),
}

impl From<std::io::Error> for LoggingError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

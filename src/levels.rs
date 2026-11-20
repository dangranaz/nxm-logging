use crate::error::LoggingError;

/// Build profile used to select the default log level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogEnvironment {
    /// Development: granular logs.
    Development,
    /// Release: errors only.
    Release,
}

impl LogEnvironment {
    /// Detects the environment from Cargo's debug assertions.
    pub fn current() -> Self {
        if cfg!(debug_assertions) {
            Self::Development
        } else {
            Self::Release
        }
    }
}

/// Returns the default log directive for an environment.
pub fn default_level(environment: LogEnvironment) -> &'static str {
    match environment {
        LogEnvironment::Development => "debug",
        LogEnvironment::Release => "error",
    }
}

/// Validates a log directive.
pub fn validate_level(level: &str) -> Result<(), LoggingError> {
    let directive = level
        .split(',')
        .next()
        .unwrap_or_default()
        .split('=')
        .next_back()
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();
    match directive.as_str() {
        "error" | "warn" | "info" | "debug" | "trace" | "off" => Ok(()),
        other => Err(LoggingError::InvalidLevel(other.to_string())),
    }
}

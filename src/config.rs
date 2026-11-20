use crate::levels::{default_level, validate_level, LogEnvironment};
use std::env;
use std::path::PathBuf;

/// Configuration for the shared logging subscriber.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoggingConfig {
    pub environment: LogEnvironment,
    pub level: Option<String>,
    pub directory: PathBuf,
    pub file_name: String,
}

impl LoggingConfig {
    /// Builds configuration from a directory and optional `NXM_LOG_LEVEL` override.
    /// Invalid overrides are retained but ignored by `effective_level()`.
    pub fn from_directory(directory: impl Into<PathBuf>) -> Self {
        let level = env::var("NXM_LOG_LEVEL")
            .ok()
            .filter(|value| !value.is_empty());
        Self {
            environment: LogEnvironment::current(),
            level,
            directory: directory.into(),
            file_name: "nxm.log".to_string(),
        }
    }

    /// Returns the effective directive, falling back to the environment default.
    pub fn effective_level(&self) -> String {
        match &self.level {
            Some(level) if validate_level(level).is_ok() => level.clone(),
            _ => default_level(self.environment).to_string(),
        }
    }
}

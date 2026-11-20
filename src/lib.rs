//! Shared structured logging for the NXM platform.
//!
//! Provides two initialisation styles:
//! - [`init_logging`] — `LoggingConfig` driven, returns a `LoggingGuard` (used by `nxm-core`).
//! - [`init`] / [`init_from_env`] / [`init_product_logging`] — MCP-server style, "first wins",
//!   always JSON, optional file + stderr rotation (merged from `nxm-mcp-logs`).
//!
//! # Examples
//!
//! ```
//! use nxm_logging::{init_logging, LogEnvironment, LoggingConfig};
//! let config = LoggingConfig {
//!     environment: LogEnvironment::Development,
//!     level: None,
//!     directory: std::env::temp_dir(),
//!     file_name: "nxm-example.log".to_string(),
//! };
//! let _guard = init_logging(config).expect("logging should initialize");
//! ```

mod config;
mod error;
mod filter;
mod init;
mod levels;

pub use config::LoggingConfig;
pub use error::LoggingError;
pub use filter::LogFilter;
pub use init::{
    init, init_from_env, init_logging, init_product_logging, LogConfig, LoggingGuard, Rotation,
};
pub use levels::{default_level, validate_level, LogEnvironment};

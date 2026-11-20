//! Logging initialisation.
//!
//! Two styles coexist:
//! - [`init_logging`] — `LoggingConfig` driven, returns a `LoggingGuard`.
//! - [`init`] / [`init_from_env`] / [`init_product_logging`] — MCP-server style,
//!   "first wins", always JSON, optional file + stderr rotation.

use crate::config::LoggingConfig;
use crate::error::LoggingError;
use crate::filter::LogFilter;
use std::fs;
use std::io;
use std::path::PathBuf;
use tracing::subscriber::set_global_default;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{daily, RollingFileAppender};
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;

/// Guard that keeps the non-blocking writer alive until dropped.
pub struct LoggingGuard {
    _worker: WorkerGuard,
}

impl LoggingGuard {
    pub(crate) fn new(worker: WorkerGuard) -> Self {
        Self { _worker: worker }
    }
}

/// Initializes the global `tracing` subscriber with a rotating file appender.
///
/// This is the original `nxm-logging` API used by `nxm-core`.
pub fn init_logging(config: LoggingConfig) -> Result<LoggingGuard, LoggingError> {
    fs::create_dir_all(&config.directory)?;
    let appender = daily(&config.directory, &config.file_name);
    let (writer, worker) = tracing_appender::non_blocking(appender);
    let filter = EnvFilter::new(config.effective_level());
    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(layer().with_writer(writer).with_ansi(false));
    set_global_default(subscriber).map_err(|error| LoggingError::Io(error.to_string()))?;
    Ok(LoggingGuard::new(worker))
}

/// Logging configuration for the MCP-server style initialisation.
///
/// The format is always JSON (one log line per event, machine-parseable) —
/// this is the observability contract for all NXM MCP servers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogConfig {
    /// The tracing filter, e.g. `"nxm_mcp_tracker=debug"`.
    pub filter: LogFilter,
    /// Optional file path for log rotation. If set, logs go to file + stderr.
    /// If `None`, logs go only to stderr.
    pub file_path: Option<PathBuf>,
    /// Rotation policy for log files (default: daily).
    pub rotation: Rotation,
    /// Maximum number of rotated files to keep (default: 7).
    pub max_files: usize,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            filter: LogFilter::default(),
            file_path: None,
            rotation: Rotation::Daily,
            max_files: 7,
        }
    }
}

/// Log rotation policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    /// Rotate every minute (mostly for testing).
    Minutely,
    /// Rotate every hour.
    Hourly,
    /// Rotate daily at midnight (default).
    Daily,
    /// Never rotate (single file).
    Never,
}

impl Rotation {
    fn to_tracing_appender(&self) -> tracing_appender::rolling::Rotation {
        match self {
            Rotation::Minutely => tracing_appender::rolling::Rotation::MINUTELY,
            Rotation::Hourly => tracing_appender::rolling::Rotation::HOURLY,
            Rotation::Daily => tracing_appender::rolling::Rotation::DAILY,
            Rotation::Never => tracing_appender::rolling::Rotation::NEVER,
        }
    }
}

/// Global guard to keep the non-blocking writer alive.
static GUARD: std::sync::OnceLock<WorkerGuard> = std::sync::OnceLock::new();

/// A writer that writes to both a non-blocking file appender and stderr.
struct CombinedWriter {
    file: tracing_appender::non_blocking::NonBlocking,
    stderr: io::Stderr,
}

impl io::Write for CombinedWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let _ = self.file.write(buf);
        self.stderr.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        let _ = self.file.flush();
        self.stderr.flush()
    }
}

/// Install the global tracing subscriber with the given config.
///
/// Returns `Err` if the subscriber cannot be installed (invalid filter, or a
/// subscriber is already globally set). Safe to call multiple times — only the
/// first succeeds ("first wins").
pub fn init(config: LogConfig) -> Result<(), LoggingError> {
    let filter_str = config.filter.as_ref().to_string();

    let _ = EnvFilter::try_new(&filter_str)
        .map_err(|e| LoggingError::Subscriber(format!("invalid log filter '{filter_str}': {e}")))?;

    let make_writer: Box<dyn Fn() -> Box<dyn io::Write + Send + Sync> + Send + Sync> =
        if let Some(file_path) = config.file_path {
            let file_appender = RollingFileAppender::builder()
                .rotation(config.rotation.to_tracing_appender())
                .max_log_files(config.max_files)
                .build(file_path)
                .map_err(|e| {
                    LoggingError::Subscriber(format!("failed to create log file appender: {e}"))
                })?;

            let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
            // Store guard globally to keep it alive
            let _ = GUARD.set(guard);

            Box::new(move || {
                let writer = CombinedWriter {
                    file: non_blocking.clone(),
                    stderr: io::stderr(),
                };
                Box::new(writer) as Box<dyn io::Write + Send + Sync>
            })
        } else {
            Box::new(|| Box::new(io::stderr()) as Box<dyn io::Write + Send + Sync>)
        };

    let env_filter = EnvFilter::try_new(&config.filter.as_ref().to_string())
        .map_err(|e| LoggingError::Subscriber(format!("invalid log filter: {e}")))?;

    let subscriber = tracing_subscriber::fmt()
        .json()
        .with_writer(make_writer)
        .with_env_filter(env_filter)
        .finish();

    set_global_default(subscriber).map_err(|e| LoggingError::Subscriber(e.to_string()))
}

/// Resolve the config from the process env and initialise logging.
///
/// This is the entry point every server should use in `main`:
///
/// ```no_run
/// use nxm_logging::init_from_env;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     init_from_env()?;
///     // ... server work ...
///     Ok(())
/// }
pub fn init_from_env() -> Result<(), LoggingError> {
    init(LogConfig {
        filter: LogFilter::from_env(),
        file_path: None,
        rotation: Rotation::Daily,
        max_files: 7,
    })
}

/// Convenience function for product servers: logs to file + stderr with rotation.
///
/// The log file is placed in the `logs/` subdirectory of the standard config
/// directory: `~/.nxmconfig/logs/<server_name>.log`
///
/// ```no_run
/// use nxm_logging::init_product_logging;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     init_product_logging("my_server")?;
///     // ... server work ...
///     Ok(())
/// }
pub fn init_product_logging(server_name: &str) -> Result<(), LoggingError> {
    let config_dir = dirs::home_dir()
        .ok_or_else(|| LoggingError::Subscriber("home directory not found".into()))?
        .join(".nxmconfig");

    let logs_dir = config_dir.join("logs");
    fs::create_dir_all(&logs_dir)
        .map_err(|e| LoggingError::Subscriber(format!("failed to create logs dir: {e}")))?;

    let file_path = logs_dir.join(format!("{server_name}.log"));

    init(LogConfig {
        filter: LogFilter::from_env(),
        file_path: Some(file_path),
        rotation: Rotation::Daily,
        max_files: 7,
    })
}

// Re-export resolve_filter for convenience.
#[allow(unused_imports)]
pub use crate::filter::resolve_filter;

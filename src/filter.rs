//! Log filter resolution from environment variables.
//!
//! Contract: `NXM_MCP_LOG` wins, then `RUST_LOG`, then the built-in `"info"`.

/// Resolve the log filter string from the process environment.
///
/// Precedence (highest to lowest):
/// 1. `NXM_MCP_LOG`
/// 2. `RUST_LOG`
/// 3. `"info"` built-in default
pub fn resolve_filter() -> String {
    std::env::var("NXM_MCP_LOG")
        .or_else(|_| std::env::var("RUST_LOG"))
        .unwrap_or_else(|_| "info".to_string())
}

/// A resolved log filter, ready to hand to [`tracing_subscriber::EnvFilter`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogFilter(pub String);

impl LogFilter {
    /// Build from the process env (see [`resolve_filter`]).
    pub fn from_env() -> Self {
        Self(resolve_filter())
    }
}

impl Default for LogFilter {
    fn default() -> Self {
        Self("info".to_string())
    }
}

impl AsRef<str> for LogFilter {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

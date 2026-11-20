<div align="center">

# nxm-logging

<!-- Licensed MIT — part of the nxm tooling. -->
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-000000?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Tracing](https://img.shields.io/badge/tracing-0.3-000000?logo=rust&logoColor=white)](https://docs.rs/tracing)
[![Version](https://img.shields.io/badge/version-0.2.0-blue)](CHANGELOG.md)

**Shared structured logging for the NXM platform**

<br>

<picture>
  <source srcset="docs/assets/nexum-banner.webp" type="image/webp">
  <img src="docs/assets/nexum-banner.png" alt="Nexum-AI banner" width="1000" height="553">
</picture>

<br>

`nxm-logging` is a Rust library providing shared structured logging for the NXM platform: environment-aware log levels, rotating file output, JSON formatting, and filter resolution from environment variables.

- **Repo:** shared structured logging library for the NXM platform
- **Format:** JSON via `tracing-subscriber`
- **Environment overrides:** `NXM_LOG_LEVEL`, `NXM_MCP_LOG`, `RUST_LOG`
- **Default level:** `debug` in development, `error` in release

</div>

## What it does

| Feature | Description |
|---------|-------------|
| Environment detection | Detects `Development` or `Release` from Cargo debug assertions. |
| Default log level | `debug` in development, `error` in release. |
| Configuration | `LoggingConfig::from_directory()` with optional `NXM_LOG_LEVEL` override. |
| Initialisation | `init_logging()` returns a `LoggingGuard` and sets up rotating file output in `logs/`. |
| MCP-style init | `init()`, `init_from_env()`, `init_product_logging()` — JSON always, first-wins semantics. |
| Log filter | `LogFilter` resolves from `NXM_MCP_LOG`, then `RUST_LOG`, then `"info"`. |
| Error handling | `LoggingError` for invalid levels, I/O failures, and subscriber errors. |

## Documentation

No external documentation pages yet. See the source code and doc comments in `src/` for API details.

## Quick example

```rust
use nxm_logging::{init_logging, LogEnvironment, LoggingConfig};

let config = LoggingConfig {
    environment: LogEnvironment::Development,
    level: None,
    directory: std::env::temp_dir(),
    file_name: "nxm-example.log".to_string(),
};
let _guard = init_logging(config).expect("logging should initialize");
```

Environment variable overrides:

```bash
export NXM_LOG_LEVEL=debug
export NXM_MCP_LOG=nxm_server=trace
export RUST_LOG=info
```

## Installation

```toml
# Cargo.toml
[dependencies]
nxm-logging = { path = "../shared/nxm-logging" }
```

Or from crates.io when published:

```toml
nxm-logging = "0.2"
```

## Project rules

- Every Rust source file below 300 lines.
- Tests in `tests/`, never inline with production code.
- `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-targets`.
- Conventional Commits; update `CHANGELOG.md` and `CONTEXT.md` for significant changes.

## Tests

```bash
cargo test --all-targets
```

## Contributing

Contributions are welcome. Small, focused PRs (one logical change each) are easier to review. Use Conventional Commits and respect the Rust quality gates.

## License

Open source, **MIT** license ([LICENSE](LICENSE)).

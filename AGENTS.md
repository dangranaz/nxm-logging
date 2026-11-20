# AGENTS.md — nxm-logging

Shared structured logging library for the NXM platform.

## Rules

- Follow the Rust rules in `../../docs/configRust/rules/`.
- Keep every production file below 300 lines.
- Keep tests in `tests/`, never inline with production code.
- Run `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings` and `cargo test --all-targets` before committing.
- Use Conventional Commits and update `CHANGELOG.md` and `CONTEXT.md` for meaningful changes.

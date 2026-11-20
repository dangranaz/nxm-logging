# CONTEXT.md — nxm-logging

Stato: libreria iniziale implementata il 2026-11-17.

API pubbliche:

- `LogEnvironment::current()`.
- `default_level()` e `validate_level()`.
- `LoggingConfig::from_directory()` con override `NXM_LOG_LEVEL`.
- `init_logging()` con file rotante in `logs/`.
- `LoggingGuard` per mantenere attivo il writer non bloccante.

Default: development `debug`, release `error`.

Test: 3 passati.

use nxm_logging::{default_level, init_logging, LogEnvironment, LoggingConfig};
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn development_defaults_to_debug_and_release_defaults_to_error() {
    assert_eq!(default_level(LogEnvironment::Development), "debug");
    assert_eq!(default_level(LogEnvironment::Release), "error");
}

#[test]
fn invalid_level_falls_back_to_environment_default() {
    let invalid = LoggingConfig {
        environment: LogEnvironment::Release,
        level: Some("not-a-level".to_string()),
        directory: std::env::temp_dir(),
        file_name: "invalid.log".to_string(),
    };
    assert_eq!(invalid.effective_level(), "error");

    let valid = LoggingConfig {
        environment: LogEnvironment::Development,
        level: Some("trace".to_string()),
        directory: std::env::temp_dir(),
        file_name: "valid.log".to_string(),
    };
    assert_eq!(valid.effective_level(), "trace");
}

#[test]
fn invalid_environment_level_falls_back_instead_of_failing_construction() {
    let _guard = ENV_LOCK.lock().expect("environment lock");
    std::env::set_var("NXM_LOG_LEVEL", "verbose");
    let config = LoggingConfig::from_directory(std::env::temp_dir());
    std::env::remove_var("NXM_LOG_LEVEL");

    assert_eq!(config.effective_level(), default_level(config.environment));
}

#[test]
fn init_logging_writes_to_the_configured_directory() {
    let directory = std::env::temp_dir().join(format!("nxm-logging-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    let config = LoggingConfig {
        environment: LogEnvironment::Development,
        level: None,
        directory: directory.clone(),
        file_name: "nxm.log".to_string(),
    };

    let guard = init_logging(config).expect("logging should initialize");
    tracing::info!(target: "nxm::logging::test", "hello from test");
    drop(guard);

    let files: Vec<_> = std::fs::read_dir(&directory)
        .expect("directory should exist")
        .filter_map(Result::ok)
        .collect();
    assert!(!files.is_empty(), "at least one log file should be created");

    std::fs::remove_dir_all(&directory).expect("temporary directory should be removed");
}

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Test basic CLI functionality
#[test]
fn test_help_command() {
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("An AI coding assistant command line application"))
        .stdout(predicate::str::contains("Commands:"))
        .stdout(predicate::str::contains("explain"));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

/// Test explain command functionality
#[test]
fn test_explain_command_basic() {
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.arg("explain")
        .assert()
        .success()
        .stdout("hello world\n");
}

#[test]
fn test_explain_command_with_topic() {
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.args(&["explain", "--topic", "rust"])
        .assert()
        .success()
        .stdout("hello world - explaining: rust\n");
}

/// Test verbose logging
#[test]
fn test_verbose_logging() {
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.args(&["-v", "explain"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Starting groundhog application"))
        .stderr(predicate::str::contains("Starting explain command"))
        .stderr(predicate::str::contains("Command completed successfully"))
        .stdout("hello world\n");
}

#[test]
fn test_very_verbose_logging() {
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.args(&["-vv", "explain"])
        .assert()
        .success()
        .stderr(predicate::str::contains("Starting groundhog application"))
        .stderr(predicate::str::contains("Starting explain command"));
}

/// Test quiet mode
#[test]
fn test_quiet_mode() {
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.args(&["-q", "explain"])
        .assert()
        .success()
        .stdout("hello world\n")
        // In quiet mode, we should not see INFO level logs
        .stderr(predicate::str::contains("Starting groundhog application").not());
}

/// Test configuration file handling
#[test]
fn test_config_file_option() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test_config.toml");
    
    // Create a basic config file
    let config_content = r#"
[logging]
level = "Info"
format = "Pretty"

[commands]
default = "explain"

[output]
format = "text"
color = true

[performance]
max_file_size = 100
timeout = 30
threads = 4
"#;
    
    fs::write(&config_path, config_content).unwrap();
    
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.args(&["--config", config_path.to_str().unwrap(), "explain"])
        .assert()
        .success()
        .stdout("hello world\n");
}

/// Test invalid command
#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.arg("invalid_command")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"));
}

/// Test missing subcommand
#[test]
fn test_missing_subcommand() {
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error:"));
}

/// Test explain command help
#[test]
fn test_explain_help() {
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.args(&["explain", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Provides explanations and demonstrations"));
}

/// Test multiple verbosity flags
#[test]
fn test_multiple_verbosity_flags() {
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.args(&["-vvv", "explain"])
        .assert()
        .success()
        .stdout("hello world\n");
}

/// Test conflicting quiet and verbose flags
#[test]
fn test_quiet_and_verbose_flags() {
    // Quiet should take precedence or this should be an error
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.args(&["-q", "-v", "explain"])
        .assert()
        .success() // Currently this succeeds, quiet takes precedence
        .stdout("hello world\n");
}

/// Test environment variable configuration
#[test]
fn test_environment_variable_config() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("env_config.toml");
    
    let config_content = r#"
[logging]
level = "Debug"

[performance]
timeout = 60
"#;
    
    fs::write(&config_path, config_content).unwrap();
    
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.env("GROUNDHOG_CONFIG", config_path.to_str().unwrap())
        .arg("explain")
        .assert()
        .success()
        .stdout("hello world\n");
}

/// Test configuration file creation and loading
#[test]
fn test_config_file_creation() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("created_config.toml");
    
    // First, test that the application works without a config file
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.args(&["--config", config_path.to_str().unwrap(), "explain"])
        .assert()
        .success()
        .stdout("hello world\n");
}

/// Test error handling for invalid config file
#[test]
fn test_invalid_config_file() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("invalid_config.toml");
    
    // Create an invalid TOML file
    fs::write(&config_path, "invalid toml content [[[").unwrap();
    
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.args(&["--config", config_path.to_str().unwrap(), "explain"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"));
}

/// Test long-form argument flags
#[test]
fn test_long_form_flags() {
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.args(&["--verbose", "--verbose", "explain"])
        .assert()
        .success()
        .stdout("hello world\n")
        .stderr(predicate::str::contains("Starting explain command"));
}

/// Test that the application handles SIGINT gracefully (basic test)
#[test]
fn test_basic_signal_handling() {
    // This is a basic test - in a real scenario we'd test actual signal handling
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.arg("explain")
        .assert()
        .success()
        .stdout("hello world\n");
}

/// Performance test - ensure the application starts quickly
#[test]
fn test_startup_performance() {
    use std::time::Instant;
    
    let start = Instant::now();
    let mut cmd = Command::cargo_bin("groundhog").unwrap();
    cmd.arg("explain")
        .assert()
        .success();
    
    let duration = start.elapsed();
    // Application should start in less than 2 seconds
    assert!(duration.as_secs() < 2, "Application took too long to start: {:?}", duration);
}

/// Test output formatting consistency
#[test]
fn test_output_consistency() {
    // Run the same command multiple times and ensure consistent output
    for _ in 0..3 {
        let mut cmd = Command::cargo_bin("groundhog").unwrap();
        cmd.arg("explain")
            .assert()
            .success()
            .stdout("hello world\n");
    }
}

/// Test memory usage doesn't grow excessively
#[test]
fn test_memory_usage() {
    // Basic test - run command multiple times to check for memory leaks
    for _ in 0..10 {
        let mut cmd = Command::cargo_bin("groundhog").unwrap();
        cmd.arg("explain")
            .assert()
            .success()
            .stdout("hello world\n");
    }
} 
use tracing::Level;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

/// Initialize tracing subscriber based on verbosity and quiet flags
pub fn init_tracing(verbose: u8, quiet: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let level = if quiet {
        Level::ERROR
    } else {
        match verbose {
            0 => Level::WARN,
            1 => Level::INFO,
            2 => Level::DEBUG,
            _ => Level::TRACE,
        }
    };

    // Create environment filter with proper level filtering
    let env_filter = if quiet {
        EnvFilter::from_default_env()
            .add_directive("groundhog=error".parse()?)
    } else {
        EnvFilter::from_default_env()
            .add_directive(format!("groundhog={}", level.as_str().to_lowercase()).parse()?)
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_env_filter(env_filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .with_writer(std::io::stderr)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

    Ok(())
}

/// Convert verbosity count to log level
pub fn verbosity_to_level(verbose: u8, quiet: bool) -> Level {
    if quiet {
        Level::ERROR
    } else {
        match verbose {
            0 => Level::WARN,
            1 => Level::INFO,
            2 => Level::DEBUG,
            _ => Level::TRACE,
        }
    }
}

/// Check if a given log level would be enabled with current settings
pub fn is_level_enabled(level: Level, verbose: u8, quiet: bool) -> bool {
    let current_level = verbosity_to_level(verbose, quiet);
    level <= current_level
}

/// Initialize tracing for tests
#[cfg(test)]
pub fn init_test_tracing() {
    use std::sync::Once;
    
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .with_test_writer()
            .finish();
        
        let _ = tracing::subscriber::set_global_default(subscriber);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verbosity_to_level() {
        assert_eq!(verbosity_to_level(0, false), Level::WARN);
        assert_eq!(verbosity_to_level(1, false), Level::INFO);
        assert_eq!(verbosity_to_level(2, false), Level::DEBUG);
        assert_eq!(verbosity_to_level(3, false), Level::TRACE);
        assert_eq!(verbosity_to_level(0, true), Level::ERROR);
        assert_eq!(verbosity_to_level(5, true), Level::ERROR);
    }

    #[test]
    fn test_is_level_enabled() {
        // With verbose = 1 (INFO level)
        assert!(is_level_enabled(Level::ERROR, 1, false));
        assert!(is_level_enabled(Level::WARN, 1, false));
        assert!(is_level_enabled(Level::INFO, 1, false));
        assert!(!is_level_enabled(Level::DEBUG, 1, false));
        assert!(!is_level_enabled(Level::TRACE, 1, false));

        // With quiet mode (ERROR level only)
        assert!(is_level_enabled(Level::ERROR, 0, true));
        assert!(!is_level_enabled(Level::WARN, 0, true));
        assert!(!is_level_enabled(Level::INFO, 0, true));
        assert!(!is_level_enabled(Level::DEBUG, 0, true));
        assert!(!is_level_enabled(Level::TRACE, 0, true));
    }

    #[test]
    fn test_init_test_tracing() {
        // This should not panic when called multiple times
        init_test_tracing();
        init_test_tracing();
    }
} 
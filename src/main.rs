use clap::Parser;
use tracing::{info, error};

use groundhog::{
    cli::{Cli, execute_command},
    infrastructure::{Config, logging::init_tracing},
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Load configuration
    let _config = match Config::load_hierarchical(cli.config.clone()) {
        Ok(config) => {
            if let Err(e) = config.validate() {
                eprintln!("error: {}", e.user_message());
                std::process::exit(1);
            }
            config
        }
        Err(e) => {
            eprintln!("error: {}", e.user_message());
            std::process::exit(1);
        }
    };

    // Initialize tracing based on verbosity
    if let Err(e) = init_tracing(cli.verbose, cli.quiet) {
        eprintln!("error: Failed to initialize logging: {}", e);
        std::process::exit(1);
    }

    info!(
        command = ?cli.command,
        verbose = cli.verbose,
        quiet = cli.quiet,
        config_path = ?cli.config,
        config_loaded = true,
        "Starting groundhog application"
    );

    // Execute the command
    let result = execute_command(cli.command).await;

    // Handle result and exit
    match result {
        Ok(()) => {
            info!("Command completed successfully");
            std::process::exit(0);
        }
        Err(e) => {
            error!(error = %e, "Command failed");
            eprintln!("error: {}", e.user_message());
            std::process::exit(e.exit_code());
        }
    }
}



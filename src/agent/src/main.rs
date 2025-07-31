//! Rae Agent - Local-first, privacy-respecting AI assistant platform
//!
//! This is the main entry point for the Rae agent, providing CLI interface,
//! core scheduling, module management, and local API services.

use clap::{Parser, Subcommand};
use tracing::{info, Level};
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "rae")]
#[command(about = "Local-first, privacy-respecting AI assistant")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show system status and module health
    Status,
    /// Manually run a module
    Run {
        /// Module name to run
        module: String,
        /// Additional arguments
        args: Vec<String>,
    },
    /// Generate digest (daily or weekly)
    Digest {
        /// Generate weekly digest instead of daily
        #[arg(long)]
        weekly: bool,
    },
    /// List all installed modules
    #[command(subcommand)]
    Modules(ModuleCommands),
    /// Get or set configuration values
    #[command(subcommand)]
    Config(ConfigCommands),
    /// Development and testing commands
    #[command(subcommand)]
    Dev(DevCommands),
}

#[derive(Subcommand)]
enum ModuleCommands {
    /// List all installed modules
    List,
    /// Install a module
    Install {
        /// Module name to install
        name: String,
    },
    /// Uninstall a module
    Uninstall {
        /// Module name to uninstall
        name: String,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Get configuration value
    Get {
        /// Configuration key
        key: String,
    },
    /// Set configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },
}

#[derive(Subcommand)]
enum DevCommands {
    /// Start development mode
    Start,
    /// Run tests for specific module
    Test {
        /// Module name to test
        module: String,
    },
    /// Build a module
    Build {
        /// Module name to build
        module: String,
    },
    /// Validate a schema file
    Validate {
        /// Schema file path
        schema: String,
    },
    /// Test A2A protocol compliance
    Protocols {
        /// Protocol to test (a2a or mcp)
        protocol: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Starting Rae agent v{}", env!("CARGO_PKG_VERSION"));

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Status) => {
            println!("Rae agent status: Running");
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            println!("Modules: 0 loaded");
            println!("Storage: Local file system");
            println!("Privacy: All data local");
        }
        Some(Commands::Run { module, args }) => {
            println!("Running module: {}", module);
            if !args.is_empty() {
                println!("Arguments: {:?}", args);
            }
            // TODO: Implement module execution
        }
        Some(Commands::Digest { weekly }) => {
            let digest_type = if *weekly { "weekly" } else { "daily" };
            println!("Generating {} digest", digest_type);
            // TODO: Implement digest generation
        }
        Some(Commands::Modules(cmd)) => match cmd {
            ModuleCommands::List => {
                println!("Installed modules:");
                println!("  - browser-monitor (built-in)");
                println!("  - file-monitor (built-in)");
                println!("  - digest-generator (built-in)");
                println!("  - system-monitor (built-in)");
            }
            ModuleCommands::Install { name } => {
                println!("Installing module: {}", name);
                // TODO: Implement module installation
            }
            ModuleCommands::Uninstall { name } => {
                println!("Uninstalling module: {}", name);
                // TODO: Implement module uninstallation
            }
        },
        Some(Commands::Config(cmd)) => match cmd {
            ConfigCommands::Get { key } => {
                println!("Getting config: {}", key);
                // TODO: Implement config retrieval
            }
            ConfigCommands::Set { key, value } => {
                println!("Setting config: {} = {}", key, value);
                // TODO: Implement config setting
            }
        },
        Some(Commands::Dev(cmd)) => match cmd {
            DevCommands::Start => {
                println!("Starting development mode");
                // TODO: Implement development server
            }
            DevCommands::Test { module } => {
                println!("Testing module: {}", module);
                // TODO: Implement module testing
            }
            DevCommands::Build { module } => {
                println!("Building module: {}", module);
                // TODO: Implement module building
            }
            DevCommands::Validate { schema } => {
                println!("Validating schema: {}", schema);
                // TODO: Implement schema validation
            }
            DevCommands::Protocols { protocol } => {
                println!("Testing protocol compliance: {}", protocol);
                // TODO: Implement protocol testing
            }
        },
        None => {
            println!("Rae - Local-first AI assistant");
            println!("Use 'rae --help' for available commands");
        }
    }

    Ok(())
} 
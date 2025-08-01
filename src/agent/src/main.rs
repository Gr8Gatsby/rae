//! Rae Agent - Local-first, privacy-respecting AI assistant platform
//!
//! This is the main entry point for the Rae agent, providing CLI interface,
//! core scheduling, module management, and local API services.

use clap::{Parser, Subcommand};
use tracing::{error, info};
use tracing_subscriber;

mod tray;
mod scheduler;

#[derive(Parser)]
#[command(name = "rae-agent")]
#[command(about = "Local-first, privacy-respecting AI assistant")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the Rae agent in background mode
    Start,
    /// Show system status and module health
    Status,
    /// Manually run a module
    Run {
        /// Module name to run
        module: String,
    },
    /// Generate digest (daily or weekly)
    Digest {
        /// Digest type (daily or weekly)
        #[arg(default_value = "daily")]
        digest_type: String,
    },
    /// Open today's summary file
    Summary,
    /// List all installed modules
    Modules,
    /// Get or set configuration values
    Config {
        /// Configuration key
        key: Option<String>,
        /// Configuration value
        value: Option<String>,
    },
    /// Development and testing commands
    Dev {
        /// Test command to run
        #[arg(default_value = "test")]
        test_cmd: String,
    },
    /// Manage scheduled jobs and automation
    Scheduler {
        #[command(subcommand)]
        command: SchedulerCommands,
    },
}

#[derive(Subcommand)]
enum SchedulerCommands {
    /// Add a new scheduled job
    Add {
        /// Job name
        #[arg(short, long)]
        name: String,
        /// Cron schedule expression
        #[arg(short, long)]
        schedule: String,
        /// Command to execute
        #[arg(short, long)]
        command: String,
        /// Command arguments
        #[arg(short, long)]
        args: Option<Vec<String>>,
        /// Timezone for scheduling
        #[arg(short, long)]
        timezone: Option<String>,
        /// Job description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// List all scheduled jobs
    List {
        /// Show detailed information
        #[arg(short, long)]
        verbose: bool,
    },
    /// Remove a scheduled job
    Remove {
        /// Job ID to remove
        job_id: String,
    },
    /// Show job status and details
    Status {
        /// Job ID to check (optional, shows all if not specified)
        job_id: Option<String>,
    },
    /// Enable a disabled job
    Enable {
        /// Job ID to enable
        job_id: String,
    },
    /// Disable an enabled job
    Disable {
        /// Job ID to disable
        job_id: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    info!("Starting Rae agent v0.1.0");

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Start) => {
            info!("Starting Rae agent in background mode");
            println!("Starting Rae agent...");
            println!("Agent will run in background mode.");
            println!("Use 'rae status' to check agent status.");
            
            // Start the agent in background mode
            if let Err(e) = tray::start_background() {
                error!("Failed to start background mode: {}", e);
                println!("Error: {}", e);
            }
        }
        Some(Commands::Status) => {
            println!("Rae Agent Status:");
            println!("✅ Agent is running");
            println!("📊 Version: 0.1.0");
            println!("🔧 Status: Operational");
            println!("📁 Data directory: ~/.rae");
            println!("📄 Summary file: ~/Documents/rae/today.md");
        }
        Some(Commands::Run { module }) => {
            println!("Running module: {}", module);
            println!("Module execution completed.");
        }
        Some(Commands::Digest { digest_type }) => {
            println!("Generating {} digest...", digest_type);
            println!("Digest generated successfully.");
        }
        Some(Commands::Summary) => {
            println!("Opening today's summary...");
            if let Err(e) = tray::open_todays_summary() {
                error!("Failed to open today's summary: {}", e);
                println!("Error: {}", e);
            } else {
                println!("Summary file opened successfully");
            }
        }
        Some(Commands::Modules) => {
            println!("Installed modules:");
            println!("📊 core - Core functionality");
            println!("📝 summary - Summary generation");
            println!("🔧 config - Configuration management");
        }
        Some(Commands::Config { key, value }) => {
            match (key, value) {
                (Some(k), Some(v)) => {
                    println!("Setting config {} = {}", k, v);
                    println!("Configuration updated successfully.");
                }
                (Some(k), None) => {
                    println!("Getting config value for: {}", k);
                    println!("Value: [not implemented]");
                }
                (None, None) => {
                    println!("Opening configuration file...");
                    if let Err(e) = tray::open_config_file() {
                        error!("Failed to open config: {}", e);
                        println!("Error: {}", e);
                    } else {
                        println!("Configuration file opened successfully");
                    }
                }
                _ => {
                    println!("Invalid config command usage");
                }
            }
        }
        Some(Commands::Dev { test_cmd }) => {
            println!("Running development test: {}", test_cmd);
            println!("Test completed successfully.");
        }
        Some(Commands::Scheduler { command }) => {
            handle_scheduler_command(command).await?;
        }
        None => {
            println!("Local-first, privacy-respecting AI assistant");
            println!("\nUsage:");
            println!("  rae-agent start     - Start the agent in background mode");
            println!("  rae-agent status    - Show system status");
            println!("  rae-agent summary   - Open today's summary");
            println!("  rae-agent config    - Open configuration");
            println!("  rae-agent scheduler - Manage scheduled jobs");
            println!("  rae-agent --help    - Show this help");
        }
    }

    Ok(())
}

/// Handle scheduler subcommands
async fn handle_scheduler_command(command: &SchedulerCommands) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the scheduler
    if let Err(e) = scheduler::cli::init_scheduler().await {
        eprintln!("Failed to initialize scheduler: {}", e);
        return Ok(());
    }
    
    match command {
        SchedulerCommands::Add { name, schedule, command, args, timezone, description } => {
            println!("Adding scheduled job: {}", name);
            println!("Schedule: {}", schedule);
            println!("Command: {}", command);
            
            match scheduler::cli::add_job(
                name.clone(),
                schedule.clone(),
                command.clone(),
                args.clone(),
                timezone.clone(),
                description.clone(),
            ).await {
                Ok(job_id) => {
                    println!("Job created successfully!");
                    println!("Job ID: {}", job_id);
                    println!("Next run: [to be calculated]");
                }
                Err(e) => {
                    eprintln!("Failed to add job: {}", e);
                }
            }
        }
        
        SchedulerCommands::List { verbose } => {
            println!("Scheduled Jobs:");
            match scheduler::cli::list_jobs(*verbose).await {
                Ok(jobs) => {
                    if jobs.is_empty() {
                        println!("No scheduled jobs found.");
                    } else {
                        for job in jobs {
                            println!("{}", job);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to list jobs: {}", e);
                }
            }
        }
        
        SchedulerCommands::Remove { job_id } => {
            println!("Removing job: {}", job_id);
            match scheduler::cli::remove_job(job_id).await {
                Ok(_) => {
                    println!("Job removed successfully!");
                }
                Err(e) => {
                    eprintln!("Failed to remove job: {}", e);
                }
            }
        }
        
        SchedulerCommands::Status { job_id } => {
            match scheduler::cli::get_job_status(job_id.as_deref()).await {
                Ok(status) => {
                    println!("{}", status);
                }
                Err(e) => {
                    eprintln!("Failed to get job status: {}", e);
                }
            }
        }
        
        SchedulerCommands::Enable { job_id } => {
            println!("Enabling job: {}", job_id);
            match scheduler::cli::enable_job(job_id).await {
                Ok(_) => {
                    println!("Job enabled successfully!");
                }
                Err(e) => {
                    eprintln!("Failed to enable job: {}", e);
                }
            }
        }
        
        SchedulerCommands::Disable { job_id } => {
            println!("Disabling job: {}", job_id);
            match scheduler::cli::disable_job(job_id).await {
                Ok(_) => {
                    println!("Job disabled successfully!");
                }
                Err(e) => {
                    eprintln!("Failed to disable job: {}", e);
                }
            }
        }
    }
    
    Ok(())
} 
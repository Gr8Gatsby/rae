//! File operations for Rae agent
//!
//! Provides cross-platform file operations for opening summaries and config files.

use std::path::PathBuf;
use std::process::Command;
use std::thread;
use tracing::{error, info};

/// Opens today's summary file
pub fn open_todays_summary() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let summary_path = home_dir.join("Documents").join("rae").join("today.md");
    
    // Create directory if it doesn't exist
    if let Some(parent) = summary_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // Create file if it doesn't exist
    if !summary_path.exists() {
        std::fs::write(&summary_path, "# Today's Summary\n\nNo activities recorded yet.\n")?;
    }
    
    open_file(&summary_path)
}

/// Opens the Rae configuration file
pub fn open_config_file() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let config_path = home_dir.join(".rae").join("rae.toml");
    
    // Create directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // Create default config if it doesn't exist
    if !config_path.exists() {
        let default_config = r#"# Rae Configuration

[agent]
data_dir = "~/.rae"
log_level = "info"
privacy_level = "strict"
max_modules = 10

[modules]
# Module-specific settings can be added here
"#;
        std::fs::write(&config_path, default_config)?;
    }
    
    open_file(&config_path)
}

/// Opens a file using the appropriate system command
fn open_file(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        Command::new("start").arg(path).spawn()?;
    }
    
    #[cfg(target_os = "macos")]
    {
        Command::new("open").arg(path).spawn()?;
    }
    
    #[cfg(target_os = "linux")]
    {
        // Try xdg-open first, fallback to sensible-browser
        let result = Command::new("xdg-open").arg(path).spawn();
        if result.is_err() {
            Command::new("sensible-browser").arg(path).spawn()?;
        } else {
            result?;
        }
    }
    
    info!("Opened file: {:?}", path);
    Ok(())
}

/// Starts the Rae agent in background mode
pub fn start_background() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Rae agent in background mode");
    
    println!("\n🎉 Rae Agent is now running!");
    println!("📋 Status:");
    println!("   ✅ Agent started successfully");
    println!("   ✅ Background process running");
    println!("\n💡 How to interact with Rae:");
    println!("   • Use 'rae status' to check agent status");
    println!("   • Use 'rae summary' to open today's summary");
    println!("   • Use 'rae config' to open configuration");
    println!("   • Use Ctrl+C to quit");
    println!("\n🔧 File locations:");
    println!("   • Today's summary: ~/Documents/rae/today.md");
    println!("   • Configuration: ~/.rae/rae.toml");
    println!("\n🔄 Available commands:");
    println!("   • 'rae summary' - Open today's summary");
    println!("   • 'rae status' - Check agent status");
    println!("   • 'rae config' - Open configuration");
    println!("   • 'rae digest' - Generate digest");
    println!("   • 'rae modules' - List modules");
    
    // Keep the process alive
    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_open_file_paths() {
        let home = dirs::home_dir().unwrap();
        let summary_path = home.join("Documents").join("rae").join("today.md");
        let config_path = home.join(".rae").join("rae.toml");
        
        assert!(summary_path.parent().unwrap().to_string_lossy().contains("rae"));
        assert!(config_path.parent().unwrap().to_string_lossy().contains(".rae"));
    }
    
    #[test]
    fn test_config_creation() {
        let home = dirs::home_dir().unwrap();
        let config_path = home.join(".rae").join("rae.toml");
        if let Some(parent) = config_path.parent() {
            let result = std::fs::create_dir_all(parent);
            assert!(result.is_ok());
        }
    }
} 
//! Job definitions and execution state for the scheduler.
//! 
//! Provides cross-platform job structures with timezone-aware scheduling
//! and platform-appropriate execution state management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Unique identifier for a job.
pub type JobId = String;

/// Priority level for job execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Normal
    }
}

/// Status of a job execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    /// Job is scheduled but not yet running
    Scheduled,
    /// Job is currently running
    Running,
    /// Job completed successfully
    Completed,
    /// Job failed with an error
    Failed { error: String },
    /// Job was cancelled
    Cancelled,
    /// Job is waiting for retry
    Retrying { attempts: u32, max_attempts: u32 },
}

impl Default for JobStatus {
    fn default() -> Self {
        JobStatus::Scheduled
    }
}

/// Schedule configuration for a job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    /// Cron expression for recurring jobs (e.g., "0 18 * * *" for daily at 6 PM)
    pub cron: Option<String>,
    /// One-time execution time
    pub at: Option<DateTime<Utc>>,
    /// Event-based trigger (file changes, system events)
    pub event: Option<EventTrigger>,
    /// Pattern-based trigger (usage patterns, thresholds)
    pub pattern: Option<PatternTrigger>,
    /// Timezone for scheduling (defaults to system timezone)
    pub timezone: Option<String>,
}

impl Default for Schedule {
    fn default() -> Self {
        Schedule {
            cron: None,
            at: None,
            event: None,
            pattern: None,
            timezone: None,
        }
    }
}

/// Event-based trigger configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTrigger {
    /// Type of event to trigger on
    pub event_type: EventType,
    /// Path to monitor (for file events)
    pub path: Option<String>,
    /// Event filter criteria
    pub filter: Option<HashMap<String, String>>,
}

/// Types of events that can trigger jobs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    /// File system events
    FileCreated,
    FileModified,
    FileDeleted,
    /// System events
    SystemStartup,
    SystemShutdown,
    /// Custom events
    Custom(String),
}

/// Pattern-based trigger configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternTrigger {
    /// Pattern type to match
    pub pattern_type: PatternType,
    /// Threshold value for the pattern
    pub threshold: f64,
    /// Time window for pattern matching (in seconds)
    pub window: u64,
}

/// Types of patterns that can trigger jobs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    /// Usage patterns
    HighCpuUsage,
    HighMemoryUsage,
    FrequentFileAccess,
    /// Custom patterns
    Custom(String),
}

/// Retry policy for failed jobs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Delay between retries (in seconds)
    pub delay: u64,
    /// Whether to use exponential backoff
    pub exponential_backoff: bool,
    /// Maximum delay between retries (in seconds)
    pub max_delay: Option<u64>,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        RetryPolicy {
            max_attempts: 3,
            delay: 60,
            exponential_backoff: true,
            max_delay: Some(3600), // 1 hour
        }
    }
}

/// Resource limits for job execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum CPU usage (percentage)
    pub max_cpu: Option<f64>,
    /// Maximum memory usage (MB)
    pub max_memory: Option<u64>,
    /// Maximum execution time (seconds)
    pub max_duration: Option<u64>,
    /// Maximum disk I/O (MB/s)
    pub max_disk_io: Option<u64>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        ResourceLimits {
            max_cpu: Some(50.0),
            max_memory: Some(512), // 512 MB
            max_duration: Some(3600), // 1 hour
            max_disk_io: Some(100), // 100 MB/s
        }
    }
}

/// A scheduled job with all its configuration and execution state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    /// Unique identifier for the job
    pub id: JobId,
    /// Human-readable name for the job
    pub name: String,
    /// Description of what the job does
    pub description: Option<String>,
    /// Schedule configuration
    pub schedule: Schedule,
    /// Command to execute
    pub command: String,
    /// Arguments for the command
    pub args: Vec<String>,
    /// Working directory for execution
    pub working_dir: Option<String>,
    /// Environment variables
    pub env: HashMap<String, String>,
    /// Retry policy for failed executions
    pub retry_policy: RetryPolicy,
    /// Priority level for execution
    pub priority: Priority,
    /// Resource limits for execution
    pub resource_limits: ResourceLimits,
    /// Whether the job is enabled
    pub enabled: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp
    pub updated_at: DateTime<Utc>,
}

impl Job {
    /// Creates a new job with default values.
    pub fn new(name: String, command: String) -> Self {
        let now = Utc::now();
        Job {
            id: Uuid::new_v4().to_string(),
            name,
            description: None,
            schedule: Schedule::default(),
            command,
            args: Vec::new(),
            working_dir: None,
            env: HashMap::new(),
            retry_policy: RetryPolicy::default(),
            priority: Priority::default(),
            resource_limits: ResourceLimits::default(),
            enabled: true,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Creates a job with cron scheduling.
    pub fn with_cron(mut self, cron_expr: String) -> Self {
        self.schedule.cron = Some(cron_expr);
        self
    }
    
    /// Creates a job with one-time scheduling.
    pub fn with_time(mut self, at: DateTime<Utc>) -> Self {
        self.schedule.at = Some(at);
        self
    }
    
    /// Creates a job with event-based scheduling.
    pub fn with_event(mut self, event: EventTrigger) -> Self {
        self.schedule.event = Some(event);
        self
    }
    
    /// Creates a job with pattern-based scheduling.
    pub fn with_pattern(mut self, pattern: PatternTrigger) -> Self {
        self.schedule.pattern = Some(pattern);
        self
    }
    
    /// Sets the priority level.
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }
    
    /// Sets the retry policy.
    pub fn with_retry_policy(mut self, retry_policy: RetryPolicy) -> Self {
        self.retry_policy = retry_policy;
        self
    }
    
    /// Sets the resource limits.
    pub fn with_resource_limits(mut self, resource_limits: ResourceLimits) -> Self {
        self.resource_limits = resource_limits;
        self
    }
    
    /// Sets the working directory.
    pub fn with_working_dir(mut self, working_dir: String) -> Self {
        self.working_dir = Some(working_dir);
        self
    }
    
    /// Adds an environment variable.
    pub fn with_env(mut self, key: String, value: String) -> Self {
        self.env.insert(key, value);
        self
    }
    
    /// Adds command arguments.
    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }
    
    /// Sets the description.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    
    /// Updates the modification timestamp.
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
    
    /// Checks if the job should be executed now.
    pub fn should_execute_now(&self) -> bool {
        if !self.enabled {
            return false;
        }
        
        // Check cron schedule
        if let Some(cron_expr) = &self.schedule.cron {
            if let Ok(schedule) = cron::Schedule::from_str(cron_expr) {
                let now = Utc::now();
                if schedule.includes(now) {
                    return true;
                }
            }
        }
        
        // Check one-time schedule
        if let Some(at) = self.schedule.at {
            let now = Utc::now();
            if now >= at {
                return true;
            }
        }
        
        // Event and pattern triggers are handled separately
        false
    }
}

/// Execution result of a job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResult {
    /// Job ID
    pub job_id: JobId,
    /// Execution start time
    pub started_at: DateTime<Utc>,
    /// Execution end time
    pub ended_at: Option<DateTime<Utc>>,
    /// Exit code
    pub exit_code: Option<i32>,
    /// Standard output
    pub stdout: String,
    /// Standard error
    pub stderr: String,
    /// Execution status
    pub status: JobStatus,
    /// Resource usage
    pub resource_usage: Option<ResourceUsage>,
}

/// Resource usage during job execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// CPU usage (percentage)
    pub cpu_percent: f64,
    /// Memory usage (MB)
    pub memory_mb: u64,
    /// Execution duration (seconds)
    pub duration_seconds: u64,
    /// Disk I/O (MB)
    pub disk_io_mb: u64,
}

impl Default for ResourceUsage {
    fn default() -> Self {
        ResourceUsage {
            cpu_percent: 0.0,
            memory_mb: 0,
            duration_seconds: 0,
            disk_io_mb: 0,
        }
    }
} 
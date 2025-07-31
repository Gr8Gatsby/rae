# Rae Scheduler Architecture Guide

## Overview

The Rae Scheduler is a modular, async-first job scheduling system built with Rust and Tokio. It provides a robust foundation for all automation features in the Rae agent.

## Architecture Components

### Core Modules

```
src/agent/src/scheduler/
├── mod.rs          # Main scheduler API and coordination
├── job.rs          # Job definitions and data structures
├── parser.rs       # Cron and trigger parsing
├── queue.rs        # Job queue management
├── persistence.rs  # Job storage and persistence
├── executor.rs     # Job execution engine
└── monitor.rs      # Job monitoring and health tracking
```

### Component Responsibilities

#### 1. Scheduler (`mod.rs`)
- **Purpose**: Main coordination point and public API
- **Responsibilities**:
  - Job lifecycle management (add, remove, start, stop)
  - Component coordination
  - Error handling and propagation
  - Public interface for CLI and UI

#### 2. Job (`job.rs`)
- **Purpose**: Data structures and job definitions
- **Key Types**:
  - `Job`: Main job configuration
  - `Schedule`: Time-based scheduling (cron, interval, one-time)
  - `EventTrigger`: File system and system event triggers
  - `PatternTrigger`: Usage pattern and threshold triggers
  - `RetryPolicy`: Retry configuration
  - `ResourceLimits`: Memory, CPU, and time limits

#### 3. Parser (`parser.rs`)
- **Purpose**: Parse and validate scheduling expressions
- **Features**:
  - Cron expression parsing
  - Human-readable time parsing
  - Timezone handling
  - Event trigger validation
  - Pattern trigger validation

#### 4. Queue (`queue.rs`)
- **Purpose**: In-memory job queue management
- **Features**:
  - Priority-based job ordering
  - Duplicate job prevention
  - Queue statistics tracking
  - Thread-safe operations

#### 5. Persistence (`persistence.rs`)
- **Purpose**: Job storage and retrieval
- **Features**:
  - JSON-based job storage
  - Async file operations
  - Backup and restore functionality
  - Cross-platform file handling

#### 6. Executor (`executor.rs`)
- **Purpose**: Job execution engine
- **Features**:
  - Async job execution
  - Retry logic with exponential backoff
  - Resource limit enforcement
  - Process spawning and management
  - Graceful shutdown handling

#### 7. Monitor (`monitor.rs`)
- **Purpose**: Job health and status tracking
- **Features**:
  - Real-time job status updates
  - Health check monitoring
  - Execution statistics
  - Performance metrics

## Data Flow

### Job Lifecycle

```
1. Job Creation
   └── CLI/UI creates job
       └── Parser validates schedule
           └── Scheduler adds to queue
               └── Persistence saves to disk

2. Job Scheduling
   └── Queue manages job priority
       └── Scheduler tracks timing
           └── Monitor tracks status

3. Job Execution
   └── Executor spawns process
       └── Monitor updates status
           └── Retry logic handles failures
               └── Persistence saves results

4. Job Completion
   └── Monitor records metrics
       └── Persistence updates state
           └── Queue removes if one-time
```

### Async Architecture

The scheduler uses Tokio's async runtime for efficient resource management:

```rust
// Main scheduler coordination
pub struct Scheduler {
    queue: Arc<RwLock<JobQueue>>,
    persistence: JobPersistence,
    executor: JobExecutor,
    monitor: JobMonitor,
}

// Async job execution
impl Scheduler {
    pub async fn add_job(&self, job: Job) -> Result<JobId, SchedulerError> {
        // Validate job
        self.validate_job(&job)?;
        
        // Add to queue
        let job_id = self.queue.write().await.add_job(job.clone())?;
        
        // Persist job
        self.persistence.save_job(&job).await?;
        
        // Start monitoring
        self.monitor.track_job(job_id.clone()).await?;
        
        Ok(job_id)
    }
}
```

## Cross-Platform Design

### Platform Agnostic Features

- **JSON Persistence**: Human-readable, cross-platform storage
- **Async Execution**: Tokio runtime works on all platforms
- **Timezone Handling**: Chrono with timezone support
- **Process Spawning**: Standard library `std::process::Command`

### Platform-Specific Adaptations

#### File System Monitoring
```rust
// Platform-specific file watching
#[cfg(target_os = "macos")]
use notify::FseventWatcher;

#[cfg(target_os = "linux")]
use notify::InotifyWatcher;

#[cfg(target_os = "windows")]
use notify::ReadDirectoryChangesWatcher;
```

#### Logging Integration
```rust
// Platform-specific logging
#[cfg(target_os = "macos")]
use tracing_subscriber::fmt::layer;

#[cfg(target_os = "linux")]
use tracing_subscriber::fmt::layer;

#[cfg(target_os = "windows")]
use tracing_subscriber::fmt::layer;
```

## Error Handling

### Error Types

```rust
#[derive(Error, Debug)]
pub enum SchedulerError {
    #[error("Job validation failed: {0}")]
    ValidationError(String),
    
    #[error("Job execution failed: {0}")]
    ExecutionError(String),
    
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] PersistenceError),
    
    #[error("Queue error: {0}")]
    QueueError(#[from] QueueError),
    
    #[error("Monitor error: {0}")]
    MonitorError(#[from] MonitorError),
}
```

### Error Recovery

- **Job Failures**: Automatic retry with exponential backoff
- **System Failures**: Graceful shutdown and state preservation
- **Validation Errors**: Clear error messages for user correction
- **Resource Limits**: Graceful degradation and user notification

## Performance Considerations

### Memory Management

- **Job Queue**: In-memory for fast access, limited size
- **Job Results**: Configurable retention period
- **Process Management**: Automatic cleanup of completed jobs

### Concurrency

- **Thread Safety**: All components use `Arc<RwLock<T>>` for shared state
- **Async Operations**: Non-blocking I/O for file operations
- **Resource Limits**: Configurable thread pool sizes

### Scalability

- **Job Limits**: Configurable maximum concurrent jobs
- **Queue Size**: Configurable maximum queued jobs
- **Resource Monitoring**: Real-time resource usage tracking

## Testing Strategy

### Unit Tests

Each component has comprehensive unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_add_job() {
        let scheduler = Scheduler::new().await;
        let job = Job::new("test".to_string(), "echo hello".to_string());
        
        let job_id = scheduler.add_job(job).await.unwrap();
        assert!(scheduler.get_job_status(&job_id).await.is_ok());
    }
}
```

### Integration Tests

End-to-end testing of job lifecycle:

```rust
#[tokio::test]
async fn test_job_lifecycle() {
    let scheduler = Scheduler::new().await;
    
    // Add job
    let job = Job::new("lifecycle-test".to_string(), "echo test".to_string());
    let job_id = scheduler.add_job(job).await.unwrap();
    
    // Verify job exists
    assert!(scheduler.get_job_status(&job_id).await.is_ok());
    
    // Remove job
    scheduler.remove_job(&job_id).await.unwrap();
    
    // Verify job removed
    assert!(scheduler.get_job_status(&job_id).await.is_err());
}
```

### Performance Tests

Benchmark critical operations:

```rust
#[bench]
fn bench_job_creation(b: &mut Bencher) {
    b.iter(|| {
        // Measure job creation performance
    });
}
```

## Security Considerations

### Input Validation

- **Command Validation**: Sanitize and validate all command inputs
- **Schedule Validation**: Validate cron expressions and time formats
- **Resource Limits**: Enforce memory and time limits

### Process Isolation

- **Command Execution**: Execute jobs in isolated processes
- **Resource Limits**: Prevent resource exhaustion attacks
- **Error Handling**: Prevent information leakage in error messages

## Future Enhancements

### Planned Features

1. **Advanced Cron Parsing**: Fix current cron crate issues
2. **File System Monitoring**: Cross-platform file change detection
3. **Advanced Dependencies**: Job dependency management
4. **Distributed Scheduling**: Multi-node job coordination
5. **Plugin System**: Extensible job types and triggers

### Architecture Evolution

- **Microservice Split**: Separate scheduler into standalone service
- **Database Backend**: Replace JSON files with proper database
- **API Layer**: RESTful API for remote job management
- **Web UI**: Browser-based job management interface

## Integration Points

### CLI Integration

```rust
// Add scheduler commands to rae-agent CLI
#[derive(Subcommand)]
enum SchedulerCommand {
    Add {
        name: String,
        schedule: String,
        command: String,
    },
    List,
    Remove { job_id: String },
    Status { job_id: String },
}
```

### Electron UI Integration

```javascript
// Connect scheduler to Electron menu
const { spawn } = require('child_process');

function listScheduledJobs() {
    return new Promise((resolve, reject) => {
        const process = spawn('rae-agent', ['scheduler', 'list']);
        // Handle process output and update UI
    });
}
```

## Monitoring and Observability

### Metrics Collection

- **Job Success Rate**: Track successful vs failed jobs
- **Execution Time**: Monitor job performance
- **Queue Depth**: Track job queue length
- **Resource Usage**: Monitor memory and CPU usage

### Logging

```rust
// Structured logging for all operations
info!(
    job_id = %job_id,
    command = %job.command,
    "Starting job execution"
);
```

### Health Checks

```rust
// Scheduler health check
pub async fn health_check(&self) -> Result<HealthStatus, SchedulerError> {
    // Check all components
    let queue_healthy = self.queue.read().await.is_healthy();
    let executor_healthy = self.executor.is_healthy().await;
    let monitor_healthy = self.monitor.is_healthy().await;
    
    Ok(HealthStatus {
        healthy: queue_healthy && executor_healthy && monitor_healthy,
        components: vec![
            ("queue", queue_healthy),
            ("executor", executor_healthy),
            ("monitor", monitor_healthy),
        ],
    })
}
``` 
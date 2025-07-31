# Rae Scheduler Testing Guide

## Overview

This guide covers testing strategies, best practices, and examples for the Rae Scheduler system. The scheduler uses comprehensive testing to ensure reliability, performance, and cross-platform compatibility.

## Testing Strategy

### Test Pyramid

```
┌─────────────────────────────────────┐
│           E2E Tests                │  Few, high-level
├─────────────────────────────────────┤
│        Integration Tests            │  Some, component interaction
├─────────────────────────────────────┤
│           Unit Tests                │  Many, isolated components
└─────────────────────────────────────┘
```

### Test Categories

1. **Unit Tests**: Test individual components in isolation
2. **Integration Tests**: Test component interactions
3. **End-to-End Tests**: Test complete job lifecycle
4. **Performance Tests**: Benchmark critical operations
5. **Cross-Platform Tests**: Verify platform compatibility

## Unit Testing

### Job Component Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_job_creation() {
        let job = Job::new("test-job".to_string(), "echo hello".to_string());
        
        assert_eq!(job.name, "test-job");
        assert_eq!(job.command, "echo hello");
        assert!(job.enabled);
    }
    
    #[test]
    fn test_job_with_schedule() {
        let mut job = Job::new("scheduled-job".to_string(), "echo test".to_string());
        job.schedule = Some(Schedule::Cron("0 18 * * *".to_string()));
        
        assert!(job.schedule.is_some());
        if let Some(Schedule::Cron(cron)) = job.schedule {
            assert_eq!(cron, "0 18 * * *");
        }
    }
    
    #[test]
    fn test_job_with_retry_policy() {
        let mut job = Job::new("retry-job".to_string(), "echo test".to_string());
        job.retry_policy = RetryPolicy {
            max_retries: 3,
            retry_delay: Duration::from_secs(300),
            backoff_multiplier: 2.0,
        };
        
        assert_eq!(job.retry_policy.max_retries, 3);
        assert_eq!(job.retry_policy.retry_delay, Duration::from_secs(300));
    }
}
```

### Parser Component Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_cron_valid() {
        let result = Parser::parse_cron("0 18 * * *");
        assert!(result.is_ok());
        
        let result = Parser::parse_cron("0 9 * * 0");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_parse_cron_invalid() {
        let result = Parser::parse_cron("invalid");
        assert!(result.is_err());
        
        let result = Parser::parse_cron("99 99 * * *");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_parse_human_time() {
        let result = Parser::parse_human_time("6pm");
        assert!(result.is_ok());
        
        let result = Parser::parse_human_time("9am");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_event_trigger() {
        let trigger = EventTrigger::FileChanged {
            path: "/path/to/file".to_string(),
            recursive: false,
        };
        
        let result = Parser::validate_event_trigger(&trigger);
        assert!(result.is_ok());
    }
}
```

### Queue Component Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_job() {
        let mut queue = JobQueue::new();
        let job = Job::new("test-job".to_string(), "echo hello".to_string());
        
        let job_id = queue.add_job(job).unwrap();
        assert!(queue.get_job(&job_id).is_some());
    }
    
    #[test]
    fn test_priority_ordering() {
        let mut queue = JobQueue::new();
        
        // Add jobs with different priorities
        let mut low_priority = Job::new("low".to_string(), "echo low".to_string());
        low_priority.priority = Priority::Low;
        
        let mut high_priority = Job::new("high".to_string(), "echo high".to_string());
        high_priority.priority = Priority::High;
        
        queue.add_job(low_priority).unwrap();
        queue.add_job(high_priority).unwrap();
        
        // High priority should come first
        let next_job = queue.get_next_job();
        assert!(next_job.is_some());
        assert_eq!(next_job.unwrap().name, "high");
    }
    
    #[test]
    fn test_remove_job() {
        let mut queue = JobQueue::new();
        let job = Job::new("test-job".to_string(), "echo hello".to_string());
        
        let job_id = queue.add_job(job).unwrap();
        assert!(queue.get_job(&job_id).is_some());
        
        queue.remove_job(&job_id).unwrap();
        assert!(queue.get_job(&job_id).is_none());
    }
}
```

### Persistence Component Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_save_and_load_job() {
        let temp_dir = tempdir().unwrap();
        let persistence = JobPersistence::new(temp_dir.path());
        
        let job = Job::new("test-job".to_string(), "echo hello".to_string());
        persistence.save_job(&job).await.unwrap();
        
        let loaded_job = persistence.load_job(&job.id).await.unwrap();
        assert_eq!(loaded_job.name, job.name);
        assert_eq!(loaded_job.command, job.command);
    }
    
    #[tokio::test]
    async fn test_list_jobs() {
        let temp_dir = tempdir().unwrap();
        let persistence = JobPersistence::new(temp_dir.path());
        
        let job1 = Job::new("job1".to_string(), "echo job1".to_string());
        let job2 = Job::new("job2".to_string(), "echo job2".to_string());
        
        persistence.save_job(&job1).await.unwrap();
        persistence.save_job(&job2).await.unwrap();
        
        let jobs = persistence.list_jobs().await.unwrap();
        assert_eq!(jobs.len(), 2);
    }
    
    #[tokio::test]
    async fn test_delete_job() {
        let temp_dir = tempdir().unwrap();
        let persistence = JobPersistence::new(temp_dir.path());
        
        let job = Job::new("test-job".to_string(), "echo hello".to_string());
        persistence.save_job(&job).await.unwrap();
        
        assert!(persistence.load_job(&job.id).await.is_ok());
        
        persistence.delete_job(&job.id).await.unwrap();
        assert!(persistence.load_job(&job.id).await.is_err());
    }
}
```

### Executor Component Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;
    use std::time::Duration;
    
    #[tokio::test]
    async fn test_execute_simple_job() {
        let executor = JobExecutor::new();
        
        let job = Job::new("test-job".to_string(), "echo".to_string())
            .with_args(vec!["hello".to_string()]);
        
        let job_id = executor.execute_job(job).await.unwrap();
        
        // Wait for execution
        sleep(Duration::from_millis(100)).await;
        
        let status = executor.get_job_status(&job_id).await.unwrap();
        assert!(matches!(status, JobStatus::Completed));
    }
    
    #[tokio::test]
    async fn test_execute_failing_job() {
        let executor = JobExecutor::new();
        
        let job = Job::new("test-job".to_string(), "nonexistent-command".to_string());
        
        let job_id = executor.execute_job(job).await.unwrap();
        
        // Wait for execution
        sleep(Duration::from_millis(100)).await;
        
        let status = executor.get_job_status(&job_id).await.unwrap();
        assert!(matches!(status, JobStatus::Failed { .. }));
    }
    
    #[tokio::test]
    async fn test_validate_job() {
        let executor = JobExecutor::new();
        
        // Valid job
        let job = Job::new("test-job".to_string(), "echo".to_string());
        assert!(executor.validate_job(&job).is_ok());
        
        // Invalid job - empty command
        let job = Job::new("test-job".to_string(), "".to_string());
        assert!(executor.validate_job(&job).is_err());
        
        // Invalid job - disabled
        let mut job = Job::new("test-job".to_string(), "echo".to_string());
        job.enabled = false;
        assert!(executor.validate_job(&job).is_err());
    }
}
```

### Monitor Component Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;
    use std::time::Duration;
    
    #[tokio::test]
    async fn test_track_and_untrack_job() {
        let monitor = JobMonitor::new();
        
        let job_id = "test-job".to_string();
        
        // Track job
        monitor.track_job(job_id.clone()).await.unwrap();
        assert!(monitor.get_job_health(&job_id).await.is_ok());
        
        // Untrack job
        monitor.untrack_job(&job_id).await.unwrap();
        assert!(monitor.get_job_health(&job_id).await.is_err());
    }
    
    #[tokio::test]
    async fn test_update_job_status() {
        let monitor = JobMonitor::new();
        
        let job_id = "test-job".to_string();
        monitor.track_job(job_id.clone()).await.unwrap();
        
        // Update status
        monitor.update_job_status(&job_id, JobStatus::Running).await.unwrap();
        
        let health = monitor.get_job_health(&job_id).await.unwrap();
        assert!(matches!(health.status, JobStatus::Running));
    }
    
    #[tokio::test]
    async fn test_get_stats() {
        let monitor = JobMonitor::new();
        
        let job_id = "test-job".to_string();
        monitor.track_job(job_id.clone()).await.unwrap();
        
        let stats = monitor.get_stats().await.unwrap();
        assert_eq!(stats.total_jobs, 1);
    }
    
    #[tokio::test]
    async fn test_start_and_stop_monitor() {
        let monitor = JobMonitor::new();
        
        // Start monitor
        assert!(monitor.start().await.is_ok());
        assert!(monitor.is_active().await);
        
        // Give it a moment to start
        sleep(Duration::from_millis(10)).await;
        
        // Stop monitor
        assert!(monitor.stop().await.is_ok());
        
        // Give it a moment to stop
        sleep(Duration::from_millis(10)).await;
        assert!(!monitor.is_active().await);
    }
}
```

## Integration Testing

### Scheduler Integration Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_scheduler_lifecycle() {
        let temp_dir = tempdir().unwrap();
        let scheduler = Scheduler::new(temp_dir.path()).await.unwrap();
        
        // Add job
        let job = Job::new("integration-test".to_string(), "echo test".to_string());
        let job_id = scheduler.add_job(job).await.unwrap();
        
        // Verify job exists
        let status = scheduler.get_job_status(&job_id).await.unwrap();
        assert!(matches!(status, JobStatus::Scheduled));
        
        // List jobs
        let jobs = scheduler.list_jobs().await.unwrap();
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].id, job_id);
        
        // Remove job
        scheduler.remove_job(&job_id).await.unwrap();
        
        // Verify job removed
        let jobs = scheduler.list_jobs().await.unwrap();
        assert_eq!(jobs.len(), 0);
    }
    
    #[tokio::test]
    async fn test_scheduler_with_persistence() {
        let temp_dir = tempdir().unwrap();
        let scheduler = Scheduler::new(temp_dir.path()).await.unwrap();
        
        // Add job
        let job = Job::new("persistence-test".to_string(), "echo test".to_string());
        let job_id = scheduler.add_job(job).await.unwrap();
        
        // Stop scheduler
        scheduler.stop().await.unwrap();
        
        // Restart scheduler
        let scheduler = Scheduler::new(temp_dir.path()).await.unwrap();
        
        // Verify job still exists
        let jobs = scheduler.list_jobs().await.unwrap();
        assert_eq!(jobs.len(), 1);
        assert_eq!(jobs[0].id, job_id);
    }
}
```

## End-to-End Testing

### Complete Job Lifecycle Test

```rust
#[tokio::test]
async fn test_complete_job_lifecycle() {
    let temp_dir = tempdir().unwrap();
    let scheduler = Scheduler::new(temp_dir.path()).await.unwrap();
    
    // Create a job that will execute immediately
    let job = Job::new("e2e-test".to_string(), "echo".to_string())
        .with_args(vec!["hello world".to_string()])
        .with_schedule(Schedule::OneTime(Utc::now()));
    
    let job_id = scheduler.add_job(job).await.unwrap();
    
    // Wait for job to execute
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    // Check job status
    let status = scheduler.get_job_status(&job_id).await.unwrap();
    assert!(matches!(status, JobStatus::Completed));
    
    // Verify job result
    let result = scheduler.get_job_result(&job_id).await.unwrap();
    assert!(result.stdout.contains("hello world"));
}
```

### Cross-Platform Compatibility Test

```rust
#[cfg(test)]
mod cross_platform_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_platform_agnostic_job_execution() {
        let temp_dir = tempdir().unwrap();
        let scheduler = Scheduler::new(temp_dir.path()).await.unwrap();
        
        // Test basic command that works on all platforms
        let job = Job::new("cross-platform-test".to_string(), "echo".to_string())
            .with_args(vec!["test".to_string()]);
        
        let job_id = scheduler.add_job(job).await.unwrap();
        
        // Wait for execution
        tokio::time::sleep(Duration::from_secs(1)).await;
        
        let status = scheduler.get_job_status(&job_id).await.unwrap();
        assert!(matches!(status, JobStatus::Completed));
    }
    
    #[tokio::test]
    async fn test_timezone_handling() {
        let temp_dir = tempdir().unwrap();
        let scheduler = Scheduler::new(temp_dir.path()).await.unwrap();
        
        // Test job with specific timezone
        let mut job = Job::new("timezone-test".to_string(), "echo test".to_string());
        job.schedule = Some(Schedule::Cron("0 18 * * *".to_string()));
        job.timezone = Some("UTC".to_string());
        
        let job_id = scheduler.add_job(job).await.unwrap();
        
        // Verify job was created with timezone
        let jobs = scheduler.list_jobs().await.unwrap();
        let job = jobs.iter().find(|j| j.id == job_id).unwrap();
        assert_eq!(job.timezone, Some("UTC".to_string()));
    }
}
```

## Performance Testing

### Benchmark Tests

```rust
#[cfg(test)]
mod benchmarks {
    use super::*;
    use criterion::{criterion_group, criterion_main, Criterion};
    
    fn bench_job_creation(c: &mut Criterion) {
        c.bench_function("job_creation", |b| {
            b.iter(|| {
                Job::new("benchmark-job".to_string(), "echo test".to_string())
            });
        });
    }
    
    fn bench_job_queue_operations(c: &mut Criterion) {
        let mut queue = JobQueue::new();
        
        c.bench_function("queue_add_job", |b| {
            b.iter(|| {
                let job = Job::new("queue-job".to_string(), "echo test".to_string());
                queue.add_job(job).unwrap();
            });
        });
    }
    
    fn bench_job_parsing(c: &mut Criterion) {
        c.bench_function("parse_cron", |b| {
            b.iter(|| {
                Parser::parse_cron("0 18 * * *").unwrap();
            });
        });
    }
    
    criterion_group!(benches, bench_job_creation, bench_job_queue_operations, bench_job_parsing);
    criterion_main!(benches);
}
```

### Load Testing

```rust
#[tokio::test]
async fn test_concurrent_job_execution() {
    let temp_dir = tempdir().unwrap();
    let scheduler = Scheduler::new(temp_dir.path()).await.unwrap();
    
    // Create multiple jobs
    let mut job_ids = Vec::new();
    for i in 0..10 {
        let job = Job::new(format!("concurrent-job-{}", i), "echo".to_string())
            .with_args(vec![format!("job-{}", i)]);
        
        let job_id = scheduler.add_job(job).await.unwrap();
        job_ids.push(job_id);
    }
    
    // Wait for all jobs to complete
    tokio::time::sleep(Duration::from_secs(5)).await;
    
    // Verify all jobs completed
    for job_id in job_ids {
        let status = scheduler.get_job_status(&job_id).await.unwrap();
        assert!(matches!(status, JobStatus::Completed));
    }
}
```

## Test Utilities

### Test Helpers

```rust
#[cfg(test)]
mod test_helpers {
    use super::*;
    use tempfile::tempdir;
    
    pub async fn create_test_scheduler() -> Scheduler {
        let temp_dir = tempdir().unwrap();
        Scheduler::new(temp_dir.path()).await.unwrap()
    }
    
    pub fn create_test_job(name: &str, command: &str) -> Job {
        Job::new(name.to_string(), command.to_string())
    }
    
    pub async fn wait_for_job_completion(scheduler: &Scheduler, job_id: &JobId) {
        let mut attempts = 0;
        while attempts < 10 {
            let status = scheduler.get_job_status(job_id).await.unwrap();
            if matches!(status, JobStatus::Completed | JobStatus::Failed { .. }) {
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
            attempts += 1;
        }
    }
}
```

### Mock Components

```rust
#[cfg(test)]
mod mocks {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;
    
    mock! {
        pub JobExecutor {
            fn execute_job(&self, job: Job) -> Result<JobId, ExecutorError>;
            fn get_job_status(&self, job_id: &JobId) -> Result<JobStatus, ExecutorError>;
        }
    }
    
    impl MockJobExecutor {
        pub fn expect_execute_job(&mut self) -> mockall::predicate::MockPredicate {
            self.expect_execute_job()
                .times(1)
                .returning(|_| Ok("mock-job-id".to_string()))
        }
    }
}
```

## Running Tests

### Test Commands

```bash
# Run all tests
cargo test

# Run scheduler tests only
cargo test scheduler

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_job_creation

# Run integration tests
cargo test --test integration

# Run benchmarks
cargo bench

# Run tests with coverage
cargo tarpaulin --out Html
```

### Test Configuration

```toml
# Cargo.toml
[dev-dependencies]
tokio-test = "0.4"
mockall = "0.12"
tempfile = "3.8"
criterion = "0.5"

[[bench]]
name = "scheduler_benchmarks"
harness = false
```

## Continuous Integration

### GitHub Actions Example

```yaml
name: Scheduler Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, nightly]
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
    
    - name: Run tests
      run: cargo test --all-features
    
    - name: Run benchmarks
      run: cargo bench --no-run
    
    - name: Check formatting
      run: cargo fmt -- --check
    
    - name: Check clippy
      run: cargo clippy -- -D warnings
```

## Best Practices

### Test Organization

1. **Group related tests**: Use `mod` blocks to organize tests by component
2. **Use descriptive names**: Test names should clearly describe what they test
3. **Test one thing**: Each test should verify a single behavior
4. **Use setup/teardown**: Use `#[tokio::test]` for async tests and proper cleanup

### Test Data

1. **Use realistic data**: Test with data that resembles real usage
2. **Test edge cases**: Include boundary conditions and error cases
3. **Use constants**: Define test constants for consistency
4. **Clean up**: Ensure tests don't leave side effects

### Async Testing

1. **Use `#[tokio::test]`**: For async tests, use the tokio test macro
2. **Handle timeouts**: Use `tokio::time::timeout` for operations that might hang
3. **Wait appropriately**: Use `sleep` or polling for async operations
4. **Test cancellation**: Verify graceful shutdown behavior

### Cross-Platform Testing

1. **Test on all platforms**: Ensure compatibility with macOS, Linux, Windows
2. **Use platform-agnostic commands**: Test with commands that work everywhere
3. **Handle platform differences**: Account for different file paths, commands, etc.
4. **Test timezone handling**: Verify timezone-aware scheduling works correctly

## Debugging Tests

### Common Issues

1. **Async deadlocks**: Use proper shutdown mechanisms and timeouts
2. **File system issues**: Use `tempfile` for temporary directories
3. **Process cleanup**: Ensure background processes are properly terminated
4. **Resource leaks**: Monitor memory usage and file handles

### Debugging Tools

```bash
# Run with backtrace
RUST_BACKTRACE=1 cargo test

# Run with logging
RUST_LOG=debug cargo test

# Run specific test with output
cargo test test_name -- --nocapture

# Profile test execution
cargo test --release
```

This comprehensive testing guide ensures the scheduler is reliable, performant, and works correctly across all supported platforms. 
//! Job execution engine with thread pool and retry logic.
//! 
//! Provides cross-platform job execution with platform-appropriate
//! process management, resource limits, and error handling.

use crate::scheduler::job::{Job, JobId, JobResult, JobStatus, ResourceUsage};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio::time::{sleep, Duration};
use thiserror::Error;
use tracing::{debug, error, info, warn};

/// Errors that can occur during job execution.
#[derive(Debug, Error)]
pub enum ExecutorError {
    #[error("Job execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Process creation failed: {0}")]
    ProcessCreationFailed(String),
    
    #[error("Resource limit exceeded: {0}")]
    ResourceLimitExceeded(String),
    
    #[error("Job timeout: {0}")]
    JobTimeout(String),
    
    #[error("Retry limit exceeded: {0}")]
    RetryLimitExceeded(String),
    
    #[error("Invalid job configuration: {0}")]
    InvalidJob(String),
}

/// Job executor with thread pool and retry logic.
pub struct JobExecutor {
    /// Thread pool for job execution
    runtime: tokio::runtime::Runtime,
    /// Channel for job execution requests
    job_sender: mpsc::Sender<JobExecutionRequest>,
    /// Running jobs
    running_jobs: Arc<RwLock<HashMap<JobId, RunningJob>>>,
    /// Job results
    job_results: Arc<RwLock<HashMap<JobId, JobResult>>>,
}

/// Request to execute a job.
#[derive(Debug)]
struct JobExecutionRequest {
    job: Job,
    attempt: u32,
}

/// Information about a running job.
#[derive(Debug)]
struct RunningJob {
    job: Job,
    start_time: DateTime<Utc>,
    attempt: u32,
}

impl JobExecutor {
    /// Creates a new job executor.
    pub fn new() -> Self {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let (job_sender, job_receiver) = mpsc::channel(100);
        let running_jobs = Arc::new(RwLock::new(HashMap::new()));
        let job_results = Arc::new(RwLock::new(HashMap::new()));
        
        let executor = JobExecutor {
            runtime,
            job_sender,
            running_jobs,
            job_results,
        };
        
        // Start the job processing loop
        let running_jobs_clone = executor.running_jobs.clone();
        let job_results_clone = executor.job_results.clone();
        let job_sender_clone = executor.job_sender.clone();
        
        executor.runtime.spawn(async move {
            Self::process_jobs(job_receiver, job_sender_clone, running_jobs_clone, job_results_clone).await;
        });
        
        executor
    }
    
    /// Starts the executor.
    pub async fn start(&self) -> Result<(), ExecutorError> {
        info!("Job executor started");
        Ok(())
    }
    
    /// Stops the executor.
    pub async fn stop(&self) -> Result<(), ExecutorError> {
        info!("Job executor stopped");
        Ok(())
    }
    
    /// Executes a job.
    pub async fn execute_job(&self, job: Job) -> Result<JobId, ExecutorError> {
        let job_id = job.id.clone();
        
        // Validate job
        self.validate_job(&job)?;
        
        // Send job for execution
        let request = JobExecutionRequest {
            job,
            attempt: 1,
        };
        
        self.job_sender
            .send(request)
            .await
            .map_err(|e| ExecutorError::ExecutionFailed(e.to_string()))?;
        
        Ok(job_id)
    }
    
    /// Gets the status of a job.
    pub async fn get_job_status(&self, job_id: &JobId) -> Result<JobStatus, ExecutorError> {
        // Check if job is running
        {
            let running_jobs = self.running_jobs.read().await;
            if running_jobs.contains_key(job_id) {
                return Ok(JobStatus::Running);
            }
        }
        
        // Check job results
        {
            let job_results = self.job_results.read().await;
            if let Some(result) = job_results.get(job_id) {
                return Ok(result.status.clone());
            }
        }
        
        Ok(JobStatus::Scheduled)
    }
    
    /// Gets job results.
    pub async fn get_job_result(&self, job_id: &JobId) -> Result<Option<JobResult>, ExecutorError> {
        let job_results = self.job_results.read().await;
        Ok(job_results.get(job_id).cloned())
    }
    
    /// Cancels a running job.
    pub async fn cancel_job(&self, job_id: &JobId) -> Result<(), ExecutorError> {
        let mut running_jobs = self.running_jobs.write().await;
        
        if let Some(running_job) = running_jobs.remove(job_id) {
            // TODO: Implement actual process termination
            warn!("Cancelled job: {}", job_id);
            
            // Add cancelled result
            let result = JobResult {
                job_id: job_id.clone(),
                started_at: running_job.start_time,
                ended_at: Some(Utc::now()),
                exit_code: None,
                stdout: String::new(),
                stderr: "Job cancelled".to_string(),
                status: JobStatus::Cancelled,
                resource_usage: None,
            };
            
            let mut job_results = self.job_results.write().await;
            job_results.insert(job_id.clone(), result);
        }
        
        Ok(())
    }
    
    /// Validates a job configuration.
    fn validate_job(&self, job: &Job) -> Result<(), ExecutorError> {
        if job.command.is_empty() {
            return Err(ExecutorError::InvalidJob("Command cannot be empty".to_string()));
        }
        
        if !job.enabled {
            return Err(ExecutorError::InvalidJob("Job is disabled".to_string()));
        }
        
        Ok(())
    }
    
    /// Processes jobs from the channel.
    async fn process_jobs(
        mut job_receiver: mpsc::Receiver<JobExecutionRequest>,
        job_sender: mpsc::Sender<JobExecutionRequest>,
        running_jobs: Arc<RwLock<HashMap<JobId, RunningJob>>>,
        job_results: Arc<RwLock<HashMap<JobId, JobResult>>>,
    ) {
        while let Some(request) = job_receiver.recv().await {
            let job_id = request.job.id.clone();
            
            // Add to running jobs
            {
                let mut jobs = running_jobs.write().await;
                jobs.insert(job_id.clone(), RunningJob {
                    job: request.job.clone(),
                    start_time: Utc::now(),
                    attempt: request.attempt,
                });
            }
            
            // Execute job
            let job = request.job.clone();
            let result = Self::execute_single_job(job.clone(), request.attempt).await;
            
            // Remove from running jobs
            {
                let mut jobs = running_jobs.write().await;
                jobs.remove(&job_id);
            }
            
            // Store result
            {
                let mut results = job_results.write().await;
                results.insert(job_id.clone(), result.clone());
            }
            
            // Handle retry logic
            if let JobStatus::Failed { error } = &result.status {
                if request.attempt < job.retry_policy.max_attempts {
                    let delay = Self::calculate_retry_delay(&job, request.attempt);
                    
                    info!("Job {} failed, retrying in {} seconds (attempt {}/{})", 
                          job_id, delay.as_secs(), request.attempt + 1, job.retry_policy.max_attempts);
                    
                    sleep(delay).await;
                    
                    let retry_request = JobExecutionRequest {
                        job: job,
                        attempt: request.attempt + 1,
                    };
                    
                    // Re-queue for retry
                    if let Err(e) = job_sender.send(retry_request).await {
                        warn!("Failed to re-queue job {} for retry: {}", job_id, e);
                    }
                } else {
                    error!("Job {} failed after {} attempts: {}", 
                           job_id, request.attempt, error);
                }
            }
        }
    }
    
    /// Executes a single job.
    async fn execute_single_job(job: Job, attempt: u32) -> JobResult {
        let job_id = job.id.clone();
        let start_time = Utc::now();
        
        info!("Executing job {} (attempt {})", job_id, attempt);
        
        // Build command
        let mut command = Command::new(&job.command);
        
        // Add arguments
        for arg in &job.args {
            command.arg(arg);
        }
        
        // Set working directory
        if let Some(working_dir) = &job.working_dir {
            command.current_dir(working_dir);
        }
        
        // Set environment variables
        for (key, value) in &job.env {
            command.env(key, value);
        }
        
        // Capture output
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        
        // Execute command
        let result = command.output();
        
        let end_time = Utc::now();
        let duration = end_time.signed_duration_since(start_time);
        
        match result {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                
                let status = if output.status.success() {
                    JobStatus::Completed
                } else {
                    JobStatus::Failed {
                        error: format!("Exit code: {}", output.status.code().unwrap_or(-1))
                    }
                };
                
                let resource_usage = Some(ResourceUsage {
                    cpu_percent: 0.0, // TODO: Implement CPU monitoring
                    memory_mb: 0,      // TODO: Implement memory monitoring
                    duration_seconds: duration.num_seconds() as u64,
                    disk_io_mb: 0,     // TODO: Implement disk I/O monitoring
                });
                
                JobResult {
                    job_id,
                    started_at: start_time,
                    ended_at: Some(end_time),
                    exit_code: output.status.code(),
                    stdout,
                    stderr,
                    status,
                    resource_usage,
                }
            }
            Err(e) => {
                let status = JobStatus::Failed {
                    error: e.to_string()
                };
                
                JobResult {
                    job_id,
                    started_at: start_time,
                    ended_at: Some(end_time),
                    exit_code: None,
                    stdout: String::new(),
                    stderr: e.to_string(),
                    status,
                    resource_usage: None,
                }
            }
        }
    }
    
    /// Calculates retry delay with exponential backoff.
    fn calculate_retry_delay(job: &Job, attempt: u32) -> Duration {
        let base_delay = Duration::from_secs(job.retry_policy.delay);
        
        if job.retry_policy.exponential_backoff {
            let exponential_delay = base_delay * 2_u32.pow(attempt - 1);
            
            if let Some(max_delay) = job.retry_policy.max_delay {
                let max_delay = Duration::from_secs(max_delay);
                std::cmp::min(exponential_delay, max_delay)
            } else {
                exponential_delay
            }
        } else {
            base_delay
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::job::Job;
    
    #[tokio::test]
    async fn test_execute_simple_job() {
        let executor = JobExecutor::new();
        
        let job = Job::new("test-job".to_string(), "echo".to_string())
            .with_args(vec!["hello".to_string()]);
        
        let job_id = executor.execute_job(job).await.unwrap();
        
        // Wait a bit for execution
        sleep(Duration::from_millis(100)).await;
        
        let status = executor.get_job_status(&job_id).await.unwrap();
        assert!(matches!(status, JobStatus::Completed));
    }
    
    #[tokio::test]
    async fn test_execute_failing_job() {
        let executor = JobExecutor::new();
        
        let job = Job::new("test-job".to_string(), "nonexistent-command".to_string());
        
        let job_id = executor.execute_job(job).await.unwrap();
        
        // Wait a bit for execution
        sleep(Duration::from_millis(100)).await;
        
        let status = executor.get_job_status(&job_id).await.unwrap();
        assert!(matches!(status, JobStatus::Failed { .. }));
    }
    
    #[test]
    fn test_validate_job() {
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
//! Core scheduler implementation for the Rae agent.
//! 
//! This module provides cross-platform job scheduling with platform-specific adapters.
//! Supports cron-like syntax, timezone-aware scheduling, and platform-appropriate
//! background process management.

pub mod job;
pub mod parser;
pub mod queue;
pub mod persistence;
pub mod executor;
pub mod monitor;

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::scheduler::job::{Job, JobId, JobStatus};
use crate::scheduler::queue::JobQueue;
use crate::scheduler::persistence::JobPersistence;
use crate::scheduler::executor::JobExecutor;
use crate::scheduler::monitor::JobMonitor;

/// Main scheduler that manages all scheduled jobs and automation triggers.
/// 
/// Provides a cross-platform interface for job scheduling with platform-specific
/// adaptations for file monitoring, logging, and background process management.
pub struct Scheduler {
    queue: Arc<RwLock<JobQueue>>,
    persistence: Arc<JobPersistence>,
    executor: Arc<JobExecutor>,
    monitor: Arc<JobMonitor>,
}

impl Scheduler {
    /// Creates a new scheduler instance.
    pub async fn new() -> Result<Self, SchedulerError> {
        let persistence = Arc::new(JobPersistence::new()?);
        let queue = Arc::new(RwLock::new(JobQueue::new()));
        let executor = Arc::new(JobExecutor::new());
        let monitor = Arc::new(JobMonitor::new());
        
        Ok(Scheduler {
            queue,
            persistence,
            executor,
            monitor,
        })
    }
    
    /// Adds a new job to the scheduler.
    pub async fn add_job(&self, job: Job) -> Result<JobId, SchedulerError> {
        let job_id = job.id.clone();
        
        // Validate job configuration
        self.validate_job(&job)?;
        
        // Store job configuration
        self.persistence.save_job(&job).await?;
        
        // Add to queue
        {
            let mut queue = self.queue.write().await;
            queue.add_job(job)?;
        }
        
        // Start monitoring
        self.monitor.track_job(job_id.clone()).await?;
        
        Ok(job_id)
    }
    
    /// Removes a job from the scheduler.
    pub async fn remove_job(&self, job_id: &JobId) -> Result<(), SchedulerError> {
        // Remove from queue
        {
            let mut queue = self.queue.write().await;
            queue.remove_job(job_id)?;
        }
        
        // Remove from persistence
        self.persistence.delete_job(job_id).await?;
        
        // Stop monitoring
        self.monitor.untrack_job(job_id).await?;
        
        Ok(())
    }
    
    /// Gets the status of a specific job.
    pub async fn get_job_status(&self, job_id: &JobId) -> Result<JobStatus, SchedulerError> {
        self.monitor.get_job_status(job_id).await.map_err(|e| SchedulerError::MonitorError(e))
    }
    
    /// Lists all jobs with their current status.
    pub async fn list_jobs(&self) -> Result<Vec<JobInfo>, SchedulerError> {
        let jobs = self.persistence.list_jobs().await?;
        let mut job_infos = Vec::new();
        
        for job in jobs {
            let status = self.monitor.get_job_status(&job.id).await?;
            job_infos.push(JobInfo {
                job,
                status,
            });
        }
        
        Ok(job_infos)
    }
    
    /// Validates a job configuration.
    fn validate_job(&self, job: &Job) -> Result<(), SchedulerError> {
        // Validate cron expression if present
        if let Some(cron_expr) = &job.schedule.cron {
            cron::Schedule::from_str(cron_expr)
                .map_err(|e| SchedulerError::InvalidCronExpression(e.to_string()))?;
        }
        
        // Validate command exists
        if job.command.is_empty() {
            return Err(SchedulerError::InvalidJob("Command cannot be empty".to_string()));
        }
        
        Ok(())
    }
    
    /// Starts the scheduler background processing.
    pub async fn start(&self) -> Result<(), SchedulerError> {
        // Start the executor
        self.executor.start().await?;
        
        // Start the monitor
        self.monitor.start().await?;
        
        // Load persisted jobs
        self.load_persisted_jobs().await?;
        
        Ok(())
    }
    
    /// Stops the scheduler background processing.
    pub async fn stop(&self) -> Result<(), SchedulerError> {
        // Stop the executor
        self.executor.stop().await?;
        
        // Stop the monitor
        self.monitor.stop().await?;
        
        Ok(())
    }
    
    /// Loads persisted jobs from storage.
    async fn load_persisted_jobs(&self) -> Result<(), SchedulerError> {
        let jobs = self.persistence.list_jobs().await?;
        
        for job in jobs {
            let mut queue = self.queue.write().await;
            queue.add_job(job)?;
        }
        
        Ok(())
    }
}

/// Information about a job including its status.
#[derive(Debug, Clone)]
pub struct JobInfo {
    pub job: Job,
    pub status: JobStatus,
}

/// Errors that can occur in the scheduler.
#[derive(Debug, thiserror::Error)]
pub enum SchedulerError {
    #[error("Invalid cron expression: {0}")]
    InvalidCronExpression(String),
    
    #[error("Invalid job configuration: {0}")]
    InvalidJob(String),
    
    #[error("Job not found: {0}")]
    JobNotFound(String),
    
    #[error("Persistence error: {0}")]
    PersistenceError(#[from] persistence::PersistenceError),
    
    #[error("Queue error: {0}")]
    QueueError(#[from] queue::QueueError),
    
    #[error("Executor error: {0}")]
    ExecutorError(#[from] executor::ExecutorError),
    
    #[error("Monitor error: {0}")]
    MonitorError(#[from] monitor::MonitorError),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

use std::str::FromStr; 
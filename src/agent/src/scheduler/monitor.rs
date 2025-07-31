//! Job monitoring and health tracking.
//! 
//! Provides cross-platform job status monitoring with platform-appropriate
//! health checks, notifications, and metrics collection.

use crate::scheduler::job::{JobId, JobStatus};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use thiserror::Error;
use tracing::{debug, info, warn};

/// Errors that can occur in the job monitor.
#[derive(Debug, Error)]
pub enum MonitorError {
    #[error("Job not found: {0}")]
    JobNotFound(String),
    
    #[error("Monitoring failed: {0}")]
    MonitoringFailed(String),
    
    #[error("Health check failed: {0}")]
    HealthCheckFailed(String),
}

/// Job monitoring statistics.
#[derive(Debug, Clone)]
pub struct MonitorStats {
    pub total_jobs: usize,
    pub running_jobs: usize,
    pub completed_jobs: usize,
    pub failed_jobs: usize,
    pub cancelled_jobs: usize,
    pub average_execution_time: f64,
    pub success_rate: f64,
}

impl Default for MonitorStats {
    fn default() -> Self {
        MonitorStats {
            total_jobs: 0,
            running_jobs: 0,
            completed_jobs: 0,
            failed_jobs: 0,
            cancelled_jobs: 0,
            average_execution_time: 0.0,
            success_rate: 0.0,
        }
    }
}

/// Job health information.
#[derive(Debug, Clone)]
pub struct JobHealth {
    pub job_id: JobId,
    pub status: JobStatus,
    pub last_check: DateTime<Utc>,
    pub execution_count: u32,
    pub failure_count: u32,
    pub average_duration: f64,
    pub last_execution: Option<DateTime<Utc>>,
}

/// Job monitor for tracking status and health.
pub struct JobMonitor {
    /// Tracked jobs with their health information
    tracked_jobs: Arc<RwLock<HashMap<JobId, JobHealth>>>,
    /// Monitoring statistics
    stats: Arc<RwLock<MonitorStats>>,
    /// Health check interval
    health_check_interval: Duration,
    /// Whether monitoring is active
    is_active: Arc<RwLock<bool>>,
}

impl JobMonitor {
    /// Creates a new job monitor.
    pub fn new() -> Self {
        JobMonitor {
            tracked_jobs: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(MonitorStats::default())),
            health_check_interval: Duration::from_secs(30),
            is_active: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Starts the job monitor.
    pub async fn start(&self) -> Result<(), MonitorError> {
        let mut is_active = self.is_active.write().await;
        *is_active = true;
        
        let tracked_jobs = self.tracked_jobs.clone();
        let stats = self.stats.clone();
        let is_active_clone = self.is_active.clone();
        let interval_duration = self.health_check_interval;
        
        // Start monitoring loop
        tokio::spawn(async move {
            let mut interval = interval(interval_duration);
            
            while *is_active_clone.read().await {
                interval.tick().await;
                
                // Perform health checks
                Self::perform_health_checks(&tracked_jobs, &stats).await;
            }
        });
        
        info!("Job monitor started");
        Ok(())
    }
    
    /// Stops the job monitor.
    pub async fn stop(&self) -> Result<(), MonitorError> {
        let mut is_active = self.is_active.write().await;
        *is_active = false;
        
        info!("Job monitor stopped");
        Ok(())
    }
    
    /// Tracks a job for monitoring.
    pub async fn track_job(&self, job_id: JobId) -> Result<(), MonitorError> {
        let mut tracked_jobs = self.tracked_jobs.write().await;
        
        let health = JobHealth {
            job_id: job_id.clone(),
            status: JobStatus::Scheduled,
            last_check: Utc::now(),
            execution_count: 0,
            failure_count: 0,
            average_duration: 0.0,
            last_execution: None,
        };
        
        tracked_jobs.insert(job_id.clone(), health);
        
        // Update statistics
        self.update_stats().await;
        
        debug!("Started tracking job: {}", job_id);
        Ok(())
    }
    
    /// Stops tracking a job.
    pub async fn untrack_job(&self, job_id: &JobId) -> Result<(), MonitorError> {
        let mut tracked_jobs = self.tracked_jobs.write().await;
        
        if tracked_jobs.remove(job_id).is_some() {
            // Update statistics
            self.update_stats().await;
            
            debug!("Stopped tracking job: {}", job_id);
        }
        
        Ok(())
    }
    
    /// Updates the status of a tracked job.
    pub async fn update_job_status(&self, job_id: &JobId, status: JobStatus) -> Result<(), MonitorError> {
        let mut tracked_jobs = self.tracked_jobs.write().await;
        
        if let Some(health) = tracked_jobs.get_mut(job_id) {
            health.status = status.clone();
            health.last_check = Utc::now();
            
            // Update execution statistics
            match status {
                JobStatus::Completed => {
                    health.execution_count += 1;
                    health.last_execution = Some(Utc::now());
                }
                JobStatus::Failed { .. } => {
                    health.failure_count += 1;
                }
                _ => {}
            }
            
            // Update statistics
            self.update_stats().await;
            
            debug!("Updated job {} status to {:?}", job_id, status);
        }
        
        Ok(())
    }
    
    /// Gets the status of a tracked job.
    pub async fn get_job_status(&self, job_id: &JobId) -> Result<JobStatus, MonitorError> {
        let tracked_jobs = self.tracked_jobs.read().await;
        
        if let Some(health) = tracked_jobs.get(job_id) {
            Ok(health.status.clone())
        } else {
            Err(MonitorError::JobNotFound(job_id.clone()))
        }
    }
    
    /// Gets health information for a tracked job.
    pub async fn get_job_health(&self, job_id: &JobId) -> Result<JobHealth, MonitorError> {
        let tracked_jobs = self.tracked_jobs.read().await;
        
        if let Some(health) = tracked_jobs.get(job_id) {
            Ok(health.clone())
        } else {
            Err(MonitorError::JobNotFound(job_id.clone()))
        }
    }
    
    /// Gets all tracked jobs.
    pub async fn get_tracked_jobs(&self) -> Vec<JobHealth> {
        let tracked_jobs = self.tracked_jobs.read().await;
        tracked_jobs.values().cloned().collect()
    }
    
    /// Gets monitoring statistics.
    pub async fn get_stats(&self) -> MonitorStats {
        let stats = self.stats.read().await;
        stats.clone()
    }
    
    /// Performs health checks on tracked jobs.
    async fn perform_health_checks(
        tracked_jobs: &Arc<RwLock<HashMap<JobId, JobHealth>>>,
        stats: &Arc<RwLock<MonitorStats>>,
    ) {
        let mut jobs = tracked_jobs.write().await;
        let now = Utc::now();
        
        for (job_id, health) in jobs.iter_mut() {
            // Update last check time
            health.last_check = now;
            
            // Check for stuck jobs (running for too long)
            if let JobStatus::Running = health.status {
                if let Some(last_execution) = health.last_execution {
                    let duration = now.signed_duration_since(last_execution);
                    if duration.num_minutes() > 60 {
                        warn!("Job {} has been running for {} minutes", 
                              job_id, duration.num_minutes());
                    }
                }
            }
            
            // Check for jobs with high failure rates
            if health.execution_count > 0 {
                let failure_rate = health.failure_count as f64 / health.execution_count as f64;
                if failure_rate > 0.5 {
                    warn!("Job {} has high failure rate: {:.1}%", 
                          job_id, failure_rate * 100.0);
                }
            }
        }
        
        // Update statistics
        Self::update_stats_internal(tracked_jobs, stats).await;
    }
    
    /// Updates monitoring statistics.
    async fn update_stats(&self) {
        Self::update_stats_internal(&self.tracked_jobs, &self.stats).await;
    }
    
    /// Updates statistics internally.
    async fn update_stats_internal(
        tracked_jobs: &Arc<RwLock<HashMap<JobId, JobHealth>>>,
        stats: &Arc<RwLock<MonitorStats>>,
    ) {
        let jobs = tracked_jobs.read().await;
        let mut new_stats = MonitorStats::default();
        
        let mut total_duration = 0.0;
        let mut total_executions = 0;
        let mut total_failures = 0;
        
        for health in jobs.values() {
            new_stats.total_jobs += 1;
            
            match health.status {
                JobStatus::Running => new_stats.running_jobs += 1,
                JobStatus::Completed => {
                    new_stats.completed_jobs += 1;
                    total_executions += health.execution_count;
                }
                JobStatus::Failed { .. } => {
                    new_stats.failed_jobs += 1;
                    total_failures += health.failure_count;
                }
                JobStatus::Cancelled => new_stats.cancelled_jobs += 1,
                _ => {}
            }
            
            total_duration += health.average_duration;
        }
        
        // Calculate averages
        if new_stats.total_jobs > 0 {
            new_stats.average_execution_time = total_duration / new_stats.total_jobs as f64;
        }
        
        if total_executions > 0 {
            new_stats.success_rate = (total_executions - total_failures) as f64 / total_executions as f64;
        }
        
        // Update stats
        let mut stats_write = stats.write().await;
        *stats_write = new_stats;
    }
    
    /// Sets the health check interval.
    pub fn set_health_check_interval(&mut self, interval: Duration) {
        self.health_check_interval = interval;
    }
    
    /// Gets the health check interval.
    pub fn get_health_check_interval(&self) -> Duration {
        self.health_check_interval
    }
    
    /// Checks if the monitor is active.
    pub async fn is_active(&self) -> bool {
        let is_active = self.is_active.read().await;
        *is_active
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::job::JobStatus;
    
    #[tokio::test]
    async fn test_track_and_untrack_job() {
        let monitor = JobMonitor::new();
        
        let job_id = "test-job".to_string();
        
        // Track job
        assert!(monitor.track_job(job_id.clone()).await.is_ok());
        
        // Check if job is tracked
        let tracked_jobs = monitor.get_tracked_jobs().await;
        assert_eq!(tracked_jobs.len(), 1);
        assert_eq!(tracked_jobs[0].job_id, job_id);
        
        // Untrack job
        assert!(monitor.untrack_job(&job_id).await.is_ok());
        
        // Check if job is no longer tracked
        let tracked_jobs = monitor.get_tracked_jobs().await;
        assert_eq!(tracked_jobs.len(), 0);
    }
    
    #[tokio::test]
    async fn test_update_job_status() {
        let monitor = JobMonitor::new();
        
        let job_id = "test-job".to_string();
        
        // Track job
        monitor.track_job(job_id.clone()).await.unwrap();
        
        // Update status
        let new_status = JobStatus::Running;
        assert!(monitor.update_job_status(&job_id, new_status.clone()).await.is_ok());
        
        // Check status
        let status = monitor.get_job_status(&job_id).await.unwrap();
        assert!(matches!(status, JobStatus::Running));
    }
    
    #[tokio::test]
    async fn test_get_job_health() {
        let monitor = JobMonitor::new();
        
        let job_id = "test-job".to_string();
        
        // Track job
        monitor.track_job(job_id.clone()).await.unwrap();
        
        // Get health
        let health = monitor.get_job_health(&job_id).await.unwrap();
        assert_eq!(health.job_id, job_id);
        assert_eq!(health.execution_count, 0);
        assert_eq!(health.failure_count, 0);
    }
    
    #[tokio::test]
    async fn test_get_stats() {
        let monitor = JobMonitor::new();
        
        let job_id = "test-job".to_string();
        
        // Track job
        monitor.track_job(job_id.clone()).await.unwrap();
        
        // Get stats
        let stats = monitor.get_stats().await;
        assert_eq!(stats.total_jobs, 1);
        assert_eq!(stats.running_jobs, 0);
    }
    
    #[tokio::test]
    async fn test_start_and_stop_monitor() {
        let monitor = JobMonitor::new();
        
        // Start monitor
        assert!(monitor.start().await.is_ok());
        assert!(monitor.is_active().await);
        
        // Stop monitor
        assert!(monitor.stop().await.is_ok());
        assert!(!monitor.is_active().await);
    }
} 
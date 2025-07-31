//! Job queue management with priority-based scheduling.
//! 
//! Provides cross-platform job queuing with priority management,
//! time-based scheduling, and platform-appropriate resource limits.

use crate::scheduler::job::{Job, JobId, Priority};
use chrono::{DateTime, Utc};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::str::FromStr;
use thiserror::Error;

/// Errors that can occur in the job queue.
#[derive(Debug, Error)]
pub enum QueueError {
    #[error("Job already exists: {0}")]
    JobAlreadyExists(String),
    
    #[error("Job not found: {0}")]
    JobNotFound(String),
    
    #[error("Invalid job configuration: {0}")]
    InvalidJob(String),
}

/// A job entry in the queue with scheduling information.
#[derive(Debug, Clone)]
pub struct QueuedJob {
    pub job: Job,
    pub next_execution: Option<DateTime<Utc>>,
    pub priority: Priority,
    pub added_at: DateTime<Utc>,
}

impl PartialEq for QueuedJob {
    fn eq(&self, other: &Self) -> bool {
        self.job.id == other.job.id
    }
}

impl Eq for QueuedJob {}

impl PartialOrd for QueuedJob {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueuedJob {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority jobs come first
        match self.priority.cmp(&other.priority) {
            Ordering::Equal => {
                // For same priority, earlier execution time comes first
                match (self.next_execution, other.next_execution) {
                    (Some(self_time), Some(other_time)) => self_time.cmp(&other_time),
                    (Some(_), None) => Ordering::Less,
                    (None, Some(_)) => Ordering::Greater,
                    (None, None) => Ordering::Equal,
                }
            }
            other => other,
        }
    }
}

/// Job queue with priority-based scheduling.
pub struct JobQueue {
    /// Priority queue of jobs ordered by priority and execution time
    jobs: BinaryHeap<QueuedJob>,
    /// Index of jobs by ID for fast lookup
    job_index: HashMap<JobId, QueuedJob>,
    /// Statistics about the queue
    stats: QueueStats,
}

/// Statistics about the job queue.
#[derive(Debug, Clone)]
pub struct QueueStats {
    pub total_jobs: usize,
    pub scheduled_jobs: usize,
    pub running_jobs: usize,
    pub completed_jobs: usize,
    pub failed_jobs: usize,
    pub average_wait_time: f64,
}

impl Default for QueueStats {
    fn default() -> Self {
        QueueStats {
            total_jobs: 0,
            scheduled_jobs: 0,
            running_jobs: 0,
            completed_jobs: 0,
            failed_jobs: 0,
            average_wait_time: 0.0,
        }
    }
}

impl JobQueue {
    /// Creates a new job queue.
    pub fn new() -> Self {
        JobQueue {
            jobs: BinaryHeap::new(),
            job_index: HashMap::new(),
            stats: QueueStats::default(),
        }
    }
    
    /// Adds a job to the queue.
    pub fn add_job(&mut self, job: Job) -> Result<(), QueueError> {
        // Check if job already exists
        if self.job_index.contains_key(&job.id) {
            return Err(QueueError::JobAlreadyExists(job.id.clone()));
        }
        
        // Calculate next execution time
        let next_execution = self.calculate_next_execution(&job);
        
        // Create queued job
        let queued_job = QueuedJob {
            job: job.clone(),
            next_execution,
            priority: job.priority,
            added_at: Utc::now(),
        };
        
        // Add to queue and index
        self.jobs.push(queued_job.clone());
        self.job_index.insert(job.id.clone(), queued_job);
        
        // Update statistics
        self.stats.total_jobs += 1;
        self.stats.scheduled_jobs += 1;
        
        Ok(())
    }
    
    /// Removes a job from the queue.
    pub fn remove_job(&mut self, job_id: &JobId) -> Result<(), QueueError> {
        if !self.job_index.contains_key(job_id) {
            return Err(QueueError::JobNotFound(job_id.clone()));
        }
        
        // Remove from index
        self.job_index.remove(job_id);
        
        // Rebuild queue without the removed job
        self.rebuild_queue();
        
        // Update statistics
        self.stats.total_jobs = self.job_index.len();
        self.stats.scheduled_jobs = self.jobs.len();
        
        Ok(())
    }
    
    /// Gets the next job to execute.
    pub fn get_next_job(&mut self) -> Option<Job> {
        let now = Utc::now();
        
        // Find the next job that should be executed
        while let Some(queued_job) = self.jobs.peek() {
            if let Some(next_execution) = queued_job.next_execution {
                if next_execution <= now {
                    // This job should be executed now
                    let job = queued_job.job.clone();
                    
                    // Remove from queue
                    self.jobs.pop();
                    self.job_index.remove(&job.id);
                    
                    // Update statistics
                    self.stats.scheduled_jobs = self.jobs.len();
                    
                    return Some(job);
                } else {
                    // Job is scheduled for the future
                    break;
                }
            } else {
                // Job has no next execution time (event/pattern based)
                let job = queued_job.job.clone();
                
                // Remove from queue
                self.jobs.pop();
                self.job_index.remove(&job.id);
                
                // Update statistics
                self.stats.scheduled_jobs = self.jobs.len();
                
                return Some(job);
            }
        }
        
        None
    }
    
    /// Gets a job by ID.
    pub fn get_job(&self, job_id: &JobId) -> Option<&Job> {
        self.job_index.get(job_id).map(|qj| &qj.job)
    }
    
    /// Lists all jobs in the queue.
    pub fn list_jobs(&self) -> Vec<&Job> {
        self.job_index.values().map(|qj| &qj.job).collect()
    }
    
    /// Gets jobs that should be executed now.
    pub fn get_due_jobs(&self) -> Vec<&Job> {
        let now = Utc::now();
        self.job_index
            .values()
            .filter(|qj| {
                if let Some(next_execution) = qj.next_execution {
                    next_execution <= now
                } else {
                    // Event/pattern based jobs are always considered due
                    true
                }
            })
            .map(|qj| &qj.job)
            .collect()
    }
    
    /// Updates a job in the queue.
    pub fn update_job(&mut self, job: Job) -> Result<(), QueueError> {
        // Remove existing job
        self.remove_job(&job.id)?;
        
        // Add updated job
        self.add_job(job)
    }
    
    /// Gets queue statistics.
    pub fn get_stats(&self) -> QueueStats {
        self.stats.clone()
    }
    
    /// Calculates the next execution time for a job.
    fn calculate_next_execution(&self, job: &Job) -> Option<DateTime<Utc>> {
        if !job.enabled {
            return None;
        }
        
        let now = Utc::now();
        
        // Check cron schedule
        if let Some(cron_expr) = &job.schedule.cron {
            if let Ok(schedule) = cron::Schedule::from_str(cron_expr) {
                return schedule.after(&now).next();
            }
        }
        
        // Check one-time schedule
        if let Some(at) = job.schedule.at {
            if at > now {
                return Some(at);
            }
        }
        
        // Event and pattern triggers don't have predictable next execution times
        None
    }
    
    /// Rebuilds the queue after modifications.
    fn rebuild_queue(&mut self) {
        self.jobs.clear();
        for queued_job in self.job_index.values() {
            self.jobs.push(queued_job.clone());
        }
    }
    
    /// Clears all jobs from the queue.
    pub fn clear(&mut self) {
        self.jobs.clear();
        self.job_index.clear();
        self.stats = QueueStats::default();
    }
    
    /// Gets the number of jobs in the queue.
    pub fn len(&self) -> usize {
        self.job_index.len()
    }
    
    /// Checks if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.job_index.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::job::{Job, Schedule, Priority};
    
    fn create_test_job(id: &str, priority: Priority) -> Job {
        Job::new(id.to_string(), "echo".to_string())
            .with_priority(priority)
            .with_cron("0 18 * * *".to_string())
    }
    
    #[test]
    fn test_add_job() {
        let mut queue = JobQueue::new();
        let job = create_test_job("test1", Priority::Normal);
        
        assert!(queue.add_job(job).is_ok());
        assert_eq!(queue.len(), 1);
    }
    
    #[test]
    fn test_add_duplicate_job() {
        let mut queue = JobQueue::new();
        let job = create_test_job("test1", Priority::Normal);
        
        assert!(queue.add_job(job.clone()).is_ok());
        assert!(queue.add_job(job).is_err());
    }
    
    #[test]
    fn test_remove_job() {
        let mut queue = JobQueue::new();
        let job = create_test_job("test1", Priority::Normal);
        
        assert!(queue.add_job(job.clone()).is_ok());
        assert!(queue.remove_job(&job.id).is_ok());
        assert_eq!(queue.len(), 0);
    }
    
    #[test]
    fn test_remove_nonexistent_job() {
        let mut queue = JobQueue::new();
        assert!(queue.remove_job(&"nonexistent".to_string()).is_err());
    }
    
    #[test]
    fn test_priority_ordering() {
        let mut queue = JobQueue::new();
        
        let low_job = create_test_job("low", Priority::Low);
        let high_job = create_test_job("high", Priority::High);
        let normal_job = create_test_job("normal", Priority::Normal);
        
        queue.add_job(low_job).unwrap();
        queue.add_job(high_job).unwrap();
        queue.add_job(normal_job).unwrap();
        
        // High priority job should come first
        let next_job = queue.get_next_job();
        assert!(next_job.is_some());
        assert_eq!(next_job.unwrap().priority, Priority::High);
    }
    
    #[test]
    fn test_get_job() {
        let mut queue = JobQueue::new();
        let job = create_test_job("test1", Priority::Normal);
        
        queue.add_job(job.clone()).unwrap();
        
        let retrieved_job = queue.get_job(&job.id);
        assert!(retrieved_job.is_some());
        assert_eq!(retrieved_job.unwrap().id, job.id);
    }
    
    #[test]
    fn test_clear_queue() {
        let mut queue = JobQueue::new();
        let job1 = create_test_job("test1", Priority::Normal);
        let job2 = create_test_job("test2", Priority::High);
        
        queue.add_job(job1).unwrap();
        queue.add_job(job2).unwrap();
        
        assert_eq!(queue.len(), 2);
        queue.clear();
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
    }
} 
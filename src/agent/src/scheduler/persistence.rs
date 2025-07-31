//! Job persistence layer for storing scheduled tasks.
//! 
//! Provides cross-platform JSON-based job storage with platform-appropriate
//! file system operations and error handling.

use crate::scheduler::job::Job;
use crate::scheduler::job::JobId;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs as tokio_fs;
use tokio::io::AsyncWriteExt;

/// Errors that can occur in the persistence layer.
#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Job not found: {0}")]
    JobNotFound(String),
    
    #[error("Invalid job data: {0}")]
    InvalidJobData(String),
    
    #[error("Storage directory error: {0}")]
    StorageDirectoryError(String),
}

/// Job persistence manager for storing and retrieving jobs.
pub struct JobPersistence {
    /// Directory where job files are stored
    storage_dir: PathBuf,
    /// Cache of loaded jobs
    job_cache: HashMap<JobId, Job>,
}

impl JobPersistence {
    /// Creates a new job persistence manager.
    pub fn new() -> Result<Self, PersistenceError> {
        let storage_dir = Self::get_storage_dir()?;
        
        // Create storage directory if it doesn't exist
        if !storage_dir.exists() {
            fs::create_dir_all(&storage_dir)?;
        }
        
        Ok(JobPersistence {
            storage_dir,
            job_cache: HashMap::new(),
        })
    }
    
    /// Gets the storage directory for jobs.
    fn get_storage_dir() -> Result<PathBuf, PersistenceError> {
        let mut path = dirs::data_local_dir()
            .ok_or_else(|| PersistenceError::StorageDirectoryError(
                "Could not determine local data directory".to_string()
            ))?;
        
        path.push("rae");
        path.push("scheduler");
        path.push("jobs");
        
        Ok(path)
    }
    
    /// Gets the file path for a job.
    fn get_job_file_path(&self, job_id: &JobId) -> PathBuf {
        self.storage_dir.join(format!("{}.json", job_id))
    }
    
    /// Saves a job to storage.
    pub async fn save_job(&self, job: &Job) -> Result<(), PersistenceError> {
        let file_path = self.get_job_file_path(&job.id);
        
        // Serialize job to JSON
        let json_data = serde_json::to_string_pretty(job)?;
        
        // Write to file
        let mut file = tokio_fs::File::create(&file_path).await?;
        file.write_all(json_data.as_bytes()).await?;
        file.flush().await?;
        
        Ok(())
    }
    
    /// Loads a job from storage.
    pub async fn load_job(&self, job_id: &JobId) -> Result<Job, PersistenceError> {
        let file_path = self.get_job_file_path(job_id);
        
        if !file_path.exists() {
            return Err(PersistenceError::JobNotFound(job_id.clone()));
        }
        
        // Read file content
        let content = tokio_fs::read_to_string(&file_path).await?;
        
        // Deserialize job from JSON
        let job: Job = serde_json::from_str(&content)?;
        
        Ok(job)
    }
    
    /// Deletes a job from storage.
    pub async fn delete_job(&self, job_id: &JobId) -> Result<(), PersistenceError> {
        let file_path = self.get_job_file_path(job_id);
        
        if file_path.exists() {
            tokio_fs::remove_file(&file_path).await?;
        }
        
        Ok(())
    }
    
    /// Lists all jobs in storage.
    pub async fn list_jobs(&self) -> Result<Vec<Job>, PersistenceError> {
        let mut jobs = Vec::new();
        
        // Read all JSON files in the storage directory
        let mut entries = tokio_fs::read_dir(&self.storage_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            // Only process JSON files
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = tokio_fs::read_to_string(&path).await {
                    if let Ok(job) = serde_json::from_str::<Job>(&content) {
                        jobs.push(job);
                    }
                }
            }
        }
        
        Ok(jobs)
    }
    
    /// Loads all jobs into cache.
    pub async fn load_all_jobs(&mut self) -> Result<(), PersistenceError> {
        self.job_cache.clear();
        
        let jobs = self.list_jobs().await?;
        for job in jobs {
            self.job_cache.insert(job.id.clone(), job);
        }
        
        Ok(())
    }
    
    /// Gets a job from cache.
    pub fn get_cached_job(&self, job_id: &JobId) -> Option<&Job> {
        self.job_cache.get(job_id)
    }
    
    /// Updates a job in cache.
    pub fn update_cached_job(&mut self, job: Job) {
        self.job_cache.insert(job.id.clone(), job);
    }
    
    /// Removes a job from cache.
    pub fn remove_cached_job(&mut self, job_id: &JobId) {
        self.job_cache.remove(job_id);
    }
    
    /// Gets all cached jobs.
    pub fn get_all_cached_jobs(&self) -> Vec<&Job> {
        self.job_cache.values().collect()
    }
    
    /// Clears the cache.
    pub fn clear_cache(&mut self) {
        self.job_cache.clear();
    }
    
    /// Gets storage statistics.
    pub async fn get_storage_stats(&self) -> Result<StorageStats, PersistenceError> {
        let mut stats = StorageStats::default();
        
        let mut entries = tokio_fs::read_dir(&self.storage_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                stats.total_files += 1;
                
                if let Ok(metadata) = entry.metadata().await {
                    stats.total_size += metadata.len();
                }
            }
        }
        
        Ok(stats)
    }
    
    /// Validates job data integrity.
    pub async fn validate_job_data(&self, job_id: &JobId) -> Result<bool, PersistenceError> {
        let file_path = self.get_job_file_path(job_id);
        
        if !file_path.exists() {
            return Ok(false);
        }
        
        // Try to load and validate the job
        match self.load_job(job_id).await {
            Ok(job) => {
                // Basic validation
                if job.id.is_empty() || job.name.is_empty() || job.command.is_empty() {
                    return Ok(false);
                }
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }
    
    /// Backs up job data.
    pub async fn backup_jobs(&self, backup_dir: &Path) -> Result<(), PersistenceError> {
        // Create backup directory if it doesn't exist
        if !backup_dir.exists() {
            tokio_fs::create_dir_all(backup_dir).await?;
        }
        
        let jobs = self.list_jobs().await?;
        
        for job in jobs {
            let backup_file = backup_dir.join(format!("{}.json", job.id));
            let json_data = serde_json::to_string_pretty(&job)?;
            
            let mut file = tokio_fs::File::create(&backup_file).await?;
            file.write_all(json_data.as_bytes()).await?;
            file.flush().await?;
        }
        
        Ok(())
    }
    
    /// Restores job data from backup.
    pub async fn restore_jobs(&self, backup_dir: &Path) -> Result<(), PersistenceError> {
        if !backup_dir.exists() {
            return Err(PersistenceError::StorageDirectoryError(
                "Backup directory does not exist".to_string()
            ));
        }
        
        let mut entries = tokio_fs::read_dir(backup_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = tokio_fs::read_to_string(&path).await {
                    if let Ok(job) = serde_json::from_str::<Job>(&content) {
                        self.save_job(&job).await?;
                    }
                }
            }
        }
        
        Ok(())
    }
}

/// Statistics about job storage.
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub total_files: usize,
    pub total_size: u64,
}

impl Default for StorageStats {
    fn default() -> Self {
        StorageStats {
            total_files: 0,
            total_size: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::job::Job;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_save_and_load_job() {
        let temp_dir = tempdir().unwrap();
        let storage_dir = temp_dir.path().join("jobs");
        tokio_fs::create_dir_all(&storage_dir).await.unwrap();
        
        let mut persistence = JobPersistence {
            storage_dir,
            job_cache: HashMap::new(),
        };
        
        let job = Job::new("test-job".to_string(), "echo".to_string())
            .with_cron("0 18 * * *".to_string());
        
        // Save job
        assert!(persistence.save_job(&job).await.is_ok());
        
        // Load job
        let loaded_job = persistence.load_job(&job.id).await.unwrap();
        assert_eq!(loaded_job.id, job.id);
        assert_eq!(loaded_job.name, job.name);
        assert_eq!(loaded_job.command, job.command);
    }
    
    #[tokio::test]
    async fn test_delete_job() {
        let temp_dir = tempdir().unwrap();
        let storage_dir = temp_dir.path().join("jobs");
        tokio_fs::create_dir_all(&storage_dir).await.unwrap();
        
        let mut persistence = JobPersistence {
            storage_dir,
            job_cache: HashMap::new(),
        };
        
        let job = Job::new("test-job".to_string(), "echo".to_string());
        
        // Save job
        persistence.save_job(&job).await.unwrap();
        
        // Verify job exists
        assert!(persistence.load_job(&job.id).await.is_ok());
        
        // Delete job
        assert!(persistence.delete_job(&job.id).await.is_ok());
        
        // Verify job is deleted
        assert!(persistence.load_job(&job.id).await.is_err());
    }
    
    #[tokio::test]
    async fn test_list_jobs() {
        let temp_dir = tempdir().unwrap();
        let storage_dir = temp_dir.path().join("jobs");
        tokio_fs::create_dir_all(&storage_dir).await.unwrap();
        
        let mut persistence = JobPersistence {
            storage_dir,
            job_cache: HashMap::new(),
        };
        
        let job1 = Job::new("job1".to_string(), "echo".to_string());
        let job2 = Job::new("job2".to_string(), "ls".to_string());
        
        // Save jobs
        persistence.save_job(&job1).await.unwrap();
        persistence.save_job(&job2).await.unwrap();
        
        // List jobs
        let jobs = persistence.list_jobs().await.unwrap();
        assert_eq!(jobs.len(), 2);
        
        let job_ids: Vec<String> = jobs.iter().map(|j| j.id.clone()).collect();
        assert!(job_ids.contains(&job1.id));
        assert!(job_ids.contains(&job2.id));
    }
} 
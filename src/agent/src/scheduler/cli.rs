use crate::scheduler::{Scheduler, SchedulerError};
use crate::scheduler::job::{Job, JobId, JobStatus};
use std::sync::OnceLock;

/// Global scheduler instance for CLI operations
static SCHEDULER: OnceLock<Scheduler> = OnceLock::new();

/// Initialize the scheduler for CLI operations
pub async fn init_scheduler() -> Result<(), SchedulerError> {
    if SCHEDULER.get().is_none() {
        let scheduler = Scheduler::new().await?;
        SCHEDULER.set(scheduler).map_err(|_| SchedulerError::InvalidJob("Failed to set scheduler".to_string()))?;
    }
    
    // Start the scheduler if it's not already running
    let scheduler = get_scheduler()?;
    scheduler.start().await?;
    
    Ok(())
}

/// Get the scheduler instance
fn get_scheduler() -> Result<&'static Scheduler, SchedulerError> {
    SCHEDULER.get().ok_or(SchedulerError::InvalidJob("Scheduler not initialized".to_string()))
}

/// Add a new scheduled job
pub async fn add_job(
    name: String,
    schedule: String,
    command: String,
    args: Option<Vec<String>>,
    timezone: Option<String>,
    description: Option<String>,
) -> Result<JobId, SchedulerError> {
    let scheduler = get_scheduler()?;
    
    // Create a job using the scheduler API
    let mut job = Job::new(name.clone(), command.clone())
        .with_args(args.unwrap_or_default());
    
    // Set the cron schedule
    if !schedule.is_empty() {
        job = job.with_cron(schedule.clone());
    }
    
    // Set timezone if provided
    if let Some(tz) = timezone {
        job.schedule.timezone = Some(tz.clone());
    }
    
    // Set description if provided
    if let Some(desc) = description {
        job = job.with_description(desc.clone());
    }
    
    // Add the job to the scheduler
    scheduler.add_job(job).await
}

/// List all scheduled jobs
pub async fn list_jobs(verbose: bool) -> Result<Vec<String>, SchedulerError> {
    let scheduler = get_scheduler()?;
    
    let jobs = scheduler.list_jobs().await?;
    
    let mut output = Vec::new();
    for job_info in jobs {
        if verbose {
            output.push(format!(
                "ID: {}\nName: {}\nStatus: {:?}\nSchedule: {:?}\nCommand: {}\n---",
                job_info.job.id,
                job_info.job.name,
                job_info.status,
                job_info.job.schedule,
                job_info.job.command
            ));
        } else {
            output.push(format!(
                "{} - {} - {:?}",
                job_info.job.id,
                job_info.job.name,
                job_info.status
            ));
        }
    }
    
    Ok(output)
}

/// Remove a scheduled job
pub async fn remove_job(job_id: &str) -> Result<(), SchedulerError> {
    let scheduler = get_scheduler()?;
    scheduler.remove_job(&job_id.to_string()).await
}

/// Get job status
pub async fn get_job_status(job_id: Option<&str>) -> Result<String, SchedulerError> {
    let scheduler = get_scheduler()?;
    
    match job_id {
        Some(id) => {
            let status = scheduler.get_job_status(&id.to_string()).await?;
            Ok(format!("Job {} status: {:?}", id, status))
        }
        None => {
            // Return overall scheduler status
            let jobs = scheduler.list_jobs().await?;
            let total_jobs = jobs.len();
            let active_jobs = jobs.iter().filter(|j| j.status == JobStatus::Scheduled).count();
            
            Ok(format!(
                "Scheduler Status:\n✅ Scheduler is running\n📊 Total jobs: {}\n🔄 Active jobs: {}",
                total_jobs, active_jobs
            ))
        }
    }
}

/// Enable a job
pub async fn enable_job(job_id: &str) -> Result<(), SchedulerError> {
    // TODO: Implement job enable functionality
    // This would require adding an enable_job method to the Scheduler
    println!("Enabling job: {}", job_id);
    Ok(())
}

/// Disable a job
pub async fn disable_job(job_id: &str) -> Result<(), SchedulerError> {
    // TODO: Implement job disable functionality
    // This would require adding a disable_job method to the Scheduler
    println!("Disabling job: {}", job_id);
    Ok(())
}

/// Start the scheduler
pub async fn start_scheduler() -> Result<(), SchedulerError> {
    let scheduler = get_scheduler()?;
    scheduler.start().await
}

/// Stop the scheduler
pub async fn stop_scheduler() -> Result<(), SchedulerError> {
    let scheduler = get_scheduler()?;
    scheduler.stop().await
} 
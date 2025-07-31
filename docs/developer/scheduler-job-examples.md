# Rae Scheduler Job Creation Examples

## Overview

This guide provides comprehensive examples of how to create and configure jobs in the Rae Scheduler system. It covers various job types, scheduling patterns, and advanced configurations.

## Basic Job Creation

### Simple Daily Job

```bash
# Create a daily digest job
rae-agent scheduler add \
  --name "daily-digest" \
  --schedule "0 18 * * *" \
  --command "rae-agent summary"
```

### Weekly Summary Job

```bash
# Create a weekly summary job
rae-agent scheduler add \
  --name "weekly-summary" \
  --schedule "0 9 * * 0" \
  --command "rae-agent summary --weekly"
```

### Hourly Monitoring Job

```bash
# Create an hourly health check
rae-agent scheduler add \
  --name "health-check" \
  --schedule "0 * * * *" \
  --command "rae-agent health"
```

## Advanced Job Configuration

### Job with Arguments

```bash
# Job with command arguments
rae-agent scheduler add \
  --name "custom-summary" \
  --schedule "0 18 * * *" \
  --command "rae-agent summary" \
  --args "--format json --include-archived"
```

### Job with Working Directory

```bash
# Job with specific working directory
rae-agent scheduler add \
  --name "project-backup" \
  --schedule "0 2 * * *" \
  --command "tar" \
  --args "-czf backup-$(date +%Y%m%d).tar.gz ." \
  --working-dir "/path/to/project"
```

### Job with Environment Variables

```bash
# Job with environment variables
rae-agent scheduler add \
  --name "api-sync" \
  --schedule "0 */4 * * *" \
  --command "rae-agent sync" \
  --env "API_KEY=your-api-key" \
  --env "ENVIRONMENT=production"
```

## Scheduling Patterns

### Business Hours Jobs

```bash
# Job that runs during business hours only
rae-agent scheduler add \
  --name "business-monitoring" \
  --schedule "0 9-17 * * 1-5" \
  --command "rae-agent monitor"
```

### Weekend Jobs

```bash
# Job that runs only on weekends
rae-agent scheduler add \
  --name "weekend-maintenance" \
  --schedule "0 3 * * 0,6" \
  --command "rae-agent maintenance"
```

### Quarterly Jobs

```bash
# Job that runs quarterly
rae-agent scheduler add \
  --name "quarterly-report" \
  --schedule "0 9 1 1,4,7,10 *" \
  --command "rae-agent report --quarterly"
```

## Timezone-Aware Jobs

### UTC Jobs

```bash
# Job scheduled in UTC
rae-agent scheduler add \
  --name "utc-backup" \
  --schedule "0 2 * * *" \
  --timezone "UTC" \
  --command "rae-agent backup"
```

### Timezone-Specific Jobs

```bash
# Job scheduled in Pacific Time
rae-agent scheduler add \
  --name "pst-notification" \
  --schedule "0 9 * * 1-5" \
  --timezone "America/Los_Angeles" \
  --command "rae-agent notify"
```

### DST-Aware Jobs

```bash
# Job that handles DST transitions
rae-agent scheduler add \
  --name "dst-safe-job" \
  --schedule "0 9 * * 1-5" \
  --timezone "America/New_York" \
  --command "rae-agent process"
```

## Retry and Error Handling

### Job with Retry Logic

```bash
# Job with automatic retries
rae-agent scheduler add \
  --name "reliable-sync" \
  --schedule "0 */2 * * *" \
  --command "rae-agent sync" \
  --retries 3 \
  --retry-delay 300
```

### Job with Exponential Backoff

```bash
# Job with exponential backoff retry
rae-agent scheduler add \
  --name "api-call" \
  --schedule "0 * * * *" \
  --command "rae-agent api-call" \
  --retries 5 \
  --retry-delay 60 \
  --backoff-multiplier 2.0
```

### Job with Custom Error Handling

```bash
# Job with custom error handling
rae-agent scheduler add \
  --name "critical-process" \
  --schedule "0 18 * * *" \
  --command "rae-agent critical-process" \
  --retries 3 \
  --on-failure "rae-agent alert --level critical"
```

## Resource Management

### Job with Memory Limits

```bash
# Job with memory limit
rae-agent scheduler add \
  --name "memory-intensive" \
  --schedule "0 2 * * *" \
  --command "rae-agent process-large-data" \
  --max-memory 2048
```

### Job with Time Limits

```bash
# Job with execution time limit
rae-agent scheduler add \
  --name "time-limited" \
  --schedule "0 */6 * * *" \
  --command "rae-agent long-process" \
  --timeout 1800
```

### Job with CPU Limits

```bash
# Job with CPU usage limits
rae-agent scheduler add \
  --name "cpu-intensive" \
  --schedule "0 3 * * *" \
  --command "rae-agent cpu-task" \
  --max-cpu 50
```

## Priority and Dependencies

### High Priority Job

```bash
# Critical job with high priority
rae-agent scheduler add \
  --name "critical-alert" \
  --schedule "0 * * * *" \
  --command "rae-agent check-critical" \
  --priority high
```

### Low Priority Background Job

```bash
# Background job with low priority
rae-agent scheduler add \
  --name "background-cleanup" \
  --schedule "0 4 * * *" \
  --command "rae-agent cleanup" \
  --priority low
```

### Job with Dependencies

```bash
# Job that depends on another job
rae-agent scheduler add \
  --name "data-processing" \
  --schedule "0 1 * * *" \
  --command "rae-agent process-data" \
  --depends-on "data-collection"
```

## Event-Based Jobs

### File Change Trigger

```bash
# Job triggered by file changes
rae-agent scheduler add \
  --name "config-watcher" \
  --event "file-changed" \
  --path "/etc/rae/config.json" \
  --command "rae-agent reload-config"
```

### Directory Monitoring

```bash
# Job triggered by directory changes
rae-agent scheduler add \
  --name "backup-trigger" \
  --event "file-changed" \
  --path "/data/important" \
  --recursive \
  --command "rae-agent backup --incremental"
```

### System Event Trigger

```bash
# Job triggered by system events
rae-agent scheduler add \
  --name "startup-job" \
  --event "system-start" \
  --command "rae-agent startup"
```

## Pattern-Based Jobs

### CPU Usage Monitor

```bash
# Job triggered by high CPU usage
rae-agent scheduler add \
  --name "cpu-alert" \
  --pattern "cpu-usage" \
  --threshold 80 \
  --command "rae-agent alert --cpu-high"
```

### Memory Usage Monitor

```bash
# Job triggered by high memory usage
rae-agent scheduler add \
  --name "memory-alert" \
  --pattern "memory-usage" \
  --threshold 90 \
  --command "rae-agent alert --memory-high"
```

### Network Monitor

```bash
# Job triggered by network issues
rae-agent scheduler add \
  --name "network-check" \
  --pattern "network-latency" \
  --threshold 1000 \
  --command "rae-agent check-network"
```

## Complex Job Configurations

### Multi-Step Job

```bash
# Job with multiple steps
rae-agent scheduler add \
  --name "data-pipeline" \
  --schedule "0 2 * * *" \
  --command "bash" \
  --args "-c 'rae-agent collect-data && rae-agent process-data && rae-agent upload-results'" \
  --timeout 3600
```

### Conditional Job

```bash
# Job with conditional execution
rae-agent scheduler add \
  --name "conditional-backup" \
  --schedule "0 1 * * *" \
  --command "bash" \
  --args "-c 'if [ -f /data/important ]; then rae-agent backup; fi'"
```

### Job with Logging

```bash
# Job with detailed logging
rae-agent scheduler add \
  --name "verbose-process" \
  --schedule "0 3 * * *" \
  --command "rae-agent process" \
  --log-level debug \
  --log-file "/var/log/rae/process.log"
```

## Integration Examples

### Database Backup Job

```bash
# Database backup job
rae-agent scheduler add \
  --name "db-backup" \
  --schedule "0 2 * * *" \
  --command "pg_dump" \
  --args "-h localhost -U postgres -d myapp > /backups/db-$(date +%Y%m%d).sql" \
  --env "PGPASSWORD=your-password"
```

### Email Notification Job

```bash
# Email notification job
rae-agent scheduler add \
  --name "daily-report" \
  --schedule "0 9 * * 1-5" \
  --command "rae-agent report" \
  --args "--email admin@example.com --format html" \
  --env "SMTP_SERVER=smtp.example.com"
```

### API Synchronization Job

```bash
# API sync job
rae-agent scheduler add \
  --name "api-sync" \
  --schedule "0 */30 * * *" \
  --command "rae-agent sync" \
  --args "--api external-api --endpoint /data" \
  --env "API_KEY=your-api-key" \
  --env "API_URL=https://api.example.com" \
  --retries 3 \
  --retry-delay 300
```

## Job Management Examples

### List All Jobs

```bash
# List all scheduled jobs
rae-agent scheduler list

# List jobs with details
rae-agent scheduler list --verbose

# List jobs by status
rae-agent scheduler list --status running
```

### Modify Existing Job

```bash
# Update job schedule
rae-agent scheduler update <job-id> --schedule "0 19 * * *"

# Update job command
rae-agent scheduler update <job-id> --command "rae-agent summary --new-format"

# Update job priority
rae-agent scheduler update <job-id> --priority high
```

### Job Status and Monitoring

```bash
# Check job status
rae-agent scheduler status <job-id>

# View job logs
rae-agent scheduler logs <job-id>

# Get job history
rae-agent scheduler history <job-id>
```

### Job Control

```bash
# Enable job
rae-agent scheduler enable <job-id>

# Disable job
rae-agent scheduler disable <job-id>

# Run job immediately
rae-agent scheduler run <job-id>

# Stop running job
rae-agent scheduler stop <job-id>
```

## Best Practices

### Naming Conventions

```bash
# Use descriptive names with schedule info
rae-agent scheduler add --name "daily-digest-6pm-pst" --schedule "0 18 * * *" --timezone "America/Los_Angeles"

# Include job type in name
rae-agent scheduler add --name "backup-database-daily" --schedule "0 2 * * *"

# Include environment in name
rae-agent scheduler add --name "sync-api-production" --schedule "0 */30 * * *"
```

### Error Handling

```bash
# Always include retry logic for network jobs
rae-agent scheduler add --name "api-sync" --schedule "0 */30 * * *" --retries 3

# Use appropriate timeouts
rae-agent scheduler add --name "long-process" --schedule "0 2 * * *" --timeout 3600

# Include failure notifications
rae-agent scheduler add --name "critical-job" --schedule "0 18 * * *" --on-failure "rae-agent alert"
```

### Resource Management

```bash
# Set appropriate resource limits
rae-agent scheduler add --name "memory-job" --schedule "0 3 * * *" --max-memory 1024

# Use low priority for background jobs
rae-agent scheduler add --name "cleanup" --schedule "0 4 * * *" --priority low

# Avoid scheduling too many concurrent jobs
rae-agent scheduler config --max-concurrent 10
```

### Testing Jobs

```bash
# Test job command before scheduling
rae-agent summary

# Test job with dry run
rae-agent scheduler add --name "test-job" --schedule "0 18 * * *" --command "echo test" --dry-run

# Test job execution
rae-agent scheduler run <job-id>
```

These examples provide a comprehensive guide for creating and managing jobs in the Rae Scheduler system. Use them as templates and adapt them to your specific needs. 
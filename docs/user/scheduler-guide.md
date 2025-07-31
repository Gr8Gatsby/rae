# Rae Scheduler User Guide

## Overview

The Rae Scheduler is a powerful automation system that allows you to schedule and manage jobs for your Rae agent. It supports time-based scheduling, event triggers, and pattern-based automation.

## Quick Start

### Basic Job Scheduling

Create a simple daily job:

```bash
# Schedule a daily digest at 6:00 PM
rae-agent scheduler add --name "daily-digest" --schedule "0 18 * * *" --command "rae-agent summary"
```

### Weekly Summary

Schedule a weekly summary on Sundays:

```bash
# Schedule weekly summary every Sunday at 9:00 AM
rae-agent scheduler add --name "weekly-summary" --schedule "0 9 * * 0" --command "rae-agent summary --weekly"
```

## Cron Syntax Reference

The scheduler uses standard cron syntax for time-based scheduling:

```
┌───────────── minute (0 - 59)
│ ┌───────────── hour (0 - 23)
│ │ ┌───────────── day of month (1 - 31)
│ │ │ ┌───────────── month (1 - 12)
│ │ │ │ ┌───────────── day of week (0 - 6) (Sunday = 0)
│ │ │ │ │
* * * * *
```

### Common Examples

| Schedule | Description |
|----------|-------------|
| `0 18 * * *` | Daily at 6:00 PM |
| `0 9 * * 0` | Weekly on Sundays at 9:00 AM |
| `0 0 1 * *` | Monthly on the 1st at midnight |
| `*/15 * * * *` | Every 15 minutes |
| `0 */2 * * *` | Every 2 hours |

### Timezone Support

Jobs are scheduled in your local timezone by default. You can specify a different timezone:

```bash
# Schedule in UTC
rae-agent scheduler add --name "utc-job" --schedule "0 18 * * *" --timezone "UTC" --command "echo 'UTC job'"

# Schedule in a specific timezone
rae-agent scheduler add --name "pst-job" --schedule "0 18 * * *" --timezone "America/Los_Angeles" --command "echo 'PST job'"
```

## Job Management

### List All Jobs

```bash
rae-agent scheduler list
```

### Check Job Status

```bash
rae-agent scheduler status <job-id>
```

### Remove a Job

```bash
rae-agent scheduler remove <job-id>
```

### Enable/Disable Jobs

```bash
# Disable a job
rae-agent scheduler disable <job-id>

# Enable a job
rae-agent scheduler enable <job-id>
```

## Advanced Features

### Retry Logic

Jobs can be configured with retry policies:

```bash
# Job with 3 retries, 5-minute delays
rae-agent scheduler add --name "retry-job" --schedule "0 18 * * *" --command "some-command" --retries 3 --retry-delay 300
```

### Priority Scheduling

Jobs can have different priorities:

```bash
# High priority job
rae-agent scheduler add --name "critical-job" --schedule "0 18 * * *" --command "critical-command" --priority high

# Low priority job
rae-agent scheduler add --name "background-job" --schedule "0 18 * * *" --command "background-command" --priority low
```

### Resource Limits

Set resource limits for jobs:

```bash
# Job with memory and time limits
rae-agent scheduler add --name "limited-job" --schedule "0 18 * * *" --command "memory-intensive-command" --max-memory 512 --max-time 3600
```

## Event-Based Triggers

### File Change Triggers

Trigger jobs when files change:

```bash
# Run job when config file changes
rae-agent scheduler add --name "config-watcher" --event "file-changed" --path "/path/to/config" --command "rae-agent reload-config"
```

### System Event Triggers

Trigger jobs on system events:

```bash
# Run job when system starts
rae-agent scheduler add --name "startup-job" --event "system-start" --command "rae-agent startup"
```

## Pattern-Based Triggers

### Usage Pattern Triggers

Trigger jobs based on usage patterns:

```bash
# Run job when high CPU usage detected
rae-agent scheduler add --name "cpu-monitor" --pattern "cpu-usage" --threshold 80 --command "rae-agent alert high-cpu"
```

## Troubleshooting

### Common Issues

#### Job Not Running

1. **Check if scheduler is running:**
   ```bash
   rae-agent scheduler status
   ```

2. **Verify job is enabled:**
   ```bash
   rae-agent scheduler list
   ```

3. **Check job logs:**
   ```bash
   rae-agent scheduler logs <job-id>
   ```

#### Job Failing

1. **Check command syntax:**
   ```bash
   # Test command manually
   rae-agent summary
   ```

2. **Verify permissions:**
   ```bash
   # Check if command can be executed
   which rae-agent
   ```

3. **Check resource limits:**
   ```bash
   # Verify system has enough resources
   rae-agent scheduler status <job-id>
   ```

#### Timezone Issues

1. **Check system timezone:**
   ```bash
   date
   ```

2. **Verify job timezone:**
   ```bash
   rae-agent scheduler list --verbose
   ```

### Getting Help

For additional help:

```bash
# Show scheduler help
rae-agent scheduler --help

# Show specific command help
rae-agent scheduler add --help
```

## Best Practices

### Job Naming

Use descriptive names for your jobs:

```bash
# Good
rae-agent scheduler add --name "daily-digest-6pm" --schedule "0 18 * * *" --command "rae-agent summary"

# Bad
rae-agent scheduler add --name "job1" --schedule "0 18 * * *" --command "rae-agent summary"
```

### Testing Jobs

Always test your commands before scheduling:

```bash
# Test the command manually first
rae-agent summary

# Then schedule it
rae-agent scheduler add --name "tested-job" --schedule "0 18 * * *" --command "rae-agent summary"
```

### Monitoring

Regularly check job status:

```bash
# Check all jobs
rae-agent scheduler list

# Check specific job
rae-agent scheduler status <job-id>
```

### Backup and Recovery

Jobs are automatically persisted, but you can backup manually:

```bash
# Backup jobs
rae-agent scheduler backup --output jobs-backup.json

# Restore jobs
rae-agent scheduler restore --input jobs-backup.json
``` 
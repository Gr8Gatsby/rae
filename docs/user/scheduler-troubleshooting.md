# Rae Scheduler Troubleshooting Guide

## Overview

This guide helps you diagnose and resolve common issues with the Rae Scheduler system. It covers job execution problems, scheduling issues, and system-level troubleshooting.

## Quick Diagnostic Commands

### Check Scheduler Status
```bash
# Check if scheduler is running
rae-agent scheduler status

# List all jobs
rae-agent scheduler list

# Check scheduler health
rae-agent scheduler health
```

### Check Job Status
```bash
# Check specific job status
rae-agent scheduler status <job-id>

# View job logs
rae-agent scheduler logs <job-id>

# Get job history
rae-agent scheduler history <job-id>
```

## Common Issues and Solutions

### Job Not Running

#### Issue: Job is scheduled but not executing

**Symptoms:**
- Job appears in list but never runs
- No execution logs
- Job status remains "Scheduled"

**Diagnostic Steps:**

1. **Check scheduler status:**
   ```bash
   rae-agent scheduler status
   ```

2. **Verify job is enabled:**
   ```bash
   rae-agent scheduler list --verbose
   ```

3. **Check cron expression:**
   ```bash
   rae-agent scheduler validate --schedule "0 18 * * *"
   ```

4. **Test command manually:**
   ```bash
   # Test the command that should run
   rae-agent summary
   ```

**Solutions:**

1. **Enable the job:**
   ```bash
   rae-agent scheduler enable <job-id>
   ```

2. **Fix cron expression:**
   ```bash
   # Remove and recreate with correct schedule
   rae-agent scheduler remove <job-id>
   rae-agent scheduler add --name "fixed-job" --schedule "0 18 * * *" --command "rae-agent summary"
   ```

3. **Check system time:**
   ```bash
   date
   # Ensure system clock is accurate
   ```

#### Issue: Job runs at wrong time

**Symptoms:**
- Job executes at unexpected times
- Timezone confusion
- DST-related issues

**Diagnostic Steps:**

1. **Check system timezone:**
   ```bash
   date
   timedatectl status  # Linux
   systemsetup -gettimezone  # macOS
   ```

2. **Verify job timezone:**
   ```bash
   rae-agent scheduler list --verbose
   ```

3. **Check DST status:**
   ```bash
   # Check if DST is active
   date +%Z
   ```

**Solutions:**

1. **Set correct timezone:**
   ```bash
   # Recreate job with specific timezone
   rae-agent scheduler remove <job-id>
   rae-agent scheduler add --name "timezone-job" --schedule "0 18 * * *" --timezone "America/New_York" --command "rae-agent summary"
   ```

2. **Use UTC for consistency:**
   ```bash
   rae-agent scheduler add --name "utc-job" --schedule "0 18 * * *" --timezone "UTC" --command "rae-agent summary"
   ```

### Job Execution Failures

#### Issue: Job fails to execute

**Symptoms:**
- Job status shows "Failed"
- Error messages in logs
- Command not found errors

**Diagnostic Steps:**

1. **Check job logs:**
   ```bash
   rae-agent scheduler logs <job-id>
   ```

2. **Test command manually:**
   ```bash
   # Run the command that failed
   rae-agent summary
   ```

3. **Check file permissions:**
   ```bash
   # Check if rae-agent is executable
   which rae-agent
   ls -la $(which rae-agent)
   ```

4. **Check working directory:**
   ```bash
   # Verify the command can run from scheduler context
   pwd
   echo $PATH
   ```

**Solutions:**

1. **Fix command path:**
   ```bash
   # Use absolute path
   rae-agent scheduler add --name "fixed-job" --schedule "0 18 * * *" --command "/usr/local/bin/rae-agent summary"
   ```

2. **Fix permissions:**
   ```bash
   # Make rae-agent executable
   chmod +x $(which rae-agent)
   ```

3. **Set working directory:**
   ```bash
   # Add working directory to job
   rae-agent scheduler add --name "wd-job" --schedule "0 18 * * *" --command "rae-agent summary" --working-dir "/path/to/directory"
   ```

#### Issue: Job times out

**Symptoms:**
- Job runs but never completes
- Timeout errors in logs
- Resource exhaustion

**Diagnostic Steps:**

1. **Check resource limits:**
   ```bash
   rae-agent scheduler status <job-id>
   ```

2. **Monitor system resources:**
   ```bash
   # Check CPU and memory usage
   top
   htop
   ```

3. **Check job duration:**
   ```bash
   rae-agent scheduler history <job-id>
   ```

**Solutions:**

1. **Increase timeout:**
   ```bash
   # Recreate job with longer timeout
   rae-agent scheduler add --name "long-job" --schedule "0 18 * * *" --command "long-running-command" --timeout 3600
   ```

2. **Optimize command:**
   ```bash
   # Use more efficient command
   rae-agent scheduler add --name "optimized-job" --schedule "0 18 * * *" --command "rae-agent summary --fast"
   ```

### System-Level Issues

#### Issue: Scheduler won't start

**Symptoms:**
- `rae-agent scheduler start` fails
- Error messages about missing components
- Permission denied errors

**Diagnostic Steps:**

1. **Check dependencies:**
   ```bash
   # Verify Rust binary is built
   cargo build --release
   ```

2. **Check file permissions:**
   ```bash
   # Check scheduler directory permissions
   ls -la ~/.rae/scheduler/
   ```

3. **Check system resources:**
   ```bash
   # Check available disk space
   df -h
   
   # Check available memory
   free -h
   ```

**Solutions:**

1. **Rebuild scheduler:**
   ```bash
   # Clean and rebuild
   cargo clean
   cargo build --release
   ```

2. **Fix permissions:**
   ```bash
   # Create scheduler directory with correct permissions
   mkdir -p ~/.rae/scheduler
   chmod 755 ~/.rae/scheduler
   ```

3. **Free up resources:**
   ```bash
   # Clean up old job results
   rae-agent scheduler cleanup
   ```

#### Issue: Scheduler crashes

**Symptoms:**
- Scheduler stops unexpectedly
- Error messages in system logs
- Jobs stop running

**Diagnostic Steps:**

1. **Check system logs:**
   ```bash
   # Check system logs for errors
   journalctl -u rae-agent  # Linux
   log show --predicate 'process == "rae-agent"'  # macOS
   ```

2. **Check scheduler logs:**
   ```bash
   # Check scheduler-specific logs
   tail -f ~/.rae/logs/scheduler.log
   ```

3. **Check memory usage:**
   ```bash
   # Monitor memory usage
   ps aux | grep rae-agent
   ```

**Solutions:**

1. **Restart scheduler:**
   ```bash
   rae-agent scheduler stop
   rae-agent scheduler start
   ```

2. **Reset scheduler state:**
   ```bash
   # Backup and reset
   rae-agent scheduler backup --output backup.json
   rm -rf ~/.rae/scheduler/jobs.json
   rae-agent scheduler start
   ```

3. **Update scheduler:**
   ```bash
   # Pull latest changes and rebuild
   git pull
   cargo build --release
   ```

### Performance Issues

#### Issue: Scheduler is slow

**Symptoms:**
- Job execution takes too long
- UI is unresponsive
- High CPU/memory usage

**Diagnostic Steps:**

1. **Check system resources:**
   ```bash
   # Monitor system performance
   top
   iostat  # Linux
   vm_stat  # macOS
   ```

2. **Check job queue:**
   ```bash
   rae-agent scheduler queue
   ```

3. **Check concurrent jobs:**
   ```bash
   rae-agent scheduler stats
   ```

**Solutions:**

1. **Limit concurrent jobs:**
   ```bash
   # Set maximum concurrent jobs
   rae-agent scheduler config --max-concurrent 5
   ```

2. **Optimize job execution:**
   ```bash
   # Use more efficient commands
   rae-agent scheduler add --name "fast-job" --schedule "0 18 * * *" --command "rae-agent summary --quick"
   ```

3. **Clean up old data:**
   ```bash
   # Remove old job results
   rae-agent scheduler cleanup --older-than 30d
   ```

## Advanced Troubleshooting

### Debug Mode

Enable debug logging for detailed diagnostics:

```bash
# Enable debug mode
RUST_LOG=debug rae-agent scheduler start

# Check debug logs
tail -f ~/.rae/logs/debug.log
```

### Network Issues

If jobs involve network operations:

```bash
# Test network connectivity
ping google.com

# Check DNS resolution
nslookup google.com

# Test specific endpoints
curl -I https://api.example.com
```

### File System Issues

For jobs that work with files:

```bash
# Check disk space
df -h

# Check file permissions
ls -la /path/to/files

# Check file system health
fsck /dev/sda1  # Linux
diskutil verifyVolume /  # macOS
```

### Permission Issues

```bash
# Check user permissions
whoami
groups

# Check file ownership
ls -la ~/.rae/

# Fix permissions if needed
chown -R $USER:$USER ~/.rae/
chmod -R 755 ~/.rae/
```

## Recovery Procedures

### Complete Reset

If the scheduler is completely broken:

```bash
# Stop scheduler
rae-agent scheduler stop

# Backup current state
cp -r ~/.rae/scheduler/ ~/.rae/scheduler.backup/

# Remove scheduler data
rm -rf ~/.rae/scheduler/

# Restart scheduler
rae-agent scheduler start

# Recreate jobs from backup
# (Manually recreate jobs from backup.json)
```

### Job Recovery

If specific jobs are problematic:

```bash
# Disable problematic job
rae-agent scheduler disable <job-id>

# Check job configuration
rae-agent scheduler show <job-id>

# Fix and re-enable
rae-agent scheduler enable <job-id>
```

### Data Recovery

If job data is corrupted:

```bash
# Backup current data
rae-agent scheduler backup --output recovery-backup.json

# Validate backup
cat recovery-backup.json | jq .

# Restore from backup
rae-agent scheduler restore --input recovery-backup.json
```

## Prevention

### Regular Maintenance

```bash
# Weekly cleanup
rae-agent scheduler cleanup --older-than 7d

# Monthly backup
rae-agent scheduler backup --output monthly-backup-$(date +%Y%m).json

# Check scheduler health
rae-agent scheduler health
```

### Monitoring

Set up monitoring for:
- Job success rates
- Execution times
- Resource usage
- Error rates

### Best Practices

1. **Test commands before scheduling**
2. **Use descriptive job names**
3. **Document job purposes**
4. **Monitor job execution**
5. **Regular backups**
6. **Keep scheduler updated**

## Getting Help

### Collecting Information

Before seeking help, collect:

```bash
# System information
uname -a
cat /etc/os-release  # Linux
sw_vers  # macOS

# Scheduler status
rae-agent scheduler status
rae-agent scheduler list --verbose

# Recent logs
tail -100 ~/.rae/logs/scheduler.log

# Job configuration
rae-agent scheduler show <job-id>
```

### Reporting Issues

Include in bug reports:
1. **System information** (OS, version, architecture)
2. **Scheduler version** (`rae-agent --version`)
3. **Error messages** (full text)
4. **Steps to reproduce**
5. **Expected vs actual behavior**
6. **Relevant logs**

This comprehensive troubleshooting guide should help resolve most scheduler issues. For persistent problems, check the project documentation or create an issue in the repository. 
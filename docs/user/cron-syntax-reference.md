# Cron Syntax Reference

## Overview

The Rae Scheduler uses standard cron syntax for time-based job scheduling. This reference covers all supported cron expressions and provides examples for common scheduling patterns.

## Basic Cron Format

```
┌───────────── minute (0 - 59)
│ ┌───────────── hour (0 - 23)
│ │ ┌───────────── day of month (1 - 31)
│ │ │ ┌───────────── month (1 - 12)
│ │ │ │ ┌───────────── day of week (0 - 6) (Sunday = 0)
│ │ │ │ │
* * * * *
```

### Field Definitions

| Field | Range | Description |
|-------|-------|-------------|
| Minute | 0-59 | Minute of the hour |
| Hour | 0-23 | Hour of the day (24-hour format) |
| Day of Month | 1-31 | Day of the month |
| Month | 1-12 | Month of the year |
| Day of Week | 0-6 | Day of the week (0 = Sunday) |

## Special Characters

### Asterisk (*)
Represents "every" value in the field.

```
* * * * *    # Every minute of every hour
0 * * * *    # Every hour at minute 0
0 0 * * *    # Every day at midnight
```

### Comma (,)
Specifies multiple values.

```
0 8,12,18 * * *    # At 8 AM, 12 PM, and 6 PM daily
0 0 1,15 * *        # On the 1st and 15th of every month
```

### Hyphen (-)
Specifies a range of values.

```
0 9-17 * * *        # Every hour from 9 AM to 5 PM
0 0 1-7 * *         # First 7 days of every month
```

### Slash (/)
Specifies step values.

```
*/15 * * * *        # Every 15 minutes
0 */2 * * *         # Every 2 hours
0 0 */3 * *         # Every 3 days
```

### Question Mark (?)
Used in day-of-month or day-of-week fields to indicate "no specific value" (used when the other field is specified).

```
0 0 1 * ?           # 1st of every month
0 0 ? * 1           # Every Monday
```

## Common Examples

### Daily Jobs

| Expression | Description |
|------------|-------------|
| `0 6 * * *` | Daily at 6:00 AM |
| `0 18 * * *` | Daily at 6:00 PM |
| `0 0 * * *` | Daily at midnight |
| `30 14 * * *` | Daily at 2:30 PM |

### Weekly Jobs

| Expression | Description |
|------------|-------------|
| `0 9 * * 0` | Every Sunday at 9:00 AM |
| `0 18 * * 1` | Every Monday at 6:00 PM |
| `0 0 * * 6` | Every Saturday at midnight |
| `30 10 * * 5` | Every Friday at 10:30 AM |

### Monthly Jobs

| Expression | Description |
|------------|-------------|
| `0 0 1 * *` | 1st of every month at midnight |
| `0 9 15 * *` | 15th of every month at 9:00 AM |
| `0 18 28 * *` | 28th of every month at 6:00 PM |
| `0 0 1,15 * *` | 1st and 15th of every month |

### Hourly Jobs

| Expression | Description |
|------------|-------------|
| `0 * * * *` | Every hour at minute 0 |
| `30 * * * *` | Every hour at minute 30 |
| `0 */2 * * *` | Every 2 hours |
| `0 */4 * * *` | Every 4 hours |

### Frequent Jobs

| Expression | Description |
|------------|-------------|
| `* * * * *` | Every minute |
| `*/5 * * * *` | Every 5 minutes |
| `*/15 * * * *` | Every 15 minutes |
| `*/30 * * * *` | Every 30 minutes |

## Advanced Patterns

### Business Hours

```
0 9-17 * * 1-5     # Every hour from 9 AM to 5 PM, Monday to Friday
```

### Weekend Jobs

```
0 10 * * 0,6       # Every Saturday and Sunday at 10:00 AM
```

### Quarterly Jobs

```
0 0 1 1,4,7,10 *   # 1st of January, April, July, and October
```

### Yearly Jobs

```
0 0 1 1 *          # January 1st at midnight
```

## Timezone Considerations

### Local Timezone
By default, cron expressions are interpreted in the local system timezone.

```
0 18 * * *         # 6:00 PM in local timezone
```

### UTC Timezone
For consistent scheduling across timezones, use UTC.

```
0 18 * * *         # 6:00 PM UTC
```

### Specific Timezone
You can specify a timezone when creating jobs:

```bash
# Schedule in Pacific Time
rae-agent scheduler add --name "pst-job" --schedule "0 18 * * *" --timezone "America/Los_Angeles"

# Schedule in Eastern Time
rae-agent scheduler add --name "est-job" --schedule "0 18 * * *" --timezone "America/New_York"
```

## Common Scheduling Patterns

### Daily Digest
```
0 18 * * *         # Daily digest at 6:00 PM
```

### Weekly Summary
```
0 9 * * 0          # Weekly summary every Sunday at 9:00 AM
```

### Monthly Backup
```
0 2 1 * *          # Monthly backup on the 1st at 2:00 AM
```

### System Maintenance
```
0 3 * * 0          # System maintenance every Sunday at 3:00 AM
```

### Data Processing
```
0 */4 * * *        # Process data every 4 hours
```

### Monitoring
```
*/5 * * * *        # Health check every 5 minutes
```

## Validation Rules

### Valid Expressions
- All fields must be present
- Values must be within valid ranges
- Day of month and day of week cannot both be specified (use ? for one)

### Invalid Expressions
```
* * * *             # Missing field
99 * * * *          # Invalid minute (99)
* 25 * * *          # Invalid hour (25)
* * 32 * *          # Invalid day of month (32)
* * * 13 *          # Invalid month (13)
* * * * 7            # Invalid day of week (7)
```

## Troubleshooting

### Common Issues

#### Job Not Running
1. **Check timezone**: Verify the timezone setting
2. **Check system time**: Ensure system clock is accurate
3. **Check expression**: Validate cron syntax

#### Unexpected Execution Times
1. **Daylight Saving Time**: Be aware of DST transitions
2. **Timezone confusion**: Verify timezone interpretation
3. **Date boundaries**: Check month/day combinations

### Testing Cron Expressions

```bash
# Test a cron expression
rae-agent scheduler validate --schedule "0 18 * * *"

# List next execution times
rae-agent scheduler next --schedule "0 18 * * *" --count 5
```

## Best Practices

### Naming Conventions
- Use descriptive names that include the schedule
- Include timezone in name if not local
- Example: `daily-digest-6pm-pst`

### Documentation
- Document the purpose of each scheduled job
- Include timezone information
- Note any dependencies or prerequisites

### Testing
- Test cron expressions before scheduling
- Verify execution times in different timezones
- Test during DST transitions

### Monitoring
- Monitor job execution success rates
- Track execution times and durations
- Set up alerts for failed jobs

## Examples by Use Case

### Development
```
*/5 * * * *        # Auto-save every 5 minutes
0 9 * * 1-5        # Daily standup reminder
0 18 * * 1-5       # End-of-day summary
```

### Operations
```
0 */6 * * *        # System health check every 6 hours
0 2 * * 0          # Weekly backup
0 0 1 * *          # Monthly maintenance
```

### Analytics
```
0 0 * * *          # Daily analytics processing
0 1 * * 1          # Weekly report generation
0 0 1 * *          # Monthly data aggregation
```

### Monitoring
```
*/2 * * * *        # High-frequency monitoring
0 */15 * * *       # Regular health checks
0 0 * * *          # Daily system scan
```

This comprehensive cron syntax reference provides all the information needed to create effective scheduled jobs in the Rae Scheduler system. 
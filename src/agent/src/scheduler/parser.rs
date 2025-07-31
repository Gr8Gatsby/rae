//! Parser for cron syntax and trigger validation.
//! 
//! Provides cross-platform cron parsing with timezone support and
//! platform-appropriate trigger validation.

use crate::scheduler::job::{Schedule, EventTrigger, PatternTrigger, EventType, PatternType};
use chrono::{DateTime, Utc, TimeZone};
use std::str::FromStr;
use thiserror::Error;

/// Errors that can occur during parsing.
#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Invalid cron expression: {0}")]
    InvalidCronExpression(String),
    
    #[error("Invalid time format: {0}")]
    InvalidTimeFormat(String),
    
    #[error("Invalid timezone: {0}")]
    InvalidTimezone(String),
    
    #[error("Invalid event trigger: {0}")]
    InvalidEventTrigger(String),
    
    #[error("Invalid pattern trigger: {0}")]
    InvalidPatternTrigger(String),
}

/// Parser for job scheduling and triggers.
pub struct Parser;

impl Parser {
    /// Parses a cron expression and validates it.
    pub fn parse_cron(cron_expr: &str) -> Result<cron::Schedule, ParserError> {
        cron::Schedule::from_str(cron_expr)
            .map_err(|e| ParserError::InvalidCronExpression(e.to_string()))
    }
    
    /// Parses a time string in ISO 8601 format.
    pub fn parse_time(time_str: &str) -> Result<DateTime<Utc>, ParserError> {
        DateTime::parse_from_rfc3339(time_str)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| ParserError::InvalidTimeFormat(e.to_string()))
    }
    
    /// Parses a timezone string.
    pub fn parse_timezone(tz_str: &str) -> Result<chrono_tz::Tz, ParserError> {
        tz_str.parse::<chrono_tz::Tz>()
            .map_err(|e| ParserError::InvalidTimezone(e.to_string()))
    }
    
    /// Validates a schedule configuration.
    pub fn validate_schedule(schedule: &Schedule) -> Result<(), ParserError> {
        // Validate cron expression if present
        if let Some(cron_expr) = &schedule.cron {
            Self::parse_cron(cron_expr)?;
        }
        
        // Validate timezone if present
        if let Some(tz_str) = &schedule.timezone {
            Self::parse_timezone(tz_str)?;
        }
        
        // Validate event trigger if present
        if let Some(event) = &schedule.event {
            Self::validate_event_trigger(event)?;
        }
        
        // Validate pattern trigger if present
        if let Some(pattern) = &schedule.pattern {
            Self::validate_pattern_trigger(pattern)?;
        }
        
        Ok(())
    }
    
    /// Validates an event trigger configuration.
    pub fn validate_event_trigger(event: &EventTrigger) -> Result<(), ParserError> {
        match &event.event_type {
            EventType::FileCreated | EventType::FileModified | EventType::FileDeleted => {
                if event.path.is_none() {
                    return Err(ParserError::InvalidEventTrigger(
                        "File events require a path".to_string()
                    ));
                }
            }
            EventType::SystemStartup | EventType::SystemShutdown => {
                // System events don't require additional validation
            }
            EventType::Custom(_) => {
                // Custom events are always valid
            }
        }
        
        Ok(())
    }
    
    /// Validates a pattern trigger configuration.
    pub fn validate_pattern_trigger(pattern: &PatternTrigger) -> Result<(), ParserError> {
        if pattern.threshold < 0.0 {
            return Err(ParserError::InvalidPatternTrigger(
                "Threshold must be non-negative".to_string()
            ));
        }
        
        if pattern.window == 0 {
            return Err(ParserError::InvalidPatternTrigger(
                "Window must be greater than 0".to_string()
            ));
        }
        
        Ok(())
    }
    
    /// Gets the next execution time for a cron schedule.
    pub fn next_cron_execution(cron_expr: &str, after: DateTime<Utc>) -> Result<DateTime<Utc>, ParserError> {
        let schedule = Self::parse_cron(cron_expr)?;
        Ok(schedule.after(&after).next().unwrap_or(after))
    }
    
    /// Gets the next execution time for a schedule.
    pub fn next_execution(schedule: &Schedule, after: DateTime<Utc>) -> Result<Option<DateTime<Utc>>, ParserError> {
        // Check cron schedule
        if let Some(cron_expr) = &schedule.cron {
            return Ok(Some(Self::next_cron_execution(cron_expr, after)?));
        }
        
        // Check one-time schedule
        if let Some(at) = schedule.at {
            if at > after {
                return Ok(Some(at));
            }
        }
        
        // Event and pattern triggers don't have predictable next execution times
        Ok(None)
    }
    
    /// Formats a cron expression for display.
    pub fn format_cron(cron_expr: &str) -> Result<String, ParserError> {
        let schedule = Self::parse_cron(cron_expr)?;
        
        // Get the next few executions for display
        let now = Utc::now();
        let next_executions: Vec<DateTime<Utc>> = schedule
            .after(&now)
            .take(3)
            .collect();
        
        let executions_str = next_executions
            .iter()
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .collect::<Vec<_>>()
            .join(", ");
        
        Ok(format!("{} (next: {})", cron_expr, executions_str))
    }
    
    /// Parses a human-readable time string.
    pub fn parse_human_time(time_str: &str) -> Result<DateTime<Utc>, ParserError> {
        // Try common formats
        let formats = [
            "%Y-%m-%d %H:%M:%S",
            "%Y-%m-%d %H:%M",
            "%Y-%m-%d",
            "%H:%M:%S",
            "%H:%M",
        ];
        
        for format in &formats {
            if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(time_str, format) {
                return Ok(dt.and_utc());
            }
        }
        
        // Try relative time expressions
        if let Some(dt) = Self::parse_relative_time(time_str) {
            return Ok(dt);
        }
        
        Err(ParserError::InvalidTimeFormat(format!(
            "Could not parse time string: {}", time_str
        )))
    }
    
    /// Parses relative time expressions like "in 5 minutes", "tomorrow at 6pm".
    fn parse_relative_time(time_str: &str) -> Option<DateTime<Utc>> {
        let now = Utc::now();
        let time_str = time_str.to_lowercase();
        
        // Handle "in X minutes/hours/days"
        if time_str.starts_with("in ") {
            let parts: Vec<&str> = time_str[3..].split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(amount) = parts[0].parse::<i64>() {
                    match parts[1] {
                        "minute" | "minutes" => return Some(now + chrono::Duration::minutes(amount)),
                        "hour" | "hours" => return Some(now + chrono::Duration::hours(amount)),
                        "day" | "days" => return Some(now + chrono::Duration::days(amount)),
                        _ => {}
                    }
                }
            }
        }
        
        // Handle "tomorrow at X"
        if time_str.starts_with("tomorrow at ") {
            let time_part = &time_str[12..];
            if let Ok(naive_time) = chrono::NaiveTime::parse_from_str(time_part, "%H:%M") {
                let tomorrow = now.date_naive() + chrono::Duration::days(1);
                let naive_dt = chrono::NaiveDateTime::new(tomorrow, naive_time);
                return Some(naive_dt.and_utc());
            }
        }
        
        // Handle "today at X"
        if time_str.starts_with("today at ") {
            let time_part = &time_str[9..];
            if let Ok(naive_time) = chrono::NaiveTime::parse_from_str(time_part, "%H:%M") {
                let today = now.date_naive();
                let naive_dt = chrono::NaiveDateTime::new(today, naive_time);
                return Some(naive_dt.and_utc());
            }
        }
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::job::{EventTrigger, PatternTrigger};
    
    #[test]
    fn test_parse_cron() {
        let result = Parser::parse_cron("0 18 * * *");
        assert!(result.is_ok());
        
        let result = Parser::parse_cron("invalid");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_parse_time() {
        let result = Parser::parse_time("2024-01-01T18:00:00Z");
        assert!(result.is_ok());
        
        let result = Parser::parse_time("invalid");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_parse_human_time() {
        let result = Parser::parse_human_time("in 5 minutes");
        assert!(result.is_ok());
        
        let result = Parser::parse_human_time("tomorrow at 18:00");
        assert!(result.is_ok());
        
        let result = Parser::parse_human_time("invalid");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validate_event_trigger() {
        let event = EventTrigger {
            event_type: EventType::FileCreated,
            path: Some("/tmp/test".to_string()),
            filter: None,
        };
        assert!(Parser::validate_event_trigger(&event).is_ok());
        
        let event = EventTrigger {
            event_type: EventType::FileCreated,
            path: None,
            filter: None,
        };
        assert!(Parser::validate_event_trigger(&event).is_err());
    }
    
    #[test]
    fn test_validate_pattern_trigger() {
        let pattern = PatternTrigger {
            pattern_type: PatternType::HighCpuUsage,
            threshold: 80.0,
            window: 300,
        };
        assert!(Parser::validate_pattern_trigger(&pattern).is_ok());
        
        let pattern = PatternTrigger {
            pattern_type: PatternType::HighCpuUsage,
            threshold: -1.0,
            window: 300,
        };
        assert!(Parser::validate_pattern_trigger(&pattern).is_err());
    }
} 
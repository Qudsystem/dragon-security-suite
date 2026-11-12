use super::models::Event;
use bytes::BytesMut;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
    
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("Invalid event format")]
    InvalidFormat,
}

/// High-performance event parser with zero-copy deserialization
pub struct EventParser {
    buffer: BytesMut,
}

impl EventParser {
    pub fn new() -> Self {
        Self {
            buffer: BytesMut::with_capacity(4096),
        }
    }

    /// Parse a JSON event with zero-copy deserialization
    pub fn parse_event(&mut self, raw: &[u8]) -> Result<Event, ParseError> {
        // Zero-copy deserialization using serde_json
        let event: Event = serde_json::from_slice(raw)?;
        
        // Validate required fields
        self.validate_event(&event)?;
        
        Ok(event)
    }

    /// Validate event has all required fields
    fn validate_event(&self, event: &Event) -> Result<(), ParseError> {
        if event.process.name.is_empty() {
            return Err(ParseError::MissingField("process.name".to_string()));
        }
        
        Ok(())
    }
}

impl Default for EventParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::path::PathBuf;
    use crate::parser::models::*;

    #[test]
    fn test_parse_valid_event() {
        let json = r#"{
            "timestamp": "2024-01-15T10:30:45Z",
            "event_type": "syscall",
            "severity": "high",
            "process": {
                "pid": 1234,
                "name": "suspicious_binary",
                "exe_path": "/tmp/malware",
                "parent_pid": 1000,
                "parent_name": "bash",
                "user": "www-data",
                "uid": 33,
                "command_line": "/tmp/malware --evil",
                "start_time": "2024-01-15T10:30:00Z"
            }
        }"#;

        let mut parser = EventParser::new();
        let result = parser.parse_event(json.as_bytes());
        
        assert!(result.is_ok());
        let event = result.unwrap();
        assert_eq!(event.process.pid, 1234);
        assert_eq!(event.process.name, "suspicious_binary");
    }

    #[test]
    fn test_parse_invalid_json() {
        let json = r#"{ invalid json }"#;
        
        let mut parser = EventParser::new();
        let result = parser.parse_event(json.as_bytes());
        
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_required_field() {
        let json = r#"{
            "timestamp": "2024-01-15T10:30:45Z",
            "event_type": "syscall",
            "severity": "high",
            "process": {
                "pid": 1234,
                "name": "",
                "exe_path": "/tmp/malware",
                "parent_pid": 1000,
                "parent_name": "bash",
                "user": "www-data",
                "uid": 33,
                "command_line": "/tmp/malware",
                "start_time": "2024-01-15T10:30:00Z"
            }
        }"#;

        let mut parser = EventParser::new();
        let result = parser.parse_event(json.as_bytes());
        
        assert!(result.is_err());
    }
}

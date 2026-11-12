use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;
use std::path::PathBuf;
use std::net::IpAddr;

/// Main event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub severity: Severity,
    pub process: ProcessInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syscall: Option<SyscallInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<NetworkInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<FileInfo>,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Syscall,
    NetworkConnection,
    FileAccess,
    ProcessExecution,
    PrivilegeEscalation,
}

/// Severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Low,      // 0-30
    Medium,   // 31-60
    High,     // 61-85
    Critical, // 86-100
}

impl Severity {
    pub fn from_score(score: u8) -> Self {
        match score {
            0..=30 => Severity::Low,
            31..=60 => Severity::Medium,
            61..=85 => Severity::High,
            _ => Severity::Critical,
        }
    }

    pub fn to_score(&self) -> u8 {
        match self {
            Severity::Low => 15,
            Severity::Medium => 45,
            Severity::High => 73,
            Severity::Critical => 93,
        }
    }
}

/// Process information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub exe_path: PathBuf,
    pub parent_pid: u32,
    pub parent_name: String,
    pub user: String,
    pub uid: u32,
    pub command_line: String,
    pub start_time: DateTime<Utc>,
}

/// System call information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyscallInfo {
    pub name: String,
    pub args: HashMap<String, serde_json::Value>,
    pub return_code: i32,
}

/// Network connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub src_ip: IpAddr,
    pub src_port: u16,
    pub dst_ip: IpAddr,
    pub dst_port: u16,
    pub protocol: Protocol,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub connection_state: ConnectionState,
}

/// Network protocols
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Protocol {
    TCP,
    UDP,
    ICMP,
}

/// Connection states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ConnectionState {
    Established,
    Listen,
    Closed,
    TimeWait,
}

/// File access information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: PathBuf,
    pub mode: FileAccessMode,
    pub hash: Option<String>,
    pub size: Option<u64>,
}

/// File access modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileAccessMode {
    Read,
    Write,
    Execute,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_from_score() {
        assert_eq!(Severity::from_score(10), Severity::Low);
        assert_eq!(Severity::from_score(50), Severity::Medium);
        assert_eq!(Severity::from_score(75), Severity::High);
        assert_eq!(Severity::from_score(95), Severity::Critical);
    }

    #[test]
    fn test_event_serialization() {
        let event = Event {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: EventType::Syscall,
            severity: Severity::High,
            process: ProcessInfo {
                pid: 1234,
                name: "test".to_string(),
                exe_path: PathBuf::from("/bin/test"),
                parent_pid: 1,
                parent_name: "init".to_string(),
                user: "root".to_string(),
                uid: 0,
                command_line: "test --flag".to_string(),
                start_time: Utc::now(),
            },
            syscall: None,
            network: None,
            file: None,
            metadata: HashMap::new(),
        };

        let json = serde_json::to_string(&event).unwrap();
        let deserialized: Event = serde_json::from_str(&json).unwrap();
        
        assert_eq!(event.id, deserialized.id);
        assert_eq!(event.event_type, deserialized.event_type);
    }
}

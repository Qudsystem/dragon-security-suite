// Graph query helpers and common queries

/// Query to find attack chain from initial compromise
pub const ATTACK_CHAIN_QUERY: &str = r#"
    SELECT * FROM event
    WHERE id = $event_id
    FETCH ->triggered_by->process,
          ->triggered_by->process->connects_to->ip,
          ->triggered_by->process->accesses->file
"#;

/// Query to find all events in time range
pub const TIME_RANGE_QUERY: &str = r#"
    SELECT * FROM event
    WHERE timestamp >= $start_time AND timestamp <= $end_time
    ORDER BY timestamp ASC
"#;

/// Query to find related processes
pub const RELATED_PROCESSES_QUERY: &str = r#"
    SELECT * FROM process
    WHERE parent_pid = $pid OR pid = $parent_pid
"#;

/// Query to find network connections from process
pub const PROCESS_CONNECTIONS_QUERY: &str = r#"
    SELECT * FROM process
    WHERE pid = $pid
    FETCH ->connects_to->ip
"#;

/// Query to find file accesses by process
pub const PROCESS_FILE_ACCESS_QUERY: &str = r#"
    SELECT * FROM process
    WHERE pid = $pid
    FETCH ->accesses->file
"#;

/// Query to find suspicious patterns
pub const SUSPICIOUS_PATTERN_QUERY: &str = r#"
    SELECT * FROM event
    WHERE severity IN ['high', 'critical']
    AND timestamp >= $start_time
    ORDER BY timestamp DESC
    LIMIT 100
"#;

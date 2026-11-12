use crate::parser::models::Event;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GraphError {
    #[error("Database connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Query failed: {0}")]
    QueryFailed(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Graph database client for SurrealDB
pub struct GraphClient {
    db: Option<Surreal<Client>>,
    connection_string: String,
}

impl GraphClient {
    pub fn new(connection_string: String) -> Self {
        Self {
            db: None,
            connection_string,
        }
    }

    /// Connect to SurrealDB
    pub async fn connect(&mut self) -> Result<(), GraphError> {
        let db = Surreal::new::<Ws>(&self.connection_string)
            .await
            .map_err(|e| GraphError::ConnectionFailed(e.to_string()))?;

        // Sign in as root
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .map_err(|e| GraphError::ConnectionFailed(e.to_string()))?;

        // Select namespace and database
        db.use_ns("protect_x")
            .use_db("security")
            .await
            .map_err(|e| GraphError::ConnectionFailed(e.to_string()))?;

        self.db = Some(db);
        Ok(())
    }

    /// Store an event in the graph database
    pub async fn store_event(&self, event: &Event) -> Result<(), GraphError> {
        let db = self.db.as_ref()
            .ok_or_else(|| GraphError::ConnectionFailed("Not connected".to_string()))?;

        // Create process node
        let process_id = format!("process:{}", event.process.pid);
        
        let _: Option<serde_json::Value> = db
            .create(("process", event.process.pid.to_string()))
            .content(serde_json::json!({
                "pid": event.process.pid,
                "name": event.process.name,
                "exe_path": event.process.exe_path,
                "user": event.process.user,
                "uid": event.process.uid,
                "start_time": event.process.start_time,
            }))
            .await
            .map_err(|e| GraphError::QueryFailed(e.to_string()))?;

        // Create event node
        let _: Option<serde_json::Value> = db
            .create(("event", event.id.to_string()))
            .content(serde_json::json!({
                "id": event.id,
                "timestamp": event.timestamp,
                "event_type": event.event_type,
                "severity": event.severity,
            }))
            .await
            .map_err(|e| GraphError::QueryFailed(e.to_string()))?;

        // Create relationship: event -> process
        let _: Option<serde_json::Value> = db
            .query("RELATE $event->triggered_by->$process")
            .bind(("event", format!("event:{}", event.id)))
            .bind(("process", process_id.clone()))
            .await
            .map_err(|e| GraphError::QueryFailed(e.to_string()))?;

        // If network connection, create IP node and relationship
        if let Some(network) = &event.network {
            let ip_id = format!("ip:{}", network.dst_ip);
            
            let _: Option<serde_json::Value> = db
                .create(("ip", network.dst_ip.to_string()))
                .content(serde_json::json!({
                    "address": network.dst_ip,
                    "port": network.dst_port,
                    "protocol": network.protocol,
                }))
                .await
                .map_err(|e| GraphError::QueryFailed(e.to_string()))?;

            let _: Option<serde_json::Value> = db
                .query("RELATE $process->connects_to->$ip")
                .bind(("process", process_id.clone()))
                .bind(("ip", ip_id))
                .await
                .map_err(|e| GraphError::QueryFailed(e.to_string()))?;
        }

        // If file access, create file node and relationship
        if let Some(file) = &event.file {
            let file_id = format!("file:{}", file.path.display());
            
            let _: Option<serde_json::Value> = db
                .create(("file", file.path.to_string_lossy().to_string()))
                .content(serde_json::json!({
                    "path": file.path,
                    "mode": file.mode,
                    "hash": file.hash,
                    "size": file.size,
                }))
                .await
                .map_err(|e| GraphError::QueryFailed(e.to_string()))?;

            let _: Option<serde_json::Value> = db
                .query("RELATE $process->accesses->$file")
                .bind(("process", process_id))
                .bind(("file", file_id))
                .await
                .map_err(|e| GraphError::QueryFailed(e.to_string()))?;
        }

        Ok(())
    }

    /// Query attack chain for a given entity
    pub async fn query_attack_chain(&self, entity_id: &str) -> Result<Vec<Event>, GraphError> {
        let db = self.db.as_ref()
            .ok_or_else(|| GraphError::ConnectionFailed("Not connected".to_string()))?;

        // TODO: Implement graph traversal query
        // This would traverse the graph to find all related events
        
        Ok(vec![])
    }

    /// Get entity relationships
    pub async fn get_relationships(&self, entity_id: &str) -> Result<Vec<String>, GraphError> {
        let db = self.db.as_ref()
            .ok_or_else(|| GraphError::ConnectionFailed("Not connected".to_string()))?;

        // TODO: Implement relationship query
        
        Ok(vec![])
    }

    /// Get system state at a specific time
    pub async fn get_state_at_time(&self, timestamp: chrono::DateTime<chrono::Utc>) -> Result<Vec<Event>, GraphError> {
        let db = self.db.as_ref()
            .ok_or_else(|| GraphError::ConnectionFailed("Not connected".to_string()))?;

        // TODO: Implement time-travel query
        
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_client_creation() {
        let client = GraphClient::new("ws://localhost:8000".to_string());
        assert!(client.db.is_none());
    }

    // Note: Integration tests require running SurrealDB instance
    // These would be in tests/integration/ directory
}

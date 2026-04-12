use crate::core::error::{Result, SesError};
use crate::core::types::{ExecutionContext, JsonValue, NodeKind};
use crate::engine::executor::NodeExecutor;
use crate::models::node::{NodeConfig, NodeExecutionResult};
use async_trait::async_trait;
use std::sync::Arc;

/// Send email node
pub struct SendEmailNode;

#[async_trait]
impl NodeExecutor for SendEmailNode {
    fn kind(&self) -> NodeKind {
        NodeKind::System
    }

    fn node_type(&self) -> &str {
        "send_email"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, config: &NodeConfig) -> Result<NodeExecutionResult> {
        let recipient = input.get("recipient")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("recipient is required".to_string()))?;

        let subject = input.get("subject")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("subject is required".to_string()))?;

        let body = input.get("body")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Simulate sending email
        let output = serde_json::json!({
            "success": true,
            "messageId": uuid::Uuid::new_v4().to_string(),
            "recipient": recipient,
            "subject": subject,
            "sentAt": chrono::Utc::now().to_rfc3339()
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// Log record node
pub struct LogRecordNode;

#[async_trait]
impl NodeExecutor for LogRecordNode {
    fn kind(&self) -> NodeKind {
        NodeKind::System
    }

    fn node_type(&self) -> &str {
        "log_record"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, config: &NodeConfig) -> Result<NodeExecutionResult> {
        let level = input.get("level")
            .and_then(|v| v.as_str())
            .unwrap_or("INFO");

        let message = input.get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let data = input.get("data");

        // Log the message
        match level.to_uppercase().as_str() {
            "ERROR" => tracing::error!("Node log: {}", message),
            "WARN" => tracing::warn!("Node log: {}", message),
            "DEBUG" => tracing::debug!("Node log: {}", message),
            _ => tracing::info!("Node log: {}", message),
        }

        let output = serde_json::json!({
            "success": true,
            "level": level,
            "message": message,
            "loggedAt": chrono::Utc::now().to_rfc3339()
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// File upload node
pub struct FileUploadNode;

#[async_trait]
impl NodeExecutor for FileUploadNode {
    fn kind(&self) -> NodeKind {
        NodeKind::System
    }

    fn node_type(&self) -> &str {
        "file_upload"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, config: &NodeConfig) -> Result<NodeExecutionResult> {
        let file_name = input.get("fileName")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let file_type = input.get("fileType")
            .and_then(|v| v.as_str())
            .unwrap_or("application/octet-stream");

        // Simulate file upload
        let file_url = format!("https://storage.ses.local/files/{}", uuid::Uuid::new_v4());

        let output = serde_json::json!({
            "success": true,
            "fileUrl": file_url,
            "fileName": file_name,
            "fileType": file_type,
            "uploadedAt": chrono::Utc::now().to_rfc3339()
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// Dynamic page query node
pub struct DynamicPageQueryNode;

#[async_trait]
impl NodeExecutor for DynamicPageQueryNode {
    fn kind(&self) -> NodeKind {
        NodeKind::System
    }

    fn node_type(&self) -> &str {
        "dynamic_page_query"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, config: &NodeConfig) -> Result<NodeExecutionResult> {
        let page_id = input.get("pageId")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let condition = input.get("condition");

        let output = serde_json::json!({
            "pageData": {
                "pageId": page_id,
                "title": "Dynamic Page",
                "columns": [
                    {"key": "id", "title": "ID"},
                    {"key": "name", "title": "Name"},
                    {"key": "status", "title": "Status"}
                ],
                "data": [],
                "total": 0
            }
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// Register all system nodes
pub fn register_nodes(runtime: &mut crate::engine::executor::ExecutionRuntime) {
    runtime.register(Arc::new(SendEmailNode));
    runtime.register(Arc::new(LogRecordNode));
    runtime.register(Arc::new(FileUploadNode));
    runtime.register(Arc::new(DynamicPageQueryNode));
}

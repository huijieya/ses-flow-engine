use crate::core::error::{Result, SesError};
use crate::core::types::{ExecutionContext, JsonValue, NodeKind};
use crate::engine::executor::NodeExecutor;
use crate::models::node::{NodeConfig, NodeExecutionResult};
use async_trait::async_trait;
use std::sync::Arc;

/// Chute operate node
pub struct ChuteOperateNode;

#[async_trait]
impl NodeExecutor for ChuteOperateNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Device
    }

    fn node_type(&self) -> &str {
        "chute_operate"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, config: &NodeConfig) -> Result<NodeExecutionResult> {
        // Extract parameters
        let platform_id = input.get("platformId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("platformId is required".to_string()))?;

        let chute_ids = input.get("chuteIds")
            .and_then(|v| v.as_array())
            .ok_or_else(|| SesError::Validation("chuteIds is required".to_string()))?;

        let operation = input.get("operation")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("operation is required".to_string()))?;

        // Validate operation
        let valid_operations = ["open", "close", "forbidden", "enable"];
        if !valid_operations.contains(&operation) {
            return Err(SesError::Validation(format!("Invalid operation: {}", operation)));
        }

        // Simulate device operation
        let output = serde_json::json!({
            "success": true,
            "platformId": platform_id,
            "operation": operation,
            "processedChutes": chute_ids.len(),
            "failedChutes": []
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// Device command node
pub struct DeviceCommandNode;

#[async_trait]
impl NodeExecutor for DeviceCommandNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Device
    }

    fn node_type(&self) -> &str {
        "device_command"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, config: &NodeConfig) -> Result<NodeExecutionResult> {
        let device_id = input.get("deviceId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("deviceId is required".to_string()))?;

        let command_type = input.get("commandType")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("commandType is required".to_string()))?;

        let task_id = uuid::Uuid::new_v4().to_string();

        let output = serde_json::json!({
            "success": true,
            "taskId": task_id,
            "deviceId": device_id,
            "commandType": command_type,
            "status": "dispatched"
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// Set light node
pub struct DeviceSetLightNode;

#[async_trait]
impl NodeExecutor for DeviceSetLightNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Device
    }

    fn node_type(&self) -> &str {
        "device_set_light"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, config: &NodeConfig) -> Result<NodeExecutionResult> {
        let device_id = input.get("deviceId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("deviceId is required".to_string()))?;

        let light_config = input.get("lightConfig")
            .ok_or_else(|| SesError::Validation("lightConfig is required".to_string()))?;

        let output = serde_json::json!({
            "success": true,
            "deviceId": device_id,
            "lightStatus": light_config
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// Printer node
pub struct PrinterNode;

#[async_trait]
impl NodeExecutor for PrinterNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Device
    }

    fn node_type(&self) -> &str {
        "printer_print"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, config: &NodeConfig) -> Result<NodeExecutionResult> {
        let platform_id = input.get("platformId")
            .and_then(|v| v.as_str())
            .unwrap_or("default");

        let chute_id = input.get("chuteId")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let pack_id = input.get("packId")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let output = serde_json::json!({
            "success": true,
            "printResult": {
                "status": "printed",
                "platformId": platform_id,
                "chuteId": chute_id,
                "packId": pack_id,
                "timestamp": chrono::Utc::now().to_rfc3339()
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

/// Register all device nodes
pub fn register_nodes(runtime: &mut crate::engine::executor::ExecutionRuntime) {
    runtime.register(Arc::new(ChuteOperateNode));
    runtime.register(Arc::new(DeviceCommandNode));
    runtime.register(Arc::new(DeviceSetLightNode));
    runtime.register(Arc::new(PrinterNode));
}

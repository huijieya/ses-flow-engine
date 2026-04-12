use crate::core::error::{Result, SesError};
use crate::core::types::{ExecutionContext, JsonValue, NodeKind};
use crate::engine::executor::NodeExecutor;
use crate::models::node::{NodeConfig, NodeExecutionResult};
use async_trait::async_trait;
use std::sync::Arc;

/// Query chute info node
pub struct ChuteQueryNode;

#[async_trait]
impl NodeExecutor for ChuteQueryNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Data
    }

    fn node_type(&self) -> &str {
        "chute_query"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, config: &NodeConfig) -> Result<NodeExecutionResult> {
        let platform_id = input.get("platformId")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let chute_id = input.get("chuteId")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // Simulate database query
        let output = serde_json::json!({
            "chuteInfo": {
                "chuteId": chute_id,
                "platformId": platform_id,
                "status": "OPEN",
                "type": "NORMAL",
                "physicalStatus": {
                    "close": false,
                    "full": false,
                    "disable": false
                },
                "ioStatus": {
                    "input": true,
                    "output": false
                }
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

/// Query order info node
pub struct OrderQueryNode;

#[async_trait]
impl NodeExecutor for OrderQueryNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Data
    }

    fn node_type(&self) -> &str {
        "order_query"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, config: &NodeConfig) -> Result<NodeExecutionResult> {
        let wave_id = input.get("waveId")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let order_id = input.get("orderId")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let output = serde_json::json!({
            "orderInfo": {
                "orderId": order_id,
                "waveId": wave_id,
                "orderType": "CHUTE",
                "status": "STARTED",
                "priority": 1,
                "chuteId": "CH001",
                "details": [
                    {
                        "sku": "SKU001",
                        "barcode": "123456789",
                        "qty": 10,
                        "completedQty": 5
                    }
                ]
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

/// Create order node
pub struct OrderCreateNode;

#[async_trait]
impl NodeExecutor for OrderCreateNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Data
    }

    fn node_type(&self) -> &str {
        "order_create"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, config: &NodeConfig) -> Result<NodeExecutionResult> {
        let order_info = input.get("orderInfo")
            .ok_or_else(|| SesError::Validation("orderInfo is required".to_string()))?;

        let order_id = uuid::Uuid::new_v4().to_string();

        let output = serde_json::json!({
            "success": true,
            "orderId": order_id,
            "message": "Order created successfully"
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// Update order status node
pub struct OrderUpdateStatusNode;

#[async_trait]
impl NodeExecutor for OrderUpdateStatusNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Data
    }

    fn node_type(&self) -> &str {
        "order_update_status"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, config: &NodeConfig) -> Result<NodeExecutionResult> {
        let order_id = input.get("orderId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("orderId is required".to_string()))?;

        let status = input.get("status")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("status is required".to_string()))?;

        let output = serde_json::json!({
            "success": true,
            "orderId": order_id,
            "status": status
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// Query wave info node
pub struct WaveQueryNode;

#[async_trait]
impl NodeExecutor for WaveQueryNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Data
    }

    fn node_type(&self) -> &str {
        "wave_query"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, config: &NodeConfig) -> Result<NodeExecutionResult> {
        let wave_id = input.get("waveId")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let output = serde_json::json!({
            "waveInfo": {
                "waveId": wave_id,
                "waveName": format!("Wave {}", wave_id),
                "status": "STARTED",
                "priority": 1,
                "totalOrders": 100,
                "completedOrders": 45,
                "progress": 45.0
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

/// Register all data nodes
pub fn register_nodes(runtime: &mut crate::engine::executor::ExecutionRuntime) {
    runtime.register(Arc::new(ChuteQueryNode));
    runtime.register(Arc::new(OrderQueryNode));
    runtime.register(Arc::new(OrderCreateNode));
    runtime.register(Arc::new(OrderUpdateStatusNode));
    runtime.register(Arc::new(WaveQueryNode));
}

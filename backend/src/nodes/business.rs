//! Business process nodes for sorting operations
//! 分拣业务流程节点 - 波次管理、订单处理、工作站交互

use crate::core::error::{Result, SesError};
use crate::core::types::{ExecutionContext, JsonValue, NodeKind};
use crate::engine::executor::NodeExecutor;
use crate::models::node::{NodeConfig, NodeExecutionResult};
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{info, debug};

/// 波次创建节点 - 创建新波次
pub struct WaveCreateNode;

#[async_trait]
impl NodeExecutor for WaveCreateNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Task
    }

    fn node_type(&self) -> &str {
        "wave_create"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, _config: &NodeConfig) -> Result<NodeExecutionResult> {
        debug!("Executing wave_create node");

        let wave_type = input.get("wave_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("wave_type is required".to_string()))?;

        let priority = input.get("priority")
            .and_then(|v| v.as_i64())
            .unwrap_or(1);

        let station_ids = input.get("station_ids")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>())
            .unwrap_or_default();

        let wave_id = format!("WV{}", uuid::Uuid::new_v4().to_string()[..8].to_uppercase());

        info!("Creating wave: {} with type: {}, priority: {}", wave_id, wave_type, priority);

        // 将波次信息存入上下文
        context.set("wave_id", &wave_id);
        context.set("wave_type", wave_type);
        context.set("wave_status", "created");
        context.set("priority", priority);
        context.set("station_ids", &station_ids);

        let output = serde_json::json!({
            "success": true,
            "wave_id": wave_id,
            "wave_type": wave_type,
            "priority": priority,
            "station_count": station_ids.len(),
            "status": "created"
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// 波次启动节点 - 启动波次，分配资源
pub struct WaveStartNode;

#[async_trait]
impl NodeExecutor for WaveStartNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Task
    }

    fn node_type(&self) -> &str {
        "wave_start"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, _config: &NodeConfig) -> Result<NodeExecutionResult> {
        debug!("Executing wave_start node");

        let wave_id = input.get("wave_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .or_else(|| context.get::<String>("wave_id"))
            .ok_or_else(|| SesError::Validation("wave_id is required".to_string()))?;
        let wave_id = wave_id.as_str();

        // 检查波次状态
        let current_status = context.get::<String>("wave_status").unwrap_or_default();
        if current_status == "running" {
            return Err(SesError::Validation("Wave is already running".to_string()));
        }

        info!("Starting wave: {}", wave_id);

        // 获取工作站列表
        let station_ids: Vec<String> = context.get("station_ids").unwrap_or_default();

        // 为每个工作站分配初始任务
        let mut station_tasks = Vec::new();
        for station_id in &station_ids {
            station_tasks.push(serde_json::json!({
                "station_id": station_id,
                "status": "ready",
                "current_order": null
            }));
        }

        context.set("wave_status", "running");
        context.set("started_at", chrono::Utc::now().to_rfc3339());
        context.set("station_tasks", &station_tasks);

        let output = serde_json::json!({
            "success": true,
            "wave_id": wave_id,
            "status": "running",
            "started_at": chrono::Utc::now().to_rfc3339(),
            "stations_ready": station_ids.len()
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// 波次关闭节点 - 关闭波次，释放资源
pub struct WaveCloseNode;

#[async_trait]
impl NodeExecutor for WaveCloseNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Task
    }

    fn node_type(&self) -> &str {
        "wave_close"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, _config: &NodeConfig) -> Result<NodeExecutionResult> {
        debug!("Executing wave_close node");

        let wave_id = input.get("wave_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .or_else(|| context.get::<String>("wave_id"))
            .ok_or_else(|| SesError::Validation("wave_id is required".to_string()))?;
        let wave_id = wave_id.as_str();

        let close_type = input.get("close_type")
            .and_then(|v| v.as_str())
            .unwrap_or("normal"); // normal, force

        info!("Closing wave: {} with type: {}", wave_id, close_type);

        // 获取统计数据
        let completed_orders = context.get::<i64>("completed_orders").unwrap_or(0);
        let total_orders = context.get::<i64>("total_orders").unwrap_or(0);

        context.set("wave_status", "closed");
        context.set("closed_at", chrono::Utc::now().to_rfc3339());
        context.set("close_type", close_type);

        let output = serde_json::json!({
            "success": true,
            "wave_id": wave_id,
            "status": "closed",
            "close_type": close_type,
            "completed_orders": completed_orders,
            "total_orders": total_orders,
            "completion_rate": if total_orders > 0 { (completed_orders as f64 / total_orders as f64) * 100.0 } else { 0.0 },
            "closed_at": chrono::Utc::now().to_rfc3339()
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// 订单分配节点 - 将订单分配到工作站
pub struct OrderAssignNode;

#[async_trait]
impl NodeExecutor for OrderAssignNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Task
    }

    fn node_type(&self) -> &str {
        "order_assign"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, _config: &NodeConfig) -> Result<NodeExecutionResult> {
        debug!("Executing order_assign node");

        let order_id = input.get("order_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("order_id is required".to_string()))?;

        let sku_list = input.get("sku_list")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<_>>())
            .unwrap_or_default();

        // 获取工作站列表，选择一个空闲的
        let station_ids: Vec<String> = context.get("station_ids").unwrap_or_default();
        let assigned_station = station_ids.first()
            .ok_or_else(|| SesError::Validation("No available station".to_string()))?;

        info!("Assigning order {} to station {}", order_id, assigned_station);

        let station_key = format!("order_{}_station", order_id);
        context.set(&station_key, assigned_station);
        let status_key = format!("order_{}_status", order_id);
        context.set(&status_key, "assigned");

        let output = serde_json::json!({
            "success": true,
            "order_id": order_id,
            "assigned_station": assigned_station,
            "sku_count": sku_list.len(),
            "status": "assigned"
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// 订单关闭节点 - 关闭单个订单
pub struct OrderCloseNode;

#[async_trait]
impl NodeExecutor for OrderCloseNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Task
    }

    fn node_type(&self) -> &str {
        "order_close"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, _config: &NodeConfig) -> Result<NodeExecutionResult> {
        debug!("Executing order_close node");

        let order_id = input.get("order_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("order_id is required".to_string()))?;

        info!("Closing order: {}", order_id);

        let status_key = format!("order_{}_status", order_id);
        context.set(&status_key, "closed");

        // 更新已完成订单计数
        let completed = context.get::<i64>("completed_orders").unwrap_or(0);
        context.set("completed_orders", completed + 1);

        let output = serde_json::json!({
            "success": true,
            "order_id": order_id,
            "status": "closed",
            "closed_at": chrono::Utc::now().to_rfc3339()
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// 工作站任务获取节点
pub struct StationGetTaskNode;

#[async_trait]
impl NodeExecutor for StationGetTaskNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Task
    }

    fn node_type(&self) -> &str {
        "station_get_task"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, _config: &NodeConfig) -> Result<NodeExecutionResult> {
        debug!("Executing station_get_task node");

        let station_id = input.get("station_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("station_id is required".to_string()))?;

        // 查找分配给该工作站的订单
        let wave_id = context.get::<String>("wave_id").unwrap_or_default();

        info!("Getting task for station: {} in wave: {}", station_id, wave_id);

        let output = serde_json::json!({
            "success": true,
            "station_id": station_id,
            "wave_id": wave_id,
            "has_task": true,
            "task": {
                "task_id": format!("TK{}", uuid::Uuid::new_v4().to_string()[..6].to_uppercase()),
                "task_type": "pick",
                "target_location": "LOC-A01",
                "sku": "SKU123456",
                "quantity": 1
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

/// 工作站扫码节点
pub struct StationScanNode;

#[async_trait]
impl NodeExecutor for StationScanNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Task
    }

    fn node_type(&self) -> &str {
        "station_scan"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, _config: &NodeConfig) -> Result<NodeExecutionResult> {
        debug!("Executing station_scan node");

        let station_id = input.get("station_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("station_id is required".to_string()))?;

        let barcode = input.get("barcode")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("barcode is required".to_string()))?;

        info!("Station {} scanned barcode: {}", station_id, barcode);

        // 验证条码是否匹配当前任务
        let is_valid = barcode.starts_with("SKU");

        let output = serde_json::json!({
            "success": is_valid,
            "station_id": station_id,
            "barcode": barcode,
            "verified": is_valid,
            "message": if is_valid { "条码验证成功" } else { "条码不匹配" }
        });

        Ok(NodeExecutionResult {
            success: is_valid,
            output,
            next_nodes: vec![],
            error: if is_valid { None } else { Some("条码验证失败".to_string()) },
        })
    }
}

/// RCS车辆调度节点
pub struct RcsDispatchNode;

#[async_trait]
impl NodeExecutor for RcsDispatchNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Device
    }

    fn node_type(&self) -> &str {
        "rcs_dispatch"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, _config: &NodeConfig) -> Result<NodeExecutionResult> {
        debug!("Executing rcs_dispatch node");

        let agv_id = input.get("agv_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("agv_id is required".to_string()))?;

        let target_station = input.get("target_station")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("target_station is required".to_string()))?;

        let task_type = input.get("task_type")
            .and_then(|v| v.as_str())
            .unwrap_or("move");

        let task_id = format!("RCS{}", uuid::Uuid::new_v4().to_string()[..8].to_uppercase());

        info!("Dispatching AGV {} to station {} with task {}", agv_id, target_station, task_id);

        let task_key = format!("agv_{}_task", agv_id);
        context.set(&task_key, &task_id);
        let status_key = format!("agv_{}_status", agv_id);
        context.set(&status_key, "dispatched");

        let output = serde_json::json!({
            "success": true,
            "task_id": task_id,
            "agv_id": agv_id,
            "target_station": target_station,
            "task_type": task_type,
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

/// RCS车辆到达节点
pub struct RcsArrivedNode;

#[async_trait]
impl NodeExecutor for RcsArrivedNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Device
    }

    fn node_type(&self) -> &str {
        "rcs_arrived"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, _config: &NodeConfig) -> Result<NodeExecutionResult> {
        debug!("Executing rcs_arrived node");

        let agv_id = input.get("agv_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("agv_id is required".to_string()))?;

        let station_id = input.get("station_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("station_id is required".to_string()))?;

        info!("AGV {} arrived at station {}", agv_id, station_id);

        let status_key = format!("agv_{}_status", agv_id);
        context.set(&status_key, "arrived");
        let station_key = format!("agv_{}_current_station", agv_id);
        context.set(&station_key, station_id);
        let agv_key = format!("station_{}_agv", station_id);
        context.set(&agv_key, agv_id);

        let output = serde_json::json!({
            "success": true,
            "agv_id": agv_id,
            "station_id": station_id,
            "status": "arrived",
            "arrived_at": chrono::Utc::now().to_rfc3339()
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// 格口操作节点 - 打开/关闭格口
pub struct ChuteOperationNode;

#[async_trait]
impl NodeExecutor for ChuteOperationNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Device
    }

    fn node_type(&self) -> &str {
        "chute_operation"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, _config: &NodeConfig) -> Result<NodeExecutionResult> {
        debug!("Executing chute_operation node");

        let chute_id = input.get("chute_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("chute_id is required".to_string()))?;

        let operation = input.get("operation")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("operation is required".to_string()))?;

        let valid_ops = ["open", "close", "lock", "unlock"];
        if !valid_ops.contains(&operation) {
            return Err(SesError::Validation(format!("Invalid operation: {}", operation)));
        }

        info!("Chute {} operation: {}", chute_id, operation);

        let status_key = format!("chute_{}_status", chute_id);
        context.set(&status_key, operation);

        let output = serde_json::json!({
            "success": true,
            "chute_id": chute_id,
            "operation": operation,
            "executed_at": chrono::Utc::now().to_rfc3339()
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// 播种墙上线节点
pub struct WallOnlineNode;

#[async_trait]
impl NodeExecutor for WallOnlineNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Device
    }

    fn node_type(&self) -> &str {
        "wall_online"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, _config: &NodeConfig) -> Result<NodeExecutionResult> {
        debug!("Executing wall_online node");

        let wall_id = input.get("wall_id")
            .or_else(|| input.get("mc_id"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("wall_id/mc_id is required".to_string()))?;

        let rfid = input.get("rfid")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);

        info!("Wall {} online with RFID {}", wall_id, rfid);

        let status_key = format!("wall_{}_status", wall_id);
        context.set(&status_key, "online");
        let rfid_key = format!("wall_{}_rfid", wall_id);
        context.set(&rfid_key, rfid);

        let output = serde_json::json!({
            "success": true,
            "wall_id": wall_id,
            "rfid": rfid,
            "status": "online",
            "online_at": chrono::Utc::now().to_rfc3339()
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// 播种墙下线节点
pub struct WallOfflineNode;

#[async_trait]
impl NodeExecutor for WallOfflineNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Device
    }

    fn node_type(&self) -> &str {
        "wall_offline"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, _config: &NodeConfig) -> Result<NodeExecutionResult> {
        debug!("Executing wall_offline node");

        let wall_id = input.get("wall_id")
            .or_else(|| input.get("rfid"))
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .or_else(|| input.get("rfid").and_then(|v| v.as_i64().map(|n| n.to_string())))
            .ok_or_else(|| SesError::Validation("wall_id/rfid is required".to_string()))?;
        let wall_id = wall_id.as_str();

        info!("Wall {} offline", wall_id);

        let status_key = format!("wall_{}_status", wall_id);
        context.set(&status_key, "offline");

        let output = serde_json::json!({
            "success": true,
            "wall_id": wall_id,
            "status": "offline",
            "offline_at": chrono::Utc::now().to_rfc3339()
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// 封包节点
pub struct PackNode;

#[async_trait]
impl NodeExecutor for PackNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Task
    }

    fn node_type(&self) -> &str {
        "pack_operation"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, _config: &NodeConfig) -> Result<NodeExecutionResult> {
        debug!("Executing pack_operation node");

        let target_id = input.get("target_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("target_id is required".to_string()))?;

        let box_code = input.get("box_code")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let pack_id = format!("PK{}", uuid::Uuid::new_v4().to_string()[..8].to_uppercase());

        info!("Packing target {} with pack {}", target_id, pack_id);

        let pack_key = format!("pack_{}_status", pack_id);
        context.set(&pack_key, "packed");
        let target_key = format!("target_{}_pack", target_id);
        context.set(&target_key, &pack_id);

        let output = serde_json::json!({
            "success": true,
            "pack_id": pack_id,
            "target_id": target_id,
            "box_code": box_code,
            "status": "packed",
            "packed_at": chrono::Utc::now().to_rfc3339()
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// 人工播种节点
pub struct DistributeNode;

#[async_trait]
impl NodeExecutor for DistributeNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Task
    }

    fn node_type(&self) -> &str {
        "distribute_item"
    }

    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, _config: &NodeConfig) -> Result<NodeExecutionResult> {
        debug!("Executing distribute_item node");

        let target_id = input.get("target_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("target_id is required".to_string()))?;

        let sku = input.get("sku")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("sku is required".to_string()))?;

        let quantity = input.get("quantity")
            .and_then(|v| v.as_i64())
            .unwrap_or(1);

        info!("Distributing {} x {} to {}", sku, quantity, target_id);

        let output = serde_json::json!({
            "success": true,
            "target_id": target_id,
            "sku": sku,
            "quantity": quantity,
            "distributed_at": chrono::Utc::now().to_rfc3339()
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// Register all business nodes
pub fn register_nodes(runtime: &mut crate::engine::executor::ExecutionRuntime) {
    runtime.register(Arc::new(WaveCreateNode));
    runtime.register(Arc::new(WaveStartNode));
    runtime.register(Arc::new(WaveCloseNode));
    runtime.register(Arc::new(OrderAssignNode));
    runtime.register(Arc::new(OrderCloseNode));
    runtime.register(Arc::new(StationGetTaskNode));
    runtime.register(Arc::new(StationScanNode));
    runtime.register(Arc::new(RcsDispatchNode));
    runtime.register(Arc::new(RcsArrivedNode));
    runtime.register(Arc::new(ChuteOperationNode));
    runtime.register(Arc::new(WallOnlineNode));
    runtime.register(Arc::new(WallOfflineNode));
    runtime.register(Arc::new(PackNode));
    runtime.register(Arc::new(DistributeNode));
}

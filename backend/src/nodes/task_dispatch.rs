//! Task dispatch node - assign destination chute to order
//! 任务分配节点 - 为订单分配目的地格口并下发RCS任务

use crate::clients::rcs_client::{RcsTaskDispatchReq, RcsTaskParams};
use crate::core::error::{Result, SesError};
use crate::core::types::{ExecutionContext, JsonValue, NodeKind};
use crate::engine::executor::NodeExecutor;
use crate::models::node::{NodeConfig, NodeExecutionResult};
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

/// Task dispatch node - 任务分配节点
/// 1. 找到订单对应的波次和platform
/// 2. 从该platform的空闲格口中分配一个
/// 3. 绑定订单与格口
/// 4. 调用RCS下发任务
pub struct TaskDispatchNode {
    db_pool: Arc<PgPool>,
}

impl TaskDispatchNode {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }

    /// Find available chute for the order's platform
    async fn find_available_chute(
        &self,
        platform_id: &str,
        order_id: &str,
    ) -> Result<Option<(String, String)>> {
        // First check if order already has a chute assigned
        let existing: Option<(String,)> = sqlx::query_as(
            r#"
            SELECT chute_id FROM ses_order_chute_bindings
            WHERE order_id = $1 AND status = 'ACTIVE'
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(order_id)
        .fetch_optional(&*self.db_pool)
        .await
        .map_err(SesError::Database)?;

        if let Some((chute_id,)) = existing {
            info!("Order {} already has chute {} assigned", order_id, chute_id);
            return Ok(Some((chute_id, platform_id.to_string())));
        }

        // Find an available chute from the platform
        let available: Option<(String,)> = sqlx::query_as(
            r#"
            SELECT cg.chute_id 
            FROM ses_chute_grids cg
            LEFT JOIN ses_order_chute_bindings b 
                ON cg.chute_id = b.chute_id AND b.status = 'ACTIVE'
            WHERE cg.platform_id = $1 
                AND cg.grid_type = 'CHUTE'
                AND cg.status = 'OPEN'
                AND b.chute_id IS NULL
            ORDER BY cg.created_at ASC
            LIMIT 1
            "#,
        )
        .bind(platform_id)
        .fetch_optional(&*self.db_pool)
        .await
        .map_err(SesError::Database)?;

        Ok(available.map(|(chute_id,)| (chute_id, platform_id.to_string())))
    }

    /// Bind order to chute
    async fn bind_order_to_chute(
        &self,
        order_id: &str,
        chute_id: &str,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO ses_order_chute_bindings 
            (id, order_id, chute_id, status, created_at, updated_at)
            VALUES ($1, $2, $3, 'ACTIVE', NOW(), NOW())
            "#,
        )
        .bind(uuid::Uuid::new_v4())
        .bind(order_id)
        .bind(chute_id)
        .execute(&*self.db_pool)
        .await
        .map_err(SesError::Database)?;

        info!("Order {} bound to chute {}", order_id, chute_id);
        Ok(())
    }

    /// Get order details
    async fn get_order_details(&self, order_id: &str) -> Result<Option<OrderDetails>> {
        let result: Option<OrderDetails> = sqlx::query_as(
            r#"
            SELECT o.order_id, o.wave_id, w.platform_id, o.order_type
            FROM ses_orders o
            JOIN ses_waves w ON o.wave_id = w.wave_id
            WHERE o.order_id = $1
            "#,
        )
        .bind(order_id)
        .fetch_optional(&*self.db_pool)
        .await
        .map_err(SesError::Database)?;

        Ok(result)
    }

    /// Get station info for AGV
    async fn get_station_info(&self, station_id: &str) -> Result<Option<StationInfo>> {
        let result: Option<StationInfo> = sqlx::query_as(
            r#"
            SELECT station_id, platform_id, chute_id as current_chute
            FROM ses_stations
            WHERE station_id = $1
            "#,
        )
        .bind(station_id)
        .fetch_optional(&*self.db_pool)
        .await
        .map_err(SesError::Database)?;

        Ok(result)
    }

    /// Deduct order quantity
    async fn deduct_order_quantity(
        &self,
        order_id: &str,
        sku: &str,
        quantity: i32,
    ) -> Result<bool> {
        let result = sqlx::query(
            r#"
            UPDATE ses_order_details
            SET completed_qty = completed_qty + $1,
                updated_at = NOW()
            WHERE order_id = $2 AND sku = $3
                AND completed_qty + $1 <= qty
            RETURNING id
            "#,
        )
        .bind(quantity)
        .bind(order_id)
        .bind(sku)
        .fetch_optional(&*self.db_pool)
        .await
        .map_err(SesError::Database)?;

        Ok(result.is_some())
    }
}

#[derive(Debug, sqlx::FromRow)]
struct OrderDetails {
    order_id: String,
    wave_id: String,
    platform_id: String,
    order_type: String,
}

#[derive(Debug, sqlx::FromRow)]
struct StationInfo {
    station_id: String,
    platform_id: String,
    current_chute: Option<String>,
}

#[async_trait]
impl NodeExecutor for TaskDispatchNode {
    fn kind(&self) -> NodeKind {
        NodeKind::Task
    }

    fn node_type(&self) -> &str {
        "task_dispatch"
    }

    async fn execute(
        &self,
        input: JsonValue,
        context: &mut ExecutionContext,
        _config: &NodeConfig,
    ) -> Result<NodeExecutionResult> {
        debug!("Executing task_dispatch node");

        // Get input parameters
        let order_id = input
            .get("order_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("order_id is required".to_string()))?;

        let station_id = input
            .get("station_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("station_id is required".to_string()))?;

        let agv_id = input
            .get("agv_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| SesError::Validation("agv_id is required".to_string()))?;

        let sku = input.get("sku").and_then(|v| v.as_str());
        let quantity = input.get("quantity").and_then(|v| v.as_i64()).unwrap_or(1) as i32;

        info!(
            "Dispatching task for order {} at station {} with AGV {}",
            order_id, station_id, agv_id
        );

        // Get order details
        let order = self
            .get_order_details(order_id)
            .await?
            .ok_or_else(|| SesError::NotFound(format!("Order not found: {}", order_id)))?;

        // Find available chute
        let (chute_id, platform_id) = match self.find_available_chute(&order.platform_id, order_id).await? {
            Some((chute_id, platform_id)) => (chute_id, platform_id),
            None => {
                warn!("No available chute for order {} on platform {}", order_id, order.platform_id);
                return Ok(NodeExecutionResult {
                    success: false,
                    output: serde_json::json!({
                        "success": false,
                        "error": "No available chute"
                    }),
                    next_nodes: vec![],
                    error: Some("No available chute".to_string()),
                });
            }
        };

        // Check if we need to bind (new assignment)
        let existing_binding: Option<(String,)> = sqlx::query_as(
            "SELECT chute_id FROM ses_order_chute_bindings WHERE order_id = $1 AND status = 'ACTIVE'"
        )
        .bind(order_id)
        .fetch_optional(&*self.db_pool)
        .await
        .map_err(SesError::Database)?;

        if existing_binding.is_none() {
            // Bind order to chute
            self.bind_order_to_chute(order_id, &chute_id).await?;
        }

        // Deduct quantity if SKU provided
        let deduct_success = if let Some(sku) = sku {
            self.deduct_order_quantity(order_id, sku, quantity).await?
        } else {
            true
        };

        // Store in context
        context.set("order_id", order_id);
        context.set("chute_id", &chute_id);
        context.set("agv_id", agv_id);
        context.set("station_id", station_id);

        let output = serde_json::json!({
            "success": true,
            "order_id": order_id,
            "chute_id": chute_id,
            "platform_id": platform_id,
            "agv_id": agv_id,
            "station_id": station_id,
            "deduct_success": deduct_success,
            "dispatched_at": chrono::Utc::now().to_rfc3339()
        });

        Ok(NodeExecutionResult {
            success: true,
            output,
            next_nodes: vec![],
            error: None,
        })
    }
}

/// Register task dispatch node
pub fn register_task_dispatch_node(runtime: &mut crate::engine::executor::ExecutionRuntime, db_pool: Arc<PgPool>) {
    runtime.register(Arc::new(TaskDispatchNode::new(db_pool)));
}

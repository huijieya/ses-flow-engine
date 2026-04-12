use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Order model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Order {
    pub id: Uuid,
    pub order_id: String,
    pub app_id: Uuid,
    pub wave_id: String,
    pub order_type: OrderType,
    pub chute_id: Option<String>,
    pub status: OrderStatus,
    pub priority: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "order_type", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderType {
    Chute,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "order_status", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderStatus {
    UnStarted,
    Started,
    InProgress,
    Completed,
    Closed,
    AutoComplete,
    Cancelled,
}

/// Order detail model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrderDetail {
    pub id: Uuid,
    pub order_id: String,
    pub sku: String,
    pub barcode: String,
    pub qty: i32,
    pub completed_qty: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create order request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub order_id: String,
    pub wave_id: String,
    pub order_type: OrderType,
    pub chute_id: Option<String>,
    pub priority: Option<i32>,
    pub details: Vec<CreateOrderDetailRequest>,
}

/// Create order detail request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderDetailRequest {
    pub sku: String,
    pub barcode: String,
    pub qty: i32,
}

/// Update order request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderRequest {
    pub status: Option<OrderStatus>,
    pub chute_id: Option<String>,
    pub priority: Option<i32>,
}

/// Order response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub id: Uuid,
    pub order_id: String,
    pub wave_id: String,
    pub order_type: OrderType,
    pub chute_id: Option<String>,
    pub status: OrderStatus,
    pub priority: i32,
    pub created_at: DateTime<Utc>,
}

impl From<Order> for OrderResponse {
    fn from(order: Order) -> Self {
        Self {
            id: order.id,
            order_id: order.order_id,
            wave_id: order.wave_id,
            order_type: order.order_type,
            chute_id: order.chute_id,
            status: order.status,
            priority: order.priority,
            created_at: order.created_at,
        }
    }
}

/// Order detail response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderDetailResponse {
    pub id: Uuid,
    pub order_id: String,
    pub sku: String,
    pub barcode: String,
    pub qty: i32,
    pub completed_qty: i32,
}

impl From<OrderDetail> for OrderDetailResponse {
    fn from(detail: OrderDetail) -> Self {
        Self {
            id: detail.id,
            order_id: detail.order_id,
            sku: detail.sku,
            barcode: detail.barcode,
            qty: detail.qty,
            completed_qty: detail.completed_qty,
        }
    }
}

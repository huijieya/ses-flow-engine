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
    pub order_type: String,
    pub chute_id: Option<String>,
    pub status: String,
    pub priority: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderType {
    Chute,
    Wall,
}

impl OrderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderType::Chute => "CHUTE",
            OrderType::Wall => "WALL",
        }
    }
}

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

impl OrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStatus::UnStarted => "UN_STARTED",
            OrderStatus::Started => "STARTED",
            OrderStatus::InProgress => "IN_PROGRESS",
            OrderStatus::Completed => "COMPLETED",
            OrderStatus::Closed => "CLOSED",
            OrderStatus::AutoComplete => "AUTO_COMPLETE",
            OrderStatus::Cancelled => "CANCELLED",
        }
    }
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
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
    pub status: Option<String>,
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

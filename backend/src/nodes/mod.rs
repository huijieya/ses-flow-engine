pub mod device;
pub mod logic;
pub mod data;
pub mod system;
pub mod business;
pub mod task_dispatch;

use crate::engine::executor::ExecutionRuntime;
use std::sync::Arc;
use sqlx::PgPool;

/// Register all default node executors
pub fn register_default_nodes(runtime: &mut ExecutionRuntime) {
    device::register_nodes(runtime);
    logic::register_nodes(runtime);
    data::register_nodes(runtime);
    system::register_nodes(runtime);
    business::register_nodes(runtime);
}

/// Register all node executors with database pool
pub fn register_nodes_with_db(runtime: &mut ExecutionRuntime, db_pool: Arc<PgPool>) {
    register_default_nodes(runtime);
    task_dispatch::register_task_dispatch_node(runtime, db_pool);
}

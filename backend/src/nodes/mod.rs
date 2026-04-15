pub mod device;
pub mod logic;
pub mod data;
pub mod system;
pub mod business;

use crate::engine::executor::ExecutionRuntime;

/// Register all default node executors
pub fn register_default_nodes(runtime: &mut ExecutionRuntime) {
    device::register_nodes(runtime);
    logic::register_nodes(runtime);
    data::register_nodes(runtime);
    system::register_nodes(runtime);
    business::register_nodes(runtime);
}

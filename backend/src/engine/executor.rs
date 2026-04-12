use crate::core::error::{Result, SesError};
use crate::core::types::{ExecutionContext, JsonValue, NodeKind, RetryPolicy};
use crate::models::node::{NodeConfig, NodeExecutionResult};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

/// Trait for node executors
#[async_trait]
pub trait NodeExecutor: Send + Sync {
    /// Returns the kind of this node executor
    fn kind(&self) -> NodeKind;
    
    /// Returns the node type identifier
    fn node_type(&self) -> &str;
    
    /// Execute the node with given input and context
    async fn execute(&self, input: JsonValue, context: &mut ExecutionContext, config: &NodeConfig) -> Result<NodeExecutionResult>;
    
    /// Get retry policy for this node
    fn retry_policy(&self) -> RetryPolicy {
        RetryPolicy::default()
    }
    
    /// Validate input before execution
    fn validate_input(&self, _input: &JsonValue) -> Result<()> {
        // Default implementation - always valid
        Ok(())
    }
}

/// Registry of node executors
pub struct ExecutorRegistry {
    executors: HashMap<String, Arc<dyn NodeExecutor>>,
}

impl ExecutorRegistry {
    pub fn new() -> Self {
        Self {
            executors: HashMap::new(),
        }
    }

    /// Register a node executor
    pub fn register(&mut self, executor: Arc<dyn NodeExecutor>) {
        let node_type = executor.node_type().to_string();
        info!("Registering node executor: {}", node_type);
        self.executors.insert(node_type, executor);
    }

    /// Get an executor by node type
    pub fn get(&self, node_type: &str) -> Option<Arc<dyn NodeExecutor>> {
        self.executors.get(node_type).cloned()
    }

    /// Check if an executor exists
    pub fn has(&self, node_type: &str) -> bool {
        self.executors.contains_key(node_type)
    }

    /// List all registered executors
    pub fn list(&self) -> Vec<&str> {
        self.executors.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for ExecutorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Execution runtime for nodes
pub struct ExecutionRuntime {
    registry: ExecutorRegistry,
}

impl ExecutionRuntime {
    pub fn new() -> Self {
        Self {
            registry: ExecutorRegistry::new(),
        }
    }

    pub fn with_default_executors() -> Self {
        let mut runtime = Self::new();
        runtime.register_defaults();
        runtime
    }

    /// Register default node executors
    fn register_defaults(&mut self) {
        // These will be implemented in the nodes module
        // runtime.registry.register(Arc::new(ChuteOperateNode));
        // runtime.registry.register(Arc::new(DeviceCommandNode));
        // ... etc
    }

    /// Register a custom executor
    pub fn register(&mut self, executor: Arc<dyn NodeExecutor>) {
        self.registry.register(executor);
    }

    /// Execute a node with retry logic
    pub async fn execute(
        &self,
        node_type: &str,
        input: JsonValue,
        context: &mut ExecutionContext,
        config: &NodeConfig,
    ) -> Result<NodeExecutionResult> {
        let executor = self.registry.get(node_type).ok_or_else(|| {
            SesError::NodeExecution(format!("No executor found for node type: {}", node_type))
        })?;

        // Validate input
        executor.validate_input(&input)?;

        let retry_policy = config.retry_policy.clone().unwrap_or_else(|| executor.retry_policy());
        let mut last_error = None;

        for attempt in 0..=retry_policy.max_attempts {
            if attempt > 0 {
                let backoff = retry_policy.backoff_ms * 2u64.pow(attempt - 1);
                let backoff = backoff.min(retry_policy.max_backoff_ms);
                debug!("Retrying node {} after {}ms (attempt {})", node_type, backoff, attempt + 1);
                tokio::time::sleep(tokio::time::Duration::from_millis(backoff)).await;
            }

            match executor.execute(input.clone(), context, config).await {
                Ok(result) => {
                    if attempt > 0 {
                        info!("Node {} succeeded after {} retries", node_type, attempt);
                    }
                    return Ok(result);
                }
                Err(e) => {
                    error!("Node {} execution failed (attempt {}): {}", node_type, attempt + 1, e);
                    last_error = Some(e);
                }
            }
        }

        Err(SesError::NodeExecution(format!(
            "Node {} failed after {} attempts: {:?}",
            node_type,
            retry_policy.max_attempts + 1,
            last_error
        )))
    }
}

impl Default for ExecutionRuntime {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for node execution
pub mod utils {
    use super::*;

    /// Resolve template strings with context variables
    pub fn resolve_template(template: &str, context: &ExecutionContext) -> String {
        let mut result = template.to_string();
        
        // Simple template resolution: {{key.subkey}}
        for (key, value) in &context.data {
            let placeholder = format!("{{{{{}}}}}", key);
            let replacement = match value {
                JsonValue::String(s) => s.clone(),
                other => other.to_string(),
            };
            result = result.replace(&placeholder, &replacement);
        }
        
        result
    }

    /// Merge JSON values
    pub fn merge_json(base: &mut JsonValue, overlay: JsonValue) {
        match (base, overlay) {
            (JsonValue::Object(base_map), JsonValue::Object(overlay_map)) => {
                for (key, value) in overlay_map {
                    base_map.insert(key, value);
                }
            }
            _ => {}
        }
    }

    /// Extract value from context by path (e.g., "input.orderId")
    pub fn get_value_from_context(context: &ExecutionContext, path: &str) -> Option<JsonValue> {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return None;
        }

        let mut current = context.data.get(parts[0])?.clone();
        
        for part in &parts[1..] {
            match current {
                JsonValue::Object(map) => {
                    current = map.get(*part)?.clone();
                }
                _ => return None,
            }
        }
        
        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_resolve_template() {
        let mut context = ExecutionContext::new();
        context.set("orderId", "ORD123");
        context.set("waveId", "WV456");

        let template = "Processing order {{orderId}} in wave {{waveId}}";
        let result = utils::resolve_template(template, &context);
        
        assert_eq!(result, "Processing order ORD123 in wave WV456");
    }

    #[test]
    fn test_get_value_from_context() {
        let mut context = ExecutionContext::new();
        context.set("input", json!({
            "orderId": "ORD123",
            "sku": "SKU456"
        }));

        let value = utils::get_value_from_context(&context, "input.orderId");
        assert_eq!(value, Some(json!("ORD123")));
    }
}

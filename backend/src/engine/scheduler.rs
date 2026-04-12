use crate::core::error::{Result, SesError};
use crate::core::types::{ExecutionContext, ExecutionStatus};
use crate::engine::dag::{Dag, NodeId};
use std::collections::{HashMap, HashSet, VecDeque};

/// Execution scheduler for managing node execution order
pub struct ExecutionScheduler {
    dag: Dag,
    completed_nodes: HashSet<NodeId>,
    running_nodes: HashSet<NodeId>,
    failed_nodes: HashSet<NodeId>,
    /// Node priorities (higher = execute first)
    priorities: HashMap<NodeId, i32>,
}

impl ExecutionScheduler {
    pub fn new(dag: Dag) -> Self {
        Self {
            dag,
            completed_nodes: HashSet::new(),
            running_nodes: HashSet::new(),
            failed_nodes: HashSet::new(),
            priorities: HashMap::new(),
        }
    }

    /// Get the next batch of nodes that are ready to execute
    pub fn next_ready_nodes(&mut self) -> Vec<NodeId> {
        let mut ready = Vec::new();

        for node_id in self.dag.nodes() {
            // Skip if already processed
            if self.completed_nodes.contains(node_id)
                || self.running_nodes.contains(node_id)
                || self.failed_nodes.contains(node_id)
            {
                continue;
            }

            // Check if all dependencies are completed
            if self.are_dependencies_met(node_id) {
                ready.push(node_id.clone());
            }
        }

        // Sort by priority (higher first)
        ready.sort_by_key(|node_id| {
            std::cmp::Reverse(self.priorities.get(node_id).copied().unwrap_or(0))
        });

        // Mark as running
        for node_id in &ready {
            self.running_nodes.insert(node_id.clone());
        }

        ready
    }

    /// Mark a node as completed
    pub fn mark_completed(&mut self, node_id: NodeId) {
        self.running_nodes.remove(&node_id);
        self.completed_nodes.insert(node_id);
    }

    /// Mark a node as failed
    pub fn mark_failed(&mut self, node_id: NodeId) {
        self.running_nodes.remove(&node_id);
        self.failed_nodes.insert(node_id);
    }

    /// Mark a node as cancelled (can be retried)
    pub fn mark_cancelled(&mut self, node_id: NodeId) {
        self.running_nodes.remove(&node_id);
    }

    /// Check if all dependencies for a node are met
    fn are_dependencies_met(&self, node_id: &NodeId) -> bool {
        match self.dag.incoming(node_id) {
            None => true,
            Some(incoming) => {
                incoming.iter().all(|dep| self.completed_nodes.contains(dep))
            }
        }
    }

    /// Set priority for a node
    pub fn set_priority(&mut self, node_id: NodeId, priority: i32) {
        self.priorities.insert(node_id, priority);
    }

    /// Check if execution is complete
    pub fn is_complete(&self) -> bool {
        self.completed_nodes.len() + self.failed_nodes.len() == self.dag.nodes().len()
    }

    /// Check if execution succeeded (all nodes completed)
    pub fn is_success(&self) -> bool {
        self.completed_nodes.len() == self.dag.nodes().len()
    }

    /// Get execution progress (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        let total = self.dag.nodes().len();
        if total == 0 {
            return 1.0;
        }
        let completed = self.completed_nodes.len() + self.failed_nodes.len();
        completed as f64 / total as f64
    }

    /// Get current execution status
    pub fn status(&self) -> ExecutionStatus {
        if self.is_success() {
            ExecutionStatus::Completed
        } else if !self.failed_nodes.is_empty() {
            ExecutionStatus::Failed
        } else if self.is_complete() {
            ExecutionStatus::Completed
        } else {
            ExecutionStatus::Running
        }
    }

    /// Get list of failed nodes
    pub fn failed_nodes(&self) -> &HashSet<NodeId> {
        &self.failed_nodes
    }

    /// Get list of completed nodes
    pub fn completed_nodes(&self) -> &HashSet<NodeId> {
        &self.completed_nodes
    }

    /// Get list of running nodes
    pub fn running_nodes(&self) -> &HashSet<NodeId> {
        &self.running_nodes
    }

    /// Reset scheduler state
    pub fn reset(&mut self) {
        self.completed_nodes.clear();
        self.running_nodes.clear();
        self.failed_nodes.clear();
    }

    /// Get execution path from start to end nodes
    pub fn execution_path(&self, start: &NodeId, end: &NodeId) -> Option<Vec<NodeId>> {
        // BFS to find shortest path
        let mut queue: VecDeque<(NodeId, Vec<NodeId>)> = VecDeque::new();
        let mut visited: HashSet<NodeId> = HashSet::new();

        queue.push_back((start.clone(), vec![start.clone()]));
        visited.insert(start.clone());

        while let Some((current, path)) = queue.pop_front() {
            if &current == end {
                return Some(path);
            }

            if let Some(neighbors) = self.dag.outgoing(&current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        let mut new_path = path.clone();
                        new_path.push(neighbor.clone());
                        queue.push_back((neighbor.clone(), new_path));
                    }
                }
            }
        }

        None
    }

    /// Get parallel execution levels (nodes that can execute in parallel)
    pub fn parallel_levels(&self) -> Vec<Vec<NodeId>> {
        let mut levels: Vec<Vec<NodeId>> = Vec::new();
        let mut remaining: HashSet<NodeId> = self.dag.nodes().iter().cloned().collect();
        let mut completed: HashSet<NodeId> = HashSet::new();

        while !remaining.is_empty() {
            let mut current_level = Vec::new();

            for node_id in &remaining {
                let deps_met = match self.dag.incoming(node_id) {
                    None => true,
                    Some(incoming) => incoming.iter().all(|dep| completed.contains(dep)),
                };

                if deps_met {
                    current_level.push(node_id.clone());
                }
            }

            if current_level.is_empty() {
                // Cycle or stuck
                break;
            }

            for node_id in &current_level {
                remaining.remove(node_id);
                completed.insert(node_id.clone());
            }

            levels.push(current_level);
        }

        levels
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::dag::Dag;

    fn create_test_dag() -> Dag {
        let mut dag = Dag::new();
        dag.add_node("A".to_string()).unwrap();
        dag.add_node("B".to_string()).unwrap();
        dag.add_node("C".to_string()).unwrap();
        dag.add_node("D".to_string()).unwrap();
        dag.add_node("E".to_string()).unwrap();

        dag.add_edge("A".to_string(), "B".to_string(), None).unwrap();
        dag.add_edge("A".to_string(), "C".to_string(), None).unwrap();
        dag.add_edge("B".to_string(), "D".to_string(), None).unwrap();
        dag.add_edge("C".to_string(), "D".to_string(), None).unwrap();
        dag.add_edge("D".to_string(), "E".to_string(), None).unwrap();

        dag
    }

    #[test]
    fn test_scheduler_basic() {
        let dag = create_test_dag();
        let mut scheduler = ExecutionScheduler::new(dag);

        // First batch should be A
        let ready = scheduler.next_ready_nodes();
        assert_eq!(ready, vec!["A"]);

        // Complete A
        scheduler.mark_completed("A".to_string());

        // Next batch should be B and C
        let ready = scheduler.next_ready_nodes();
        assert!(ready.contains(&"B".to_string()));
        assert!(ready.contains(&"C".to_string()));
    }

    #[test]
    fn test_parallel_levels() {
        let dag = create_test_dag();
        let scheduler = ExecutionScheduler::new(dag);

        let levels = scheduler.parallel_levels();
        assert_eq!(levels.len(), 4);
        assert_eq!(levels[0], vec!["A"]);
        assert!(levels[1].contains(&"B".to_string()));
        assert!(levels[1].contains(&"C".to_string()));
        assert_eq!(levels[2], vec!["D"]);
        assert_eq!(levels[3], vec!["E"]);
    }
}

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use thiserror::Error;

pub type NodeId = String;

/// Edge in the DAG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub from: NodeId,
    pub to: NodeId,
    pub condition: Option<String>,
}

/// DAG (Directed Acyclic Graph) for flow execution
#[derive(Debug, Clone, Default)]
pub struct Dag {
    nodes: HashSet<NodeId>,
    edges: Vec<Edge>,
    /// Adjacency list: node -> outgoing edges
    adjacency: HashMap<NodeId, Vec<NodeId>>,
    /// Reverse adjacency: node -> incoming edges
    reverse_adjacency: HashMap<NodeId, Vec<NodeId>>,
}

#[derive(Error, Debug)]
pub enum DagError {
    #[error("Node not found: {0}")]
    NodeNotFound(NodeId),
    #[error("Edge already exists: {0} -> {1}")]
    EdgeAlreadyExists(NodeId, NodeId),
    #[error("Cycle detected in DAG")]
    CycleDetected,
    #[error("Node already exists: {0}")]
    NodeAlreadyExists(NodeId),
}

impl Dag {
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: Vec::new(),
            adjacency: HashMap::new(),
            reverse_adjacency: HashMap::new(),
        }
    }

    /// Add a node to the DAG
    pub fn add_node(&mut self, node_id: NodeId) -> Result<(), DagError> {
        if self.nodes.contains(&node_id) {
            return Err(DagError::NodeAlreadyExists(node_id));
        }
        self.adjacency.entry(node_id.clone()).or_default();
        self.reverse_adjacency.entry(node_id.clone()).or_default();
        self.nodes.insert(node_id);
        Ok(())
    }

    /// Add multiple nodes
    pub fn add_nodes(&mut self, node_ids: Vec<NodeId>) -> Result<(), DagError> {
        for node_id in node_ids {
            self.add_node(node_id)?;
        }
        Ok(())
    }

    /// Add an edge between two nodes
    pub fn add_edge(&mut self, from: NodeId, to: NodeId, condition: Option<String>) -> Result<(), DagError> {
        if !self.nodes.contains(&from) {
            return Err(DagError::NodeNotFound(from));
        }
        if !self.nodes.contains(&to) {
            return Err(DagError::NodeNotFound(to));
        }

        // Check if edge already exists
        if self.adjacency.get(&from).map_or(false, |v| v.contains(&to)) {
            return Err(DagError::EdgeAlreadyExists(from, to));
        }

        // Temporarily add edge to check for cycles
        self.adjacency.entry(from.clone()).or_default().push(to.clone());
        self.reverse_adjacency.entry(to.clone()).or_default().push(from.clone());

        if self.has_cycle() {
            // Remove the edge
            if let Some(v) = self.adjacency.get_mut(&from) {
                v.retain(|n| n != &to);
            }
            if let Some(v) = self.reverse_adjacency.get_mut(&to) {
                v.retain(|n| n != &from);
            }
            return Err(DagError::CycleDetected);
        }

        self.edges.push(Edge { from, to, condition });
        Ok(())
    }

    /// Get all nodes
    pub fn nodes(&self) -> &HashSet<NodeId> {
        &self.nodes
    }

    /// Get all edges
    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    /// Get outgoing neighbors
    pub fn outgoing(&self, node_id: &NodeId) -> Option<&Vec<NodeId>> {
        self.adjacency.get(node_id)
    }

    /// Get incoming neighbors
    pub fn incoming(&self, node_id: &NodeId) -> Option<&Vec<NodeId>> {
        self.reverse_adjacency.get(node_id)
    }

    /// Check if adding an edge would create a cycle
    fn has_cycle(&self) -> bool {
        let mut in_degree: HashMap<NodeId, usize> = HashMap::new();
        
        // Calculate in-degrees
        for node in &self.nodes {
            in_degree.entry(node.clone()).or_insert(0);
        }
        for neighbors in self.adjacency.values() {
            for neighbor in neighbors {
                *in_degree.entry(neighbor.clone()).or_insert(0) += 1;
            }
        }

        // Kahn's algorithm
        let mut queue: VecDeque<NodeId> = in_degree
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(node, _)| node.clone())
            .collect();

        let mut visited = 0;

        while let Some(node) = queue.pop_front() {
            visited += 1;
            if let Some(neighbors) = self.adjacency.get(&node) {
                for neighbor in neighbors {
                    let degree = in_degree.get_mut(neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        visited != self.nodes.len()
    }

    /// Get root nodes (no incoming edges)
    pub fn root_nodes(&self) -> Vec<NodeId> {
        self.nodes
            .iter()
            .filter(|node| {
                self.reverse_adjacency
                    .get(*node)
                    .map_or(true, |v| v.is_empty())
            })
            .cloned()
            .collect()
    }

    /// Get leaf nodes (no outgoing edges)
    pub fn leaf_nodes(&self) -> Vec<NodeId> {
        self.nodes
            .iter()
            .filter(|node| {
                self.adjacency
                    .get(*node)
                    .map_or(true, |v| v.is_empty())
            })
            .cloned()
            .collect()
    }

    /// Topological sort
    pub fn topological_sort(&self) -> Result<Vec<NodeId>, DagError> {
        let mut in_degree: HashMap<NodeId, usize> = HashMap::new();
        let mut result = Vec::new();
        
        // Calculate in-degrees
        for node in &self.nodes {
            in_degree.entry(node.clone()).or_insert(0);
        }
        for neighbors in self.adjacency.values() {
            for neighbor in neighbors {
                *in_degree.entry(neighbor.clone()).or_insert(0) += 1;
            }
        }

        // Kahn's algorithm
        let mut queue: VecDeque<NodeId> = in_degree
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(node, _)| node.clone())
            .collect();

        while let Some(node) = queue.pop_front() {
            result.push(node.clone());
            if let Some(neighbors) = self.adjacency.get(&node) {
                for neighbor in neighbors {
                    let degree = in_degree.get_mut(neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        if result.len() != self.nodes.len() {
            return Err(DagError::CycleDetected);
        }

        Ok(result)
    }

    /// Get all paths from a start node to an end node
    pub fn paths(&self, start: &NodeId, end: &NodeId) -> Vec<Vec<NodeId>> {
        let mut all_paths = Vec::new();
        let mut current_path = vec![start.clone()];
        self.dfs_paths(start, end, &mut current_path, &mut all_paths);
        all_paths
    }

    fn dfs_paths(
        &self,
        current: &NodeId,
        end: &NodeId,
        current_path: &mut Vec<NodeId>,
        all_paths: &mut Vec<Vec<NodeId>>,
    ) {
        if current == end {
            all_paths.push(current_path.clone());
            return;
        }

        if let Some(neighbors) = self.adjacency.get(current) {
            for neighbor in neighbors {
                if !current_path.contains(neighbor) {
                    current_path.push(neighbor.clone());
                    self.dfs_paths(neighbor, end, current_path, all_paths);
                    current_path.pop();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dag_basic() {
        let mut dag = Dag::new();
        dag.add_node("A".to_string()).unwrap();
        dag.add_node("B".to_string()).unwrap();
        dag.add_node("C".to_string()).unwrap();
        
        dag.add_edge("A".to_string(), "B".to_string(), None).unwrap();
        dag.add_edge("B".to_string(), "C".to_string(), None).unwrap();

        assert_eq!(dag.nodes.len(), 3);
        assert_eq!(dag.edges.len(), 2);
        
        let roots = dag.root_nodes();
        assert_eq!(roots, vec!["A"]);
        
        let leaves = dag.leaf_nodes();
        assert_eq!(leaves, vec!["C"]);
    }

    #[test]
    fn test_dag_cycle_detection() {
        let mut dag = Dag::new();
        dag.add_node("A".to_string()).unwrap();
        dag.add_node("B".to_string()).unwrap();
        dag.add_node("C".to_string()).unwrap();
        
        dag.add_edge("A".to_string(), "B".to_string(), None).unwrap();
        dag.add_edge("B".to_string(), "C".to_string(), None).unwrap();
        
        // This should fail - creates cycle
        let result = dag.add_edge("C".to_string(), "A".to_string(), None);
        assert!(matches!(result, Err(DagError::CycleDetected)));
    }

    #[test]
    fn test_topological_sort() {
        let mut dag = Dag::new();
        dag.add_node("A".to_string()).unwrap();
        dag.add_node("B".to_string()).unwrap();
        dag.add_node("C".to_string()).unwrap();
        dag.add_node("D".to_string()).unwrap();
        
        dag.add_edge("A".to_string(), "B".to_string(), None).unwrap();
        dag.add_edge("A".to_string(), "C".to_string(), None).unwrap();
        dag.add_edge("B".to_string(), "D".to_string(), None).unwrap();
        dag.add_edge("C".to_string(), "D".to_string(), None).unwrap();

        let sorted = dag.topological_sort().unwrap();
        assert_eq!(sorted.len(), 4);
        assert_eq!(sorted[0], "A");
        assert_eq!(sorted[3], "D");
    }
}

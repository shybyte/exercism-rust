use graph::graph_items::node::Node;
use graph::graph_items::edge::Edge;
use std::collections::HashMap;

pub mod graph_items {
    pub mod edge;
    pub mod node;
}

#[derive(Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
        self.nodes = nodes.to_vec();
        self
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges = edges.to_vec();
        self
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs = attrs.iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self
    }
}

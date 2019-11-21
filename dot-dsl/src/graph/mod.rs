pub mod graph_items;

use graph_items::{edge::Edge, node::Node};
use std::collections::HashMap;


pub struct Graph {
  pub nodes: Vec<Node>,
  pub edges: Vec<Edge>,
  pub attrs: HashMap<String, String>
}

impl Graph {
  pub fn new() -> Self {
      Graph {
          nodes: Vec::new(),
          edges: Vec::new(),
          attrs: HashMap::new()
      }
  }

  pub fn with_nodes(self, nodes: &[Node]) -> Self {
      Graph {
          nodes: nodes.to_vec(),
          ..self
      }
  }

  pub fn with_edges(self, edges: &[Edge]) -> Self {
      Graph {
          edges: edges.to_vec(),
          ..self
      }
  }

  pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
      Graph {
          attrs: attrs
              .iter()
              .map(|(k,v)| (String::from(*k), String::from(*v)))
              .collect(),
          ..self
      }
  }

  pub fn get_node(&self, node_name: &str) -> Option<&Node> {
      self.nodes.iter().find(|node| node.get_name() == node_name)
  }
}

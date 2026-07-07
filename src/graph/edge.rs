use crate::graph::types::{EdgeType, NodeId};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub source: NodeId,
    pub target: NodeId,
    pub edge_type: EdgeType,
    pub metadata: Map<String, Value>,
}

impl Edge {
    pub fn new(
        source: NodeId,
        target: NodeId,
        edge_type: EdgeType,
        metadata: Map<String, Value>,
    ) -> Self {
        Self {
            source,
            target,
            edge_type,
            metadata,
        }
    }
}

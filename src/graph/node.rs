use crate::graph::types::{NodeId, NodeType};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub canonical_identifier: String,
    pub name: String,
    pub node_type: NodeType,
    pub source_file: Option<String>,
    pub metadata: Map<String, Value>,
}

impl Node {
    pub fn new(
        id: NodeId,
        canonical_identifier: String,
        name: String,
        node_type: NodeType,
        source_file: Option<String>,
        metadata: Map<String, Value>,
    ) -> Self {
        Self {
            id,
            canonical_identifier,
            name,
            node_type,
            source_file,
            metadata,
        }
    }
}

pub fn generate_canonical_id(
    repo: &str,
    package: &str,
    file_path: &str,
    symbol: Option<&str>,
) -> String {
    let base = format!("git2okf://{}/{}/{}", repo, package, file_path);
    if let Some(sym) = symbol {
        format!("{}#{}", base, sym)
    } else {
        base
    }
}

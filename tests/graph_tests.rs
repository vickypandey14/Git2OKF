#[test]
fn test_node_edge_serialization() {
    use git2okf::graph::{generate_canonical_id, Edge, EdgeType, Node, NodeId, NodeType};
    use serde_json::{json, Map, Value};

    let mut metadata = Map::new();
    metadata.insert("visibility".to_string(), Value::String("public".to_string()));
    metadata.insert("line_range".to_string(), json!([10, 25]));
    metadata.insert("is_async".to_string(), Value::Bool(true));

    let canonical_id = generate_canonical_id("my-repo", "core", "src/lib.rs", Some("MyStruct"));
    assert_eq!(canonical_id, "git2okf://my-repo/core/src/lib.rs#MyStruct");

    let node = Node::new(
        NodeId(42),
        canonical_id.clone(),
        "MyStruct".to_string(),
        NodeType::Struct,
        Some("src/lib.rs".to_string()),
        metadata.clone(),
    );

    let serialized_node = serde_json::to_string(&node).unwrap();
    let deserialized_node: Node = serde_json::from_str(&serialized_node).unwrap();

    assert_eq!(deserialized_node.id, NodeId(42));
    assert_eq!(deserialized_node.canonical_identifier, canonical_id);
    assert_eq!(deserialized_node.name, "MyStruct");
    assert_eq!(
        deserialized_node.metadata.get("visibility").unwrap().as_str().unwrap(),
        "public"
    );
    assert_eq!(
        deserialized_node.metadata.get("is_async").unwrap().as_bool().unwrap(),
        true
    );

    let edge = Edge::new(NodeId(42), NodeId(43), EdgeType::Calls, Map::new());
    let serialized_edge = serde_json::to_string(&edge).unwrap();
    let deserialized_edge: Edge = serde_json::from_str(&serialized_edge).unwrap();

    assert_eq!(deserialized_edge.source, NodeId(42));
    assert_eq!(deserialized_edge.target, NodeId(43));
    assert_eq!(deserialized_edge.edge_type, EdgeType::Calls);
}

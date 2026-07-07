use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeType {
    Repository,
    Workspace,
    Package,
    Crate,
    Directory,
    File,
    Namespace,
    Interface,
    Trait,
    Struct,
    Class,
    Enum,
    Function,
    Method,
    Macro,
    TypeAlias,
    Constant,
    Import,
    Dependency,
    Language,
    Framework,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EdgeType {
    Calls,
    Imports,
    Declares,
    Uses,
    Extends,
    Implements,
    DependsOn,
    BelongsTo,
    Contains,
    References,
    Overrides,
    Exports,
    InstanceOf,
    AnnotatedWith,
}

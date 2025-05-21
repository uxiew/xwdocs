use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct IndexEntry {
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub entry_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct IndexType {
    pub name: String,
    pub count: usize,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullIndex {
    pub entries: Vec<IndexEntry>,
    pub types: Vec<IndexType>,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileEvent {
    pub file_id: String,
    pub file_hash: String,
    pub version: u64,
    pub actor: String,
    pub timestamp: i64,
    pub action: String, // CREATE | MODIFY | DELETE
    pub signature: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub prev_hash: String,
    pub timestamp: i64,
    pub events: Vec<FileEvent>,
    pub merkle_root: String,
    pub block_hash: String,
}

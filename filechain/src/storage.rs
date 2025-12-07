use crate::models::Block;
use sled::Db;

pub struct Storage {
    db: Db,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            db: sled::open("filechain.db").unwrap(),
        }
    }

    pub fn save_block(&self, block: &Block) {
        let key = block.index.to_be_bytes();
        let val = serde_json::to_vec(block).unwrap();
        self.db.insert(key, val).unwrap();
    }
}

use crate::{
    crypto,
    models::{Block, FileEvent},
};
use time::OffsetDateTime;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: vec![Self::genesis()],
        }
    }

    fn genesis() -> Block {
        Block {
            index: 0,
            prev_hash: "0".repeat(64),
            timestamp: OffsetDateTime::now_utc().unix_timestamp(),
            events: vec![],
            merkle_root: "0".repeat(64),
            block_hash: "GENESIS".into(),
        }
    }

    pub fn add_block(&mut self, events: Vec<FileEvent>) {
        let prev = self.chain.last().unwrap();
        let merkle = crypto::hash_events(&events);

        let mut block = Block {
            index: prev.index + 1,
            prev_hash: prev.block_hash.clone(),
            timestamp: OffsetDateTime::now_utc().unix_timestamp(),
            events,
            merkle_root: merkle.clone(),
            block_hash: String::new(),
        };

        let input = format!(
            "{}{}{}{}",
            block.index, block.prev_hash, block.timestamp, block.merkle_root
        );
        block.block_hash = crypto::hash_str(&input);

        self.chain.push(block);
    }

    pub fn verify_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            if self.chain[i].prev_hash != self.chain[i - 1].block_hash {
                return false;
            }
        }
        true
    }
}

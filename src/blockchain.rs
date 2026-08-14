use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub index: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u64,
    pub data: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, timestamp: u64, data: String) -> Self {
        let hash = calculate_hash(index, &previous_hash, timestamp, &data);

        Block {
            index,
            hash,
            previous_hash,
            timestamp,
            data,
        }
    }
}

pub fn calculate_hash(index: u64, previous_hash: &str, timestamp: u64, data: &str) -> String {
    let input = format!("{}{}{}{}", index, previous_hash, timestamp, data);
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());

    hex::encode(hasher.finalize())
}

#[derive(Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block {
            index: 0,
            hash: "816534932c2b7154836da6afc367695e6337db8a921823784c14378abed4f7d7".to_string(),
            previous_hash: "0".to_string(),
            timestamp: 1465154705,
            data: "my genesis block!!".to_string(),
        };
        Blockchain {
            chain: vec![genesis],
        }
    }

    pub fn get_genesis(&self) -> Block {
        self.chain.first().unwrap().clone()
    }

    pub fn get_latest(&self) -> Block {
        self.chain.last().unwrap().clone()
    }

    pub fn generate_block(&self, data: String) -> Block {
        let prev = self.get_latest();

        Block::new(
            prev.index + 1,
            prev.hash,
            Utc::now().timestamp() as u64,
            data,
        )
    }

    pub fn add_block(&mut self, block: Block) -> bool {
        let latest = self.get_latest();

        if latest.index + 1 != block.index {
            return false;
        }

        if latest.hash != block.previous_hash {
            return false;
        }

        if calculate_hash(
            block.index,
            &block.previous_hash,
            block.timestamp,
            &block.data,
        ) != block.hash
        {
            return false;
        }

        self.chain.push(block);
        true
    }

    pub fn is_valid(&self, chain: &[Block]) -> bool {
        if chain.is_empty() || chain[0] != self.get_genesis() {
            return false;
        }

        for i in 1..chain.len() {
            let prev = &chain[i - 1];
            let curr = &chain[i];

            if prev.index + 1 != curr.index {
                return false;
            }

            if prev.hash != curr.previous_hash {
                return false;
            }

            let hash =
                calculate_hash(curr.index, &curr.previous_hash, curr.timestamp, &curr.data);

            if hash != curr.hash {
                return false;
            }
        }

        true
    }
}

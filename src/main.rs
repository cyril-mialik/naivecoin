use sha2::{Digest, Sha256};

#[derive(Debug)]
struct Block {
    index: u64,
    hash: String,
    previous_hash: Option<String>,
    timestamp: i64,
    data: String,
}

impl Block {
    fn genesis() -> Self {
        let mut block = Self {
            index: 0,
            hash: String::new(),
            previous_hash: None,
            timestamp: 0,
            data: String::from("genesis"),
        };

        block.hash = block.calculate_hash();

        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();

        hasher.update(self.index.to_le_bytes());
        hasher.update(self.timestamp.to_le_bytes());
        hasher.update(self.data.as_bytes());

        if let Some(ref previous_hash) = self.previous_hash {
            hasher.update(previous_hash.as_bytes());
        }

        hex::encode(hasher.finalize())
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis = Block::genesis();

        Self {
            chain: vec![genesis],
        }
    }

    fn generate_next_block(&mut self, data: String) {
        let last_block = self.last_block();

        let mut block = Block {
            index: last_block.index + 1,
            hash: String::new(),
            previous_hash: Some(last_block.hash.clone()),
            timestamp: 0,
            data,
        };

        block.hash = block.calculate_hash();

        if !Self::is_valid_new_block(&block, last_block) {
            println!("Your Block is invalid");
            return;
        }

        self.chain.push(block);
    }

    fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    fn is_valid_new_block(new_block: &Block, previous_block: &Block) -> bool {
        if new_block.index != previous_block.index + 1 {
            return false;
        }

        if let Some(ref hash) = new_block.previous_hash && *hash != previous_block.hash {
            return false;
        }

        if new_block.calculate_hash() != new_block.hash {
            return false;
        }

        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.generate_next_block(String::from("My first block!"));
    blockchain.generate_next_block(String::from("My second block!"));

    println!("{:?}", blockchain);
}

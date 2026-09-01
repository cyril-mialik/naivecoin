use serde::{Deserialize, Serialize};
use crate::transaction::Transaction;
use crate::utils::{hash_data, DIFFICULTY};

/// Структура блока
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    /// Создает новый блок без майнинга
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: &str) -> Self {
        let timestamp = chrono::Utc::now().timestamp();
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash: previous_hash.to_string(),
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    /// Вычисляет хеш блока
    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{:?}{}{}",
            self.index, self.timestamp, self.transactions, self.previous_hash, self.nonce
        );

        hash_data(&data)
    }

    /// Майнит блок (Proof of Work)
    pub fn mine(&mut self) {
        let target = "0".repeat(DIFFICULTY);
        println!("⛏️  Mining block {} with difficulty {}", self.index, DIFFICULTY);
        
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        
        println!("✅ Block mined: {} with nonce {}", self.hash, self.nonce);
    }

    /// Проверяет, что блок правильно замайнен
    pub fn is_mined(&self) -> bool {
        let target = "0".repeat(DIFFICULTY);
        self.hash.starts_with(&target) && self.hash == self.calculate_hash()
    }

    /// Проверяет валидность блока
    pub fn validate(&self, previous_block: &Block) -> Result<(), String> {
        // Проверка индекса
        if self.index != previous_block.index + 1 {
            return Err(format!(
                "Invalid block index: expected {}, got {}",
                previous_block.index + 1,
                self.index
            ));
        }

        // Проверка previous_hash
        if self.previous_hash != previous_block.hash {
            return Err("Invalid previous hash".to_string());
        }

        // Проверка хеша
        if self.hash != self.calculate_hash() {
            return Err("Invalid block hash".to_string());
        }

        // Проверка proof of work
        if !self.is_mined() {
            return Err("Invalid proof of work".to_string());
        }

        // Проверка всех транзакций
        for tx in &self.transactions {
            if !tx.is_valid() {
                return Err(format!("Invalid transaction: {:?}", tx));
            }
        }

        Ok(())
    }

    /// Создает генезис-блок
    pub fn genesis() -> Self {
        let tx = Transaction::genesis();
        let mut block = Block::new(0, vec![tx], "0");
        block.mine();
        block
    }

    /// Получает количество транзакций в блоке
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }

    /// Проверяет, является ли блок генезис-блоком
    pub fn is_genesis(&self) -> bool {
        self.index == 0 && self.previous_hash == "0"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let tx = Transaction::new("alice", "bob", 100);
        let block = Block::new(1, vec![tx], "0000000000000000");
        assert_eq!(block.index, 1);
        assert_eq!(block.hash.len(), 64);
        assert!(!block.hash.is_empty());
        assert!(!block.is_mined());
    }

    #[test]
    fn test_block_hash_changes() {
        let tx = Transaction::new("alice", "bob", 100);
        let block1 = Block::new(1, vec![tx.clone()], "prev1");
        let block2 = Block::new(1, vec![tx], "prev2");
        assert_ne!(block1.hash, block2.hash, "Different previous hash should change block hash");
    }

    #[test]
    fn test_mining() {
        let tx = Transaction::new("alice", "bob", 100);
        let mut block = Block::new(1, vec![tx], "genesis");
        block.mine();
        
        assert!(block.is_mined(), "Block should be properly mined");
        assert!(block.nonce > 0, "Nonce should be incremented during mining");
    }

    #[test]
    fn test_genesis_block() {
        let genesis = Block::genesis();
        assert_eq!(genesis.index, 0);
        assert_eq!(genesis.previous_hash, "0");
        assert!(genesis.is_genesis());
        assert!(genesis.is_mined());
        assert_eq!(genesis.transaction_count(), 1);
    }

    #[test]
    fn test_block_validation() {
        let genesis = Block::genesis();
        let tx = Transaction::new("alice", "bob", 100);
        let mut block = Block::new(1, vec![tx], &genesis.hash);
        block.mine();
        
        assert!(block.validate(&genesis).is_ok());
    }

    #[test]
    fn test_invalid_block_validation() {
        let genesis = Block::genesis();
        let tx = Transaction::new("alice", "bob", 100);
        let mut block = Block::new(2, vec![tx], &genesis.hash);
        block.mine();
        
        // Неправильный индекс
        assert!(block.validate(&genesis).is_err());
    }
}

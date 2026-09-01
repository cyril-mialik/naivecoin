use crate::block::Block;
use crate::transaction::Transaction;
use crate::wallet::Wallet;
use std::collections::HashMap;

/// Структура блокчейна
#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub wallets: HashMap<String, Wallet>,
}

impl Blockchain {
    /// Создает новый блокчейн с генезис-блоком
    pub fn new() -> Self {
        let genesis_block = Block::genesis();
        let mut blockchain = Self {
            chain: vec![genesis_block],
            wallets: HashMap::new(),
        };
        blockchain.initialize_wallets();
        blockchain
    }

    /// Инициализирует кошельки из генезис-блока
    fn initialize_wallets(&mut self) {
        // Инициализируем кошелек для генезис-адреса
        let genesis_wallet = Wallet::new("genesis");
        self.wallets.insert("genesis".to_string(), genesis_wallet);
    }

    /// Получает последний блок
    pub fn get_last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    /// Получает блок по индексу
    pub fn get_block_by_index(&self, index: u64) -> Option<&Block> {
        self.chain.iter().find(|block| block.index == index)
    }

    /// Получает баланс адреса
    pub fn get_balance(&self, address: &str) -> u64 {
        let mut balance: u64 = 0;

        for block in &self.chain {
            for tx in &block.transactions {
                if tx.from == address {
                    balance = balance.saturating_sub(tx.amount);
                }
                if tx.to == address {
                    balance = balance.saturating_add(tx.amount);
                }
            }
        }

        balance
    }

    /// Обновляет кошельки на основе балансов
    fn update_wallets(&mut self) {
        let addresses: Vec<String> = self.wallets.keys().cloned().collect();

        for address in addresses {
            let balance = self.get_balance(&address);

            if let Some(wallet) = self.wallets.get_mut(&address) {
                wallet.update_balance(balance);
            }
        }
    }

    /// Создает или получает кошелек
    pub fn get_wallet(&mut self, address: &str) -> &mut Wallet {
        if !self.wallets.contains_key(address) {
            let wallet = Wallet::new(address);
            self.wallets.insert(address.to_string(), wallet);
        }
        self.wallets.get_mut(address).unwrap()
    }

    /// Проверяет транзакцию
    pub fn validate_transaction(&self, tx: &Transaction) -> Result<(), String> {
        if !tx.is_valid() {
            return Err("Invalid transaction".to_string());
        }

        // Проверка баланса отправителя (кроме генезис-транзакций)
        if tx.from != "genesis" {
            let balance = self.get_balance(&tx.from);
            if balance < tx.amount {
                return Err(format!(
                    "Insufficient balance: {} has {}, needs {}",
                    tx.from, balance, tx.amount
                ));
            }
        }

        Ok(())
    }

    /// Добавляет транзакцию в блокчейн (создает новый блок)
    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        // Валидация транзакции
        self.validate_transaction(&tx)?;

        // Создаем блок с этой транзакцией
        let last_block = self.get_last_block();
        let mut block = Block::new(last_block.index + 1, vec![tx], &last_block.hash);

        // Майним блок
        block.mine();

        // Валидируем блок
        block.validate(last_block)?;

        // Добавляем блок
        self.chain.push(block);

        // Обновляем кошельки
        self.update_wallets();

        Ok(())
    }

    /// Добавляет блок с несколькими транзакциями
    pub fn add_block_with_transactions(
        &mut self,
        transactions: Vec<Transaction>,
    ) -> Result<(), String> {
        // Валидация всех транзакций
        for tx in &transactions {
            self.validate_transaction(tx)?;
        }

        // Создаем блок
        let last_block = self.get_last_block();
        let mut block = Block::new(last_block.index + 1, transactions, &last_block.hash);

        // Майним блок
        block.mine();

        // Валидируем блок
        block.validate(last_block)?;

        // Добавляем блок
        self.chain.push(block);

        // Обновляем кошельки
        self.update_wallets();

        Ok(())
    }

    /// Проверяет валидность всей цепочки
    pub fn validate_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if let Err(e) = current.validate(previous) {
                eprintln!("Invalid block at index {}: {}", i, e);
                return false;
            }
        }
        true
    }

    /// Заменяет цепочку на более длинную (консенсус)
    pub fn replace_chain(&mut self, new_chain: &[Block]) -> Result<(), String> {
        // Проверка, что новая цепочка длиннее
        if new_chain.len() <= self.chain.len() {
            return Err("New chain is not longer".to_string());
        }

        // Проверка валидности новой цепочки
        let temp_blockchain = Blockchain {
            chain: new_chain.to_vec(),
            wallets: HashMap::new(),
        };

        if !temp_blockchain.validate_chain() {
            return Err("New chain is invalid".to_string());
        }

        // Заменяем цепочку
        self.chain = new_chain.to_vec();

        // Обновляем кошельки
        self.update_wallets();

        println!(
            "🔄 Chain replaced with longer chain ({} blocks)",
            self.chain.len()
        );
        Ok(())
    }

    /// Получает общее количество транзакций
    pub fn get_transaction_count(&self) -> usize {
        self.chain
            .iter()
            .map(|block| block.transaction_count())
            .sum()
    }

    /// Получает общее количество блоков
    pub fn get_block_count(&self) -> usize {
        self.chain.len()
    }

    /// Проверяет, пуст ли блокчейн
    pub fn is_empty(&self) -> bool {
        self.chain.is_empty()
    }

    /// Получает список всех адресов
    pub fn get_all_addresses(&self) -> Vec<String> {
        let mut addresses: Vec<String> = Vec::new();

        for block in &self.chain {
            for tx in &block.transactions {
                if !addresses.contains(&tx.from) && tx.from != "genesis" {
                    addresses.push(tx.from.clone());
                }
                if !addresses.contains(&tx.to) && tx.to != "genesis" {
                    addresses.push(tx.to.clone());
                }
            }
        }

        addresses.sort();
        addresses
    }

    /// Получает статистику по блокчейну
    pub fn get_stats(&self) -> BlockchainStats {
        BlockchainStats {
            total_blocks: self.chain.len(),
            total_transactions: self.get_transaction_count(),
            total_addresses: self.get_all_addresses().len(),
            is_valid: self.validate_chain(),
            last_block_index: self.get_last_block().index,
            chain_difficulty: crate::utils::DIFFICULTY,
        }
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

/// Статистика блокчейна
#[derive(Debug, Clone)]
pub struct BlockchainStats {
    pub total_blocks: usize,
    pub total_transactions: usize,
    pub total_addresses: usize,
    pub is_valid: bool,
    pub last_block_index: u64,
    pub chain_difficulty: usize,
}

impl std::fmt::Display for BlockchainStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "📊 Blockchain Statistics")?;
        writeln!(f, "   Total Blocks: {}", self.total_blocks)?;
        writeln!(f, "   Total Transactions: {}", self.total_transactions)?;
        writeln!(f, "   Total Addresses: {}", self.total_addresses)?;
        writeln!(f, "   Chain Valid: {}", self.is_valid)?;
        writeln!(f, "   Last Block Index: {}", self.last_block_index)?;
        writeln!(f, "   Mining Difficulty: {}", self.chain_difficulty)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_creation() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.chain.len(), 1);
        assert_eq!(blockchain.chain[0].index, 0);
        assert!(blockchain.validate_chain());
    }

    #[test]
    fn test_add_transaction() {
        let mut blockchain = Blockchain::new();

        let tx = Transaction::new("genesis", "alice", 1000);
        blockchain.add_transaction(tx).unwrap();

        assert_eq!(blockchain.chain.len(), 2);
        assert_eq!(blockchain.get_balance("alice"), 1000);
        assert!(blockchain.validate_chain());
    }

    #[test]
    fn test_multiple_transactions() {
        let mut blockchain = Blockchain::new();

        let tx1 = Transaction::new("genesis", "alice", 1000);
        blockchain.add_transaction(tx1).unwrap();

        let tx2 = Transaction::new("alice", "bob", 300);
        blockchain.add_transaction(tx2).unwrap();

        assert_eq!(blockchain.get_balance("alice"), 700);
        assert_eq!(blockchain.get_balance("bob"), 300);
        assert!(blockchain.validate_chain());
    }

    #[test]
    fn test_insufficient_balance() {
        let mut blockchain = Blockchain::new();

        let tx = Transaction::new("alice", "bob", 100);
        let result = blockchain.add_transaction(tx);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Insufficient balance"));
    }

    #[test]
    fn test_chain_replacement() {
        let mut blockchain1 = Blockchain::new();

        // Создаем более длинную цепочку
        let mut blockchain2 = Blockchain::new();
        let tx1 = Transaction::new("genesis", "alice", 500);
        blockchain2.add_transaction(tx1).unwrap();
        let tx2 = Transaction::new("alice", "bob", 200);
        blockchain2.add_transaction(tx2).unwrap();

        // Заменяем цепочку
        assert!(blockchain1.chain.len() < blockchain2.chain.len());
        let result = blockchain1.replace_chain(&blockchain2.chain);
        assert!(result.is_ok());
        assert_eq!(blockchain1.chain.len(), blockchain2.chain.len());
        assert_eq!(blockchain1.get_balance("alice"), 300);
        assert_eq!(blockchain1.get_balance("bob"), 200);
    }

    #[test]
    fn test_get_stats() {
        let mut blockchain = Blockchain::new();

        let tx1 = Transaction::new("genesis", "alice", 1000);
        blockchain.add_transaction(tx1).unwrap();

        let tx2 = Transaction::new("alice", "bob", 300);
        blockchain.add_transaction(tx2).unwrap();

        let stats = blockchain.get_stats();
        assert_eq!(stats.total_blocks, 3);
        assert_eq!(stats.total_transactions, 3);
        assert_eq!(stats.total_addresses, 2);
        assert!(stats.is_valid);
    }
}

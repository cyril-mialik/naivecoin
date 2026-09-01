use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::utils::hash_data;

/// Структура транзакции
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub timestamp: i64,
}

impl Transaction {
    /// Создает новую транзакцию
    pub fn new(from: &str, to: &str, amount: u64) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            amount,
            timestamp: Utc::now().timestamp(),
        }
    }

    /// Вычисляет хеш транзакции
    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}",
            self.from, self.to, self.amount, self.timestamp
        );
        hash_data(&data)
    }

    /// Проверяет валидность транзакции
    pub fn is_valid(&self) -> bool {
        self.amount > 0 && !self.from.is_empty() && !self.to.is_empty()
    }

    /// Создает транзакцию для генезис-блока
    pub fn genesis() -> Self {
        Self::new("genesis", "genesis", 0)
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::genesis()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new("alice", "bob", 100);
        assert_eq!(tx.from, "alice");
        assert_eq!(tx.to, "bob");
        assert_eq!(tx.amount, 100);
        assert!(tx.is_valid());
    }

    #[test]
    fn test_transaction_hash() {
        let tx = Transaction::new("alice", "bob", 100);
        let hash1 = tx.calculate_hash();
        let hash2 = tx.calculate_hash();
        assert_eq!(hash1, hash2, "Hash should be deterministic");
        assert_eq!(hash1.len(), 64, "SHA-256 hash should be 64 hex chars");
    }

    #[test]
    fn test_transaction_validation() {
        let tx1 = Transaction::new("alice", "bob", 100);
        assert!(tx1.is_valid());

        let tx2 = Transaction::new("", "bob", 100);
        assert!(!tx2.is_valid());

        let tx3 = Transaction::new("alice", "", 100);
        assert!(!tx3.is_valid());

        let tx4 = Transaction::new("alice", "bob", 0);
        assert!(!tx4.is_valid());
    }
}

use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};

/// Структура кошелька
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub address: String,
    pub balance: u64,
}

impl Wallet {
    /// Создает новый кошелек
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            balance: 0,
        }
    }

    /// Создает транзакцию из кошелька
    pub fn create_transaction(&self, to: &str, amount: u64) -> Result<Transaction, String> {
        if amount == 0 {
            return Err("Transaction amount must be positive".to_string());
        }

        if amount > self.balance {
            return Err(format!(
                "Insufficient balance: {} has {}, needs {}",
                self.address, self.balance, amount
            ));
        }

        Ok(Transaction::new(&self.address, to, amount))
    }

    /// Обновляет баланс кошелька
    pub fn update_balance(&mut self, new_balance: u64) {
        self.balance = new_balance;
    }

    /// Проверяет, достаточно ли средств
    pub fn has_funds(&self, amount: u64) -> bool {
        self.balance >= amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let wallet = Wallet::new("alice");
        assert_eq!(wallet.address, "alice");
        assert_eq!(wallet.balance, 0);
    }

    #[test]
    fn test_create_transaction() {
        let mut wallet = Wallet::new("alice");
        wallet.balance = 1000;

        let tx = wallet.create_transaction("bob", 300).unwrap();
        assert_eq!(tx.from, "alice");
        assert_eq!(tx.to, "bob");
        assert_eq!(tx.amount, 300);
    }

    #[test]
    fn test_insufficient_funds() {
        let wallet = Wallet::new("alice");
        let result = wallet.create_transaction("bob", 100);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Insufficient balance"));
    }

    #[test]
    fn test_zero_amount() {
        let wallet = Wallet::new("alice");
        let result = wallet.create_transaction("bob", 0);
        assert!(result.is_err());
    }
}

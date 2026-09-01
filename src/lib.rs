pub mod block;
pub mod blockchain;
pub mod transaction;
pub mod wallet;
pub mod utils;

pub use block::Block;
pub use blockchain::{Blockchain, BlockchainStats};
pub use transaction::Transaction;
pub use wallet::Wallet;
pub use utils::{hash_data, DIFFICULTY};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_version() {
        assert!(!VERSION.is_empty());
    }
}

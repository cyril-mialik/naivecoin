use naivecoin::{Blockchain, DIFFICULTY, Transaction};
use std::time::Instant;

fn main() {
    println!("{}", "=".repeat(60));
    println!(
        "{:^60}",
        format!("SIMPLE BITCOIN v{}", naivecoin::VERSION)
    );
    println!("{}", "=".repeat(60));
    println!("Mining Difficulty: {} leading zeros\n", DIFFICULTY);

    let mut blockchain = Blockchain::new();
    println!("✅ Genesis block created");
    println!("   Hash: {}", blockchain.get_last_block().hash);
    println!("   Index: {}\n", blockchain.get_last_block().index);

    println!("{}", "-".repeat(60));
    println!("📝 Adding transactions...");
    println!("{}", "-".repeat(60));

    let transactions = vec![
        ("genesis", "alice", 1000),
        ("alice", "bob", 300),
        ("alice", "charlie", 200),
        ("bob", "charlie", 100),
        ("genesis", "dave", 500),
        ("dave", "alice", 200),
    ];

    for (from, to, amount) in transactions {
        let start = Instant::now();
        let tx = Transaction::new(from, to, amount);

        match blockchain.add_transaction(tx) {
            Ok(_) => {
                let duration = start.elapsed();
                let last_block = blockchain.get_last_block();
                println!(
                    "✅ {} → {}: {} coins (Block #{})",
                    from, to, amount, last_block.index
                );
                println!("   Hash: {}...", &last_block.hash[..20]);
                println!("   Nonce: {}", last_block.nonce);
                println!("   Time: {:?}\n", duration);
            }
            Err(e) => {
                println!("❌ Failed: {} → {}: {}", from, to, e);
            }
        }
    }

    let stats = blockchain.get_stats();
    println!("\n{}", stats);

    println!("{}", "=".repeat(60));
    println!("📈 Additional Information:");
    println!("   All addresses: {:?}", blockchain.get_all_addresses());
    println!("   Genesis block hash: {}", blockchain.chain[0].hash);
    println!("   Last block hash: {}", blockchain.get_last_block().hash);
    println!("   Chain validated: {}", blockchain.validate_chain());
    println!("{}", "=".repeat(60));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_demo() {
        main();
    }
}

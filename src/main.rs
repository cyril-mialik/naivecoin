mod blockchain;
mod http_server;
mod network;

use blockchain::Blockchain;
use http_server::start_server;
use network::P2PNetwork;
use std::sync::Arc;
use tokio::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Starting Minimal Blockchain Node");
    println!("===================================");

    // Создаем блокчейн
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    // Запускаем HTTP сервер
    let http_port = 3001;
    let blockchain_http = blockchain.clone();
    tokio::spawn(async move {
        start_server(http_port, blockchain_http).await;
    });

    // Запускаем P2P WebSocket сервер
    let p2p_port = 6001;
    let blockchain_p2p = blockchain.clone();
    let _network = P2PNetwork::new(p2p_port, blockchain_p2p).await;

    println!("✅ Node started successfully!");
    println!("📡 HTTP API: http://localhost:{}", http_port);
    println!("🌐 WebSocket: ws://localhost:{}", p2p_port);
    println!("===================================");
    println!("Press Ctrl+C to stop...");

    // Ждем бесконечно
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}

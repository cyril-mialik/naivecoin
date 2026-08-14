use crate::blockchain::Blockchain;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

pub struct P2PNetwork {
    _blockchain: Arc<Mutex<Blockchain>>, // Используем _ чтобы избежать warning
}

impl P2PNetwork {
    pub async fn new(port: u16, blockchain: Arc<Mutex<Blockchain>>) -> Self {
        let blockchain_clone = blockchain.clone();
        tokio::spawn(async move {
            start_websocket_server(port, blockchain_clone).await;
        });

        P2PNetwork {
            _blockchain: blockchain,
        }
    }
}

async fn start_websocket_server(port: u16, blockchain: Arc<Mutex<Blockchain>>) {
    println!("🌐 WebSocket server on ws://localhost:{}", port);

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", port)).await;

    match listener {
        Ok(listener) => {
            println!("✅ WebSocket server started");

            while let Ok((stream, _)) = listener.accept().await {
                let blockchain = blockchain.clone();
                tokio::spawn(async move {
                    handle_connection(stream, blockchain).await;
                });
            }
        }
        Err(e) => {
            println!("❌ Failed to start WebSocket: {}", e);
        }
    }
}

async fn handle_connection(stream: tokio::net::TcpStream, blockchain: Arc<Mutex<Blockchain>>) {
    match accept_async(stream).await {
        Ok(ws_stream) => {
            println!("✅ New WebSocket client connected");
            let (mut sender, mut receiver) = ws_stream.split();

            // Отправляем приветствие
            let welcome = serde_json::json!({
                "type": "welcome",
                "message": "Connected to Blockchain node",
                "blocks": blockchain.lock().await.chain.len()
            });
            let _ = sender.send(Message::Text(welcome.to_string())).await;

            // Обрабатываем сообщения
            while let Some(result) = receiver.next().await {
                match result {
                    Ok(msg) => {
                        if let Ok(text) = msg.to_text() {
                            println!("📨 Received: {}", text);

                            let response = serde_json::json!({
                                "type": "response",
                                "message": format!("Received: {}", text)
                            });
                            let _ = sender.send(Message::Text(response.to_string())).await;
                        }
                    }
                    Err(e) => {
                        println!("❌ WebSocket error: {}", e);
                        break;
                    }
                }
            }

            println!("🔌 WebSocket client disconnected");
        }
        Err(e) => {
            println!("❌ WebSocket handshake failed: {}", e);
        }
    }
}

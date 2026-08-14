use crate::blockchain::Blockchain;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
pub struct MineRequest {
    pub data: String,
}

pub async fn start_server(port: u16, blockchain: Arc<Mutex<Blockchain>>) {
    println!("HTTP server on port {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(blockchain.clone()))
            .route("/blocks", web::get().to(get_blocks))
            .route("/mine", web::post().to(mine_block))
            .route("/validate", web::get().to(validate_chain))
            .route("/reset", web::post().to(reset_chain))
            .route("/latest", web::get().to(get_latest))
            .route("/genesis", web::get().to(get_genesis))
    })
    .bind(("127.0.0.1", port))
    .unwrap()
    .run()
    .await
    .unwrap();
}

async fn get_latest(blockchain: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let chain = blockchain.lock().await;

    HttpResponse::Ok().json(chain.get_latest())
}

async fn get_genesis(blockchain: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let chain = blockchain.lock().await;

    HttpResponse::Ok().json(chain.get_genesis())
}

async fn get_blocks(blockchain: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let chain = blockchain.lock().await;

    HttpResponse::Ok().json(&chain.chain)
}

async fn mine_block(
    blockchain: web::Data<Arc<Mutex<Blockchain>>>,
    req: web::Json<MineRequest>,
) -> impl Responder {
    let mut chain = blockchain.lock().await;
    let new_block = chain.generate_block(req.data.clone());

    if chain.add_block(new_block.clone()) {
        HttpResponse::Ok().json(json!({
            "status": "ok",
            "block": new_block
        }))
    } else {
        HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "Failed to add block"
        }))
    }
}

async fn validate_chain(blockchain: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let chain = blockchain.lock().await;
    let valid = chain.is_valid(&chain.chain);

    HttpResponse::Ok().json(json!({
        "valid": valid,
        "blocks": chain.chain.len()
    }))
}

async fn reset_chain(blockchain: web::Data<Arc<Mutex<Blockchain>>>) -> impl Responder {
    let mut chain = blockchain.lock().await;
    *chain = Blockchain::new();

    HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": "Blockchain reset"
    }))
}

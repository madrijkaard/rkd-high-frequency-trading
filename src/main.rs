mod dto;
mod api;
mod trade;
mod config;
mod blockchain;
mod state;

use actix_web::{App, HttpServer};
use api::{get_trades_start, get_blockchain};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Servidor rodando em http://localhost:8080");
    HttpServer::new(|| {
        App::new()
            .service(get_trades_start)
            .service(get_blockchain)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

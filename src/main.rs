mod dto;
mod api;
mod trade;
mod config;
mod blockchain;
mod order;
mod balance;
mod client;
mod credential;
mod schedule;
mod leverage;
mod decide;

use actix_web::{App, HttpServer};
use api::{
    get_trades_start,
    get_trades_chain,
    get_last_trade,
    post_trades_order,
    get_trades_balance,
    post_close_all_positions,
    stop_schedule,
    health_check,
    put_leverage,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Servidor rodando em http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(get_trades_start)
            .service(get_trades_chain)
            .service(get_last_trade)
            .service(post_trades_order)
            .service(get_trades_balance)
            .service(post_close_all_positions)
            .service(stop_schedule)
            .service(health_check)
            .service(put_leverage)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

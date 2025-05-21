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
    post_trades_start,
    post_trades_stop,
    get_trades_health_check,
    get_trades_chain,
    get_last_trade,
    post_trades_order,
    get_trades_balance,
    post_close_all_positions,
    put_leverage,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Server running at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(post_trades_start)
            .service(post_trades_stop)
            .service(get_trades_health_check)
            .service(get_trades_chain)
            .service(get_last_trade)
            .service(post_trades_order)
            .service(get_trades_balance)
            .service(post_close_all_positions)
            .service(put_leverage)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

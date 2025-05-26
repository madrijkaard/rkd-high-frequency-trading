mod dto;
mod api;
mod trade;
mod config;
mod blockchain;
mod order;
mod balance;
mod candlestick;
mod credential;
mod schedule;
mod leverage;
mod decide;
mod log;
mod status_trade;

use actix_web::{App, HttpServer};
use api::{
    post_trades_start,
    post_trades_stop,
    get_trades_health_check,
    get_trades_chain_by_symbol,
    get_last_trade_by_symbol,
    get_all_symbols_chains,
    get_chain_validity,
    post_trades_order,
    get_trades_balance,
    post_close_all_positions,
    put_leverage,
    get_trades_spy,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(post_trades_start)
            .service(post_trades_stop)
            .service(get_trades_health_check)
            .service(get_trades_chain_by_symbol)
            .service(get_last_trade_by_symbol)
            .service(get_all_symbols_chains)
            .service(get_chain_validity)
            .service(post_trades_order)
            .service(get_trades_balance)
            .service(post_close_all_positions)
            .service(put_leverage)
            .service(get_trades_spy)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

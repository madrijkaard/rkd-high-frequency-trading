use actix_web::{get, post, HttpResponse, Responder};
use crate::balance::get_futures_balance;
use crate::blockchain::BLOCKCHAIN;
use crate::client::get_candlesticks;
use crate::config::Settings;
use crate::order::execute_future_order;
use crate::trade::generate_trade;

#[get("/trades/start")]
pub async fn get_trades_start() -> impl Responder {

    let settings = Settings::load();
    let binance_settings = &settings.binance;

    match get_candlesticks(binance_settings).await {
        Ok(candlesticks) => {
            let trade = generate_trade(candlesticks);
            {
                let mut chain = BLOCKCHAIN.lock().unwrap();
                chain.add_block(trade.clone());
            }

            HttpResponse::Ok().json(trade)
        }
        Err(err) => {
            eprintln!("Erro ao obter candlesticks: {}", err);
            HttpResponse::InternalServerError().body(err)
        }
    }
}

#[get("/trades/chain")]
pub async fn get_trades_chain() -> impl Responder {
    let chain = BLOCKCHAIN.lock().unwrap();

    if chain.is_valid() {
        HttpResponse::Ok().json(chain.all())
    } else {
        HttpResponse::InternalServerError().body("Blockchain inválida: integridade comprometida.")
    }
}

#[get("/trades/chain/last")]
pub async fn get_last_trade() -> impl Responder {
    let chain = BLOCKCHAIN.lock().unwrap();

    match chain.get_last_trade() {
        Some(trade) => HttpResponse::Ok().json(trade),
        None => HttpResponse::NotFound().body("Nenhum trade encontrado na blockchain"),
    }
}

#[post("/trades/order")]
pub async fn post_trades_order() -> impl Responder {
    match execute_future_order().await {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(e) => {
            eprintln!("Erro ao enviar ordem para Binance: {}", e);
            HttpResponse::InternalServerError().body(e)
        }
    }
}

#[get("/trades/balance")]
pub async fn get_trades_balance() -> impl Responder {
    match get_futures_balance().await {
        Ok(balances) => {
            let usdt_balance: Vec<_> = balances
                .into_iter()
                .filter(|b| b.asset == "USDT")
                .collect();
            HttpResponse::Ok().json(usdt_balance)
        }
        Err(e) => {
            eprintln!("Erro ao consultar saldo de futuros: {}", e);
            HttpResponse::InternalServerError().body(format!("Erro: {}", e))
        }
    }
}

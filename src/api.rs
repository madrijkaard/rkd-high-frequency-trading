use actix_web::{get, post, put, web, HttpResponse, Responder};
use crate::balance::get_futures_balance;
use crate::blockchain::BLOCKCHAIN;
use crate::candlestick::get_candlesticks;
use crate::config::Settings;
use crate::dto::OpenOrderRequest;
use crate::leverage::set_leverage;
use crate::order::{close_all_positions, execute_future_order};
use crate::schedule::get_scheduler;
use crate::trade::generate_trade;

#[post("/trades/start")]
pub async fn post_trades_start() -> impl Responder {
    let scheduler = get_scheduler();
    let mut scheduler = scheduler.lock().unwrap();
    scheduler.start();
    HttpResponse::Ok().body("Timer started")
}

#[post("/trades/stop")]
pub async fn post_trades_stop() -> impl Responder {
    let scheduler = get_scheduler();
    let mut scheduler = scheduler.lock().unwrap();
    scheduler.stop();
    HttpResponse::Ok().body("Timer stopped")
}

#[get("/trades/health-check")]
pub async fn get_trades_health_check() -> impl Responder {
    let scheduler = get_scheduler();
    let scheduler = scheduler.lock().unwrap();
    let status = if scheduler.is_active() { "UP" } else { "DOWN" };
    HttpResponse::Ok().body(format!("status: {}", status))
}

#[get("/trades/chain")]
pub async fn get_trades_chain() -> impl Responder {
    let chain = BLOCKCHAIN.lock().unwrap();

    if chain.is_valid() {
        HttpResponse::Ok().json(chain.all())
    } else {
        HttpResponse::InternalServerError().body("Invalid blockchain: integrity compromised")
    }
}

#[get("/trades/chain/last")]
pub async fn get_last_trade() -> impl Responder {
    let chain = BLOCKCHAIN.lock().unwrap();

    match chain.get_last_trade() {
        Some(trade) => HttpResponse::Ok().json(trade),
        None => HttpResponse::NotFound().body("No trades found on blockchain"),
    }
}

#[get("/trades/balance")]
pub async fn get_trades_balance() -> impl Responder {
    let settings = Settings::load();

    match get_futures_balance(&settings.binance).await {
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

#[post("/trades/order/open")]
pub async fn post_trades_order(req: web::Json<OpenOrderRequest>) -> impl Responder {
    let settings = Settings::load();
    let binance_settings = &settings.binance;

    let side = req.side.to_uppercase();
    if side != "BUY" && side != "SELL" {
        return HttpResponse::BadRequest().body("O parâmetro 'side' deve ser 'BUY' ou 'SELL'");
    }

    match execute_future_order(binance_settings, &side).await {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(e) => {
            eprintln!("Erro ao enviar ordem para Binance: {}", e);
            HttpResponse::InternalServerError().body(e)
        }
    }
}

#[post("/trades/order/close")]
pub async fn post_close_all_positions() -> impl Responder {
    let settings = Settings::load();
    let binance_settings = &settings.binance;

    match close_all_positions(binance_settings).await {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => {
            eprintln!("Erro ao fechar posições: {}", e);
            HttpResponse::InternalServerError().body(e)
        }
    }
}

#[put("/trades/leverage")]
pub async fn put_leverage() -> impl Responder {
    let settings = Settings::load();

    match set_leverage(&settings.binance).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => {
            eprintln!("Erro ao aplicar alavancagem: {}", e);
            HttpResponse::InternalServerError().body(format!("Erro: {}", e))
        }
    }
}

#[get("/trades/spy")]
pub async fn get_trades_spy() -> impl Responder {
    
    let settings = Settings::load();

    if !settings.spy {
        return HttpResponse::Forbidden().body("Serviço /trades/spy está desativado na configuração");
    }

    let binance_settings = &settings.binance;
    let cryptos = settings.cryptos.clone();

    let futures = cryptos.iter().map(|symbol| {
        let base_url = binance_settings.base_url.clone();
        let interval = binance_settings.interval.clone();
        let limit = binance_settings.limit;
        let symbol_clone = symbol.to_string();

        tokio::spawn(async move {
            let candles = get_candlesticks(&base_url, &symbol_clone, &interval, limit).await;
            match candles {
                Ok(candles_data) => {
                    let ref_data = get_candlesticks(&base_url, "BTCUSDT", &interval, limit).await;
                    match ref_data {
                        Ok(reference_data) => {
                            let trade = generate_trade(candles_data, reference_data);
                            let mut map = serde_json::to_value(&trade).unwrap();
                            if let serde_json::Value::Object(ref mut obj) = map {
                                obj.insert("crypto".to_string(), serde_json::Value::String(symbol_clone));
                            }
                            Ok(map)
                        }
                        Err(e) => Err(format!("Erro em BTCUSDT para {}: {}", symbol_clone, e))
                    }
                }
                Err(e) => Err(format!("Erro em {}: {}", symbol_clone, e))
            }
        })
    });

    let results = futures::future::join_all(futures).await;

    let trades: Vec<_> = results.into_iter().filter_map(|r| r.ok()).collect();

    HttpResponse::Ok().json(trades)
}

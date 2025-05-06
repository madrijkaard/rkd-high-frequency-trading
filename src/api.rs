use actix_web::{get, HttpResponse, Responder};
use reqwest::Client;
use serde_json::Value;

use crate::config::Settings;
use crate::dto::Candlestick;
use crate::trade::generate_trade;

#[get("/trades/load")]
pub async fn get_max_and_min_prices() -> impl Responder {
    
    let settings = Settings::load();
    let binance = settings.binance;

    let url = "https://api.binance.com/api/v3/uiKlines";
    
    let params = [
        ("symbol", binance.symbol.as_str()),
        ("interval", binance.interval.as_str()),
        ("limit", &binance.limit.to_string()),
    ];

    let client = Client::new();
    let response = client.get(url).query(&params).send().await;

    match response {
        Ok(resp) => match resp.json::<Vec<Vec<Value>>>().await {
            Ok(raw_data) => {
                let candlesticks: Vec<Candlestick> = raw_data
                    .into_iter()
                    .filter_map(|c| {
                        if c.len() == 12 {
                            Some(Candlestick {
                                open_time: c[0].as_u64()?,
                                open_price: c[1].as_str()?.to_string(),
                                high_price: c[2].as_str()?.to_string(),
                                low_price: c[3].as_str()?.to_string(),
                                close_price: c[4].as_str()?.to_string(),
                                volume: c[5].as_str()?.to_string(),
                                close_time: c[6].as_u64()?,
                                quote_asset_volume: c[7].as_str()?.to_string(),
                                number_of_trades: c[8].as_u64()?,
                                taker_buy_base_asset_volume: c[9].as_str()?.to_string(),
                                taker_buy_quote_asset_volume: c[10].as_str()?.to_string(),
                                ignore: c[11].as_str()?.to_string(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect();

                let trade = generate_trade(candlesticks);
                HttpResponse::Ok().json(trade)
            }
            Err(e) => {
                eprintln!("Erro ao desserializar JSON da Binance: {:?}", e);
                HttpResponse::InternalServerError().body("Erro ao processar resposta da Binance")
            }
        },
        Err(e) => {
            eprintln!("Erro na requisição HTTP: {:?}", e);
            HttpResponse::InternalServerError().body("Erro ao acessar API da Binance")
        }
    }
}

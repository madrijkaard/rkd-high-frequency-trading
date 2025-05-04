use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Candlestick {
    pub open_time: u64,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub close_price: String,
    pub volume: String,
    pub close_time: u64,
    pub quote_asset_volume: String,
    pub number_of_trades: u64,
    pub taker_buy_base_asset_volume: String,
    pub taker_buy_quote_asset_volume: String,
    pub ignore: String,
}

#[derive(Debug, Serialize)]
pub struct CandlestickStats {
    total: usize,
    max_high_price: String,
    min_low_price: String,
}

#[get("/candlesticks/max-and-min")]
pub async fn get_max_and_min_prices() -> impl Responder {
    let url = "https://api.binance.com/api/v3/uiKlines";
    let params = [
        ("symbol", "BTCUSDT"),
        ("interval", "1h"),
        ("limit", "200"),
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

                let total = candlesticks.len();

                let max_high_price = candlesticks
                    .iter()
                    .filter_map(|c| c.high_price.parse::<f64>().ok())
                    .fold(f64::MIN, f64::max);

                let min_low_price = candlesticks
                    .iter()
                    .filter_map(|c| c.low_price.parse::<f64>().ok())
                    .fold(f64::MAX, f64::min);

                let stats = CandlestickStats {
                    total,
                    max_high_price: format!("{:.2}", max_high_price),
                    min_low_price: format!("{:.2}", min_low_price),
                };

                HttpResponse::Ok().json(stats)
            }
            Err(e) => {
                eprintln!("Erro ao deserializar JSON da Binance: {:?}", e);
                HttpResponse::InternalServerError().body("Erro ao processar resposta da Binance")
            }
        },
        Err(e) => {
            eprintln!("Erro na requisição HTTP: {:?}", e);
            HttpResponse::InternalServerError().body("Erro ao acessar a API da Binance")
        }
    }
}

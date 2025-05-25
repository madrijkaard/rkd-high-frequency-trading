use crate::config::BinanceSettings;
use crate::dto::{
    Candlestick, ExchangeInfoResponse, LotSizeFilter, LotSizeInfo,
};
use reqwest::Client;
use serde_json::Value;

pub async fn get_candlesticks(
    base_url: &str,
    symbol: &str,
    interval: &str,
    limit: u32,
) -> Result<Vec<Candlestick>, String> {
    let url = format!("{}/uiKlines", base_url);

    let params = [
        ("symbol", symbol),
        ("interval", interval),
        ("limit", &limit.to_string()),
    ];

    let client = Client::new();

    let response = client
        .get(&url)
        .query(&params)
        .send()
        .await
        .map_err(|e| format!("Erro na requisicao HTTP: {:?}", e))?;

    let raw_data = response
        .json::<Vec<Vec<Value>>>()
        .await
        .map_err(|e| format!("Erro ao desserializar JSON da Binance: {:?}", e))?;

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

    Ok(candlesticks)
}

pub async fn get_current_btc_price(settings: &BinanceSettings) -> Result<f64, String> {
    let url = format!(
        "{}/ticker/price?symbol={}",
        settings.future_url, settings.symbol
    );

    let client = Client::new();

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Erro ao obter preco atual: {:?}", e))?;

    if res.status().is_success() {
        let data: Value = res
            .json()
            .await
            .map_err(|e| format!("Erro ao interpretar resposta do preco: {:?}", e))?;

        data["price"]
            .as_str()
            .ok_or("Campo 'price' ausente".to_string())?
            .parse::<f64>()
            .map_err(|_| "Erro ao converter preco para f64".to_string())
    } else {
        let err = res
            .text()
            .await
            .unwrap_or_else(|_| "Erro desconhecido".to_string());
        Err(format!("Erro ao buscar preco: {}", err))
    }
}

pub async fn get_lot_size_info(settings: &BinanceSettings) -> Result<LotSizeInfo, String> {
    let url = format!("{}/exchangeInfo?symbol={}", settings.future_url, settings.symbol);
    let client = Client::new();

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Erro ao obter exchangeInfo: {:?}", e))?;

    if !res.status().is_success() {
        let err = res
            .text()
            .await
            .unwrap_or_else(|_| "Erro desconhecido".to_string());
        return Err(format!("Erro da Binance: {}", err));
    }

    let data: ExchangeInfoResponse = res
        .json()
        .await
        .map_err(|e| format!("Erro ao interpretar exchangeInfo: {:?}", e))?;

    for filter in &data.symbols.first().ok_or("Simbolo nao encontrado")?.filters {
        if let LotSizeFilter::LotSize { step_size } = filter {
            return step_size
                .parse::<f64>()
                .map(|step| LotSizeInfo { step_size: step })
                .map_err(|_| "Erro ao converter stepSize para f64".to_string());
        }
    }

    Err("Filtro LOT_SIZE nao encontrado".to_string())
}

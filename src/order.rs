use crate::balance::get_futures_balance;
use crate::client::{get_current_btc_price, get_lot_size_info};
use crate::credential::get_credentials;
use crate::dto::{BalanceResponse, OrderResponse};
use crate::config::BinanceSettings;
use hmac::{Hmac, Mac};
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client,
};
use sha2::Sha256;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use url::form_urlencoded;

type HmacSha256 = Hmac<Sha256>;

fn round_quantity(value: f64, step: f64) -> f64 {
    (value / step).floor() * step
}

fn get_timestamp() -> u64 {
    let start = SystemTime::now();
    let since = start.duration_since(UNIX_EPOCH).unwrap();
    since.as_millis() as u64
}

fn sign_query(query: &str, secret: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(query.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

async fn get_server_time_offset() -> Result<i64, String> {
    let client = Client::new();
    let res = client
        .get("https://fapi.binance.com/fapi/v1/time")
        .send()
        .await
        .map_err(|e| format!("Erro ao consultar /time: {:?}", e))?;

    let json: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("Erro ao parsear /time: {:?}", e))?;

    let server_time = json["serverTime"].as_i64().ok_or("Campo serverTime ausente")?;
    let local_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "Erro no relógio local")?
        .as_millis() as i64;

    Ok(server_time - local_time)
}

pub async fn execute_future_order(settings: &BinanceSettings) -> Result<OrderResponse, String> {
    let credentials = get_credentials();
    let api_key = &credentials.key;
    let secret_key = &credentials.secret;

    let base_url = format!("{}/order", settings.future_url);

    let offset = get_server_time_offset().await.unwrap_or(0);
    let timestamp = (get_timestamp() as i64 + offset) as u64;
    let timestamp_str = timestamp.to_string();

    let preco_btc = get_current_btc_price().await?;
    let lot_size_info = get_lot_size_info().await?;

    let balances = get_futures_balance()
        .await
        .map_err(|e| format!("Erro ao consultar saldo: {:?}", e))?;

    let usdt_balance: BalanceResponse = balances
        .into_iter()
        .find(|b| b.asset == "USDT")
        .ok_or("Saldo de USDT não encontrado")?;

    let available_usdt: f64 = usdt_balance
        .available
        .parse()
        .map_err(|_| "Erro ao converter saldo USDT para f64")?;

    let quantity_raw = available_usdt / preco_btc;
    let quantity = round_quantity(quantity_raw, lot_size_info.step_size);

    let precision = (1.0 / lot_size_info.step_size).log10().round() as usize;
    let quantity_str = format!("{:.*}", precision, quantity)
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string();

    println!(
        "→ Enviando ordem com quantity: '{}' (USDT: {}, Preco BTC: {}, StepSize: {})",
        quantity_str, available_usdt, preco_btc, lot_size_info.step_size
    );

    let notional = quantity * preco_btc;
    if notional < 20.0 {
        return Err(format!(
            "Valor total da ordem ({:.2} USDT) e menor que o minimo exigido (20 USDT).",
            notional
        ));
    }

    let mut params = HashMap::new();
    params.insert("symbol", settings.symbol.as_str());
    params.insert("side", "SELL");
    params.insert("type", "MARKET");
    params.insert("quantity", &quantity_str);
    params.insert("recvWindow", "10000");
    params.insert("timestamp", &timestamp_str);

    let query_string = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(&params)
        .finish();

    let signature = sign_query(&query_string, secret_key);
    let signed_query = format!("{}&signature={}", query_string, signature);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("X-MBX-APIKEY", HeaderValue::from_str(api_key).unwrap());

    let client = Client::new();

    let res = client
        .post(format!("{}?{}", base_url, signed_query))
        .headers(headers)
        .send()
        .await
        .map_err(|e| format!("Erro de requisicao: {:?}", e))?;

    if res.status().is_success() {
        res.json::<OrderResponse>()
            .await
            .map_err(|e| format!("Erro ao interpretar JSON: {:?}", e))
    } else {
        let err = res
            .text()
            .await
            .unwrap_or_else(|_| "Erro desconhecido".to_string());
        Err(format!("Erro da Binance: {}", err))
    }
}

use hmac::{Hmac, Mac};
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client,
};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::credential::get_credentials;
use crate::dto::BalanceResponse;

type HmacSha256 = Hmac<Sha256>;

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

pub async fn get_futures_balance() -> Result<Vec<BalanceResponse>, Box<dyn std::error::Error>> {
    let credentials = get_credentials();
    let api_key = &credentials.key;
    let secret_key = &credentials.secret;

    let timestamp = get_timestamp();
    let query = format!("recvWindow=10000&timestamp={}", timestamp);
    let signature = sign_query(&query, secret_key);
    let full_query = format!("{}&signature={}", query, signature);

    let url = format!("https://fapi.binance.com/fapi/v2/balance?{}", full_query);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("X-MBX-APIKEY", HeaderValue::from_str(api_key)?);

    let client = Client::new();
    let res = client.get(&url).headers(headers).send().await?;

    if res.status().is_success() {
        let balances: Vec<BalanceResponse> = res.json().await?;

        for balance in &balances {
            if balance.asset == "USDT" {
                println!("Saldo total em USDT: {}", balance.total);
                println!("Saldo disponivel em USDT: {}", balance.available);
            }
        }

        Ok(balances)
    } else {
        let error_text = res.text().await?;
        println!("Erro ao consultar saldo: {}", error_text);
        Err("Erro na resposta da Binance".into())
    }
}

use serde::{Deserialize, Serialize};

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
pub struct MaxMinPrice {
    pub max_high_price: String,
    pub min_low_price: String,
    pub current_price: String,
    pub intermediate_price_1: String,
    pub intermediate_price_2: String,
    pub of: usize,
}
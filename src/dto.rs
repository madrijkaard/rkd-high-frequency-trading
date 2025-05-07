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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trade {
    pub zone_max: String,
    pub zone_7: String,
    pub zone_6: String,
    pub zone_5: String,
    pub zone_4: String,
    pub zone_3: String,
    pub zone_2: String,
    pub zone_1: String,
    pub zone_min: String,
    pub current_price: String,
    pub of: usize,
}

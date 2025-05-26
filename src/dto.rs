use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Bias {
    Bullish,
    Bearish,
    None,
}

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
    pub symbol: String,
    pub current_price: String,
    pub cma: String,
    pub oma: String,
    pub bias: Bias,
    pub status: Option<TradeStatus>,
    pub zone_max: String,
    pub zone_7: String,
    pub zone_6: String,
    pub zone_5: String,
    pub zone_4: String,
    pub zone_3: String,
    pub zone_2: String,
    pub zone_1: String,
    pub zone_min: String,
    pub of: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TradeStatus {
    InZone7,
    OutZone5,
    PrepareZone1,
    InZone3,
    PrepareZone1Long,
    LongZone3,
    TargetZone7,
    InZone1,
    OutZone3,
    PrepareZone7,
    InZone5,
    PrepareZone7Short,
    ShortZone5,
    TargetZone1,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeInfoResponse {
    pub symbols: Vec<SymbolInfo>,
}

#[derive(Debug, Deserialize)]
pub struct SymbolInfo {
    pub filters: Vec<LotSizeFilter>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "filterType")]
pub enum LotSizeFilter {
    #[serde(rename = "LOT_SIZE")]
    LotSize {
        #[serde(rename = "stepSize")]
        step_size: String,
    },
    #[serde(other)]
    Other,
}

pub struct LotSizeInfo {
    pub step_size: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceResponse {
    pub asset: String,

    #[serde(rename = "balance")]
    pub total: String,

    #[serde(rename = "availableBalance")]
    pub available: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OrderResponse {
    pub symbol: String,

    #[serde(rename = "orderId")]
    pub order_id: u64,

    pub status: String,
    pub side: String,
    pub price: String,

    #[serde(rename = "origQty")]
    pub orig_qty: String,

    #[serde(rename = "executedQty")]
    pub executed_qty: String,

    #[serde(rename = "cummulativeQuoteQty")]
    pub cummulative_quote_qty: Option<String>,

    #[serde(rename = "timeInForce")]
    pub time_in_force: String,

    #[serde(rename = "type")]
    pub order_type: String,

    #[serde(rename = "updateTime")]
    pub update_time: u64,
}

#[derive(Debug, Deserialize)]
pub struct OpenOrderRequest {
    pub side: String,
}

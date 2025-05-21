use crate::blockchain::BLOCKCHAIN;
use crate::dto::{Bias, Candlestick, Trade, TradeStatus};

pub fn generate_trade(candlesticks: Vec<Candlestick>) -> Trade {
    
    let of = candlesticks.len();

    if of < 271 {
        return Trade {
            current_price: "0.0".into(),
            cma: "0.0".into(),
            oma: "0.0".into(),
            bias: Bias::None,
            status: None,
            zone_max: "0.0".into(),
            zone_7: "0.0".into(),
            zone_6: "0.0".into(),
            zone_5: "0.0".into(),
            zone_4: "0.0".into(),
            zone_3: "0.0".into(),
            zone_2: "0.0".into(),
            zone_1: "0.0".into(),
            zone_min: "0.0".into(),
            of,
        };
    }

    let cma_valor = calculate_moving_average(&candlesticks[71..]);
    let oma_valor = calculate_moving_average(&candlesticks[..200]);

    let bias = if cma_valor > oma_valor {
        Bias::Bullish
    } else if cma_valor < oma_valor {
        Bias::Bearish
    } else {
        Bias::None
    };

    let analysis_slice = &candlesticks[71..];

    let max_high = analysis_slice
        .iter()
        .filter_map(|c| c.high_price.parse::<f64>().ok())
        .fold(f64::MIN, f64::max);

    let min_low = analysis_slice
        .iter()
        .filter_map(|c| c.low_price.parse::<f64>().ok())
        .fold(f64::MAX, f64::min);

    let current_price = analysis_slice
        .iter()
        .max_by_key(|c| c.close_time)
        .map(|c| c.close_price.clone())
        .unwrap_or_else(|| "0.0".to_string());

    let log_min = min_low.ln();
    let log_max = max_high.ln();
    let log_zone_4 = (log_min + log_max) / 2.0;
    let log_zone_2 = (log_min + log_zone_4) / 2.0;
    let log_zone_6 = (log_max + log_zone_4) / 2.0;
    let log_zone_3 = (log_zone_2 + log_zone_4) / 2.0;
    let log_zone_5 = (log_zone_6 + log_zone_4) / 2.0;
    let log_zone_1 = (log_min + log_zone_2) / 2.0;
    let log_zone_7 = (log_max + log_zone_6) / 2.0;

    let price_zone_7 = log_zone_7.exp();
    let price_zone_6 = log_zone_6.exp();
    let price_zone_5 = log_zone_5.exp();
    let price_zone_4 = log_zone_4.exp();
    let price_zone_3 = log_zone_3.exp();
    let price_zone_2 = log_zone_2.exp();
    let price_zone_1 = log_zone_1.exp();

    let trade = Trade {
        current_price,
        cma: format!("{:.8}", cma_valor),
        oma: format!("{:.8}", oma_valor),
        bias,
        status: None,
        zone_max: format!("{:.8}", max_high),
        zone_7: format!("{:.8}", price_zone_7),
        zone_6: format!("{:.8}", price_zone_6),
        zone_5: format!("{:.8}", price_zone_5),
        zone_4: format!("{:.8}", price_zone_4),
        zone_3: format!("{:.8}", price_zone_3),
        zone_2: format!("{:.8}", price_zone_2),
        zone_1: format!("{:.8}", price_zone_1),
        zone_min: format!("{:.8}", min_low),
        of,
    };

    let last_blockchain_trade = {
        let chain = BLOCKCHAIN.lock().unwrap();
        chain.get_last_trade()
    };

    match last_blockchain_trade {
        Some(ref last) => update_status(trade, last),
        None => trade,
    }
}

pub fn calculate_moving_average(candles: &[Candlestick]) -> f64 {
    let soma: f64 = candles
        .iter()
        .filter_map(|c| c.close_price.parse::<f64>().ok())
        .sum();

    soma / candles.len() as f64
}

pub fn update_status(mut trade: Trade, last: &Trade) -> Trade {

    if trade.bias != last.bias {
        trade.status = None;
        return trade;
    }
    
    let current_price = parse(&trade.current_price);

    let zone_1 = parse(&trade.zone_1);
    let zone_2 = parse(&trade.zone_2);
    let zone_3 = parse(&trade.zone_3);
    let zone_5 = parse(&trade.zone_5);
    let zone_6 = parse(&trade.zone_6);
    let zone_7 = parse(&trade.zone_7);

    match trade.bias {
        Bias::Bullish => handle_bullish_status(&mut trade, current_price, zone_1, zone_3, zone_5, zone_6, zone_7, last),
        Bias::Bearish => handle_bearish_status(&mut trade, current_price, zone_1, zone_2, zone_3, zone_5, zone_7, last),
        Bias::None => {
            trade.status = None;
        }
    }

    trade
}

fn handle_bullish_status(
    trade: &mut Trade,
    current_price: f64,
    zone_1: f64,
    zone_3: f64,
    zone_5: f64,
    zone_6: f64,
    zone_7: f64,
    _last: &Trade,
) {
    if _last.status.is_none() && current_price >= zone_7 {
        trade.status = Some(TradeStatus::InZone7);
    } else if current_price > zone_5 && _last.status == Some(TradeStatus::InZone7) {
        trade.status = Some(TradeStatus::InZone7);
    } else if _last.status.is_none() && current_price <= zone_1 {
        trade.status = Some(TradeStatus::PrepareZone1);
    } else if current_price < zone_3 && _last.status == Some(TradeStatus::PrepareZone1) {
        trade.status = Some(TradeStatus::PrepareZone1);
    } else if current_price >= zone_7 && _last.status == Some(TradeStatus::OutZone5) {
        trade.status = Some(TradeStatus::InZone7);
    } else if current_price <= zone_5 && _last.status == Some(TradeStatus::InZone7) {
        trade.status = Some(TradeStatus::OutZone5);
    } else if current_price <= zone_1 && _last.status == Some(TradeStatus::OutZone5) {
        trade.status = Some(TradeStatus::PrepareZone1);
    } else if current_price >= zone_3 && _last.status == Some(TradeStatus::PrepareZone1) {
        trade.status = Some(TradeStatus::InZone3);
    } else if current_price <= zone_1 && _last.status == Some(TradeStatus::InZone3) {
        trade.status = Some(TradeStatus::PrepareZone1Long);
    } else if current_price >= zone_3 && _last.status == Some(TradeStatus::PrepareZone1Long) {
        trade.status = Some(TradeStatus::LongZone3);
    } else if current_price <= zone_1 && _last.status == Some(TradeStatus::LongZone3) {
        trade.status = Some(TradeStatus::PrepareZone1);
    } else if current_price >= zone_7 && _last.status == Some(TradeStatus::LongZone3) {
        trade.status = Some(TradeStatus::TargetLongZone7);
    } else if current_price > zone_6 && _last.status == Some(TradeStatus::TargetLongZone7) {
        trade.status = Some(TradeStatus::TargetLongZone7);
    } else if current_price <= zone_6 && _last.status == Some(TradeStatus::TargetLongZone7) {
        trade.status = None;
    } else if current_price >= zone_7 && _last.status == Some(TradeStatus::InZone3) {
        trade.status = Some(TradeStatus::InZone7);
    }
}

fn handle_bearish_status(
    trade: &mut Trade,
    current_price: f64,
    zone_1: f64,
    zone_2: f64,
    zone_3: f64,
    zone_5: f64,
    zone_7: f64,
    _last: &Trade,
) {
    if _last.status.is_none() && current_price <= zone_1 {
        trade.status = Some(TradeStatus::InZone1);
    } else if current_price < zone_3 && _last.status == Some(TradeStatus::InZone1) {
        trade.status = Some(TradeStatus::InZone1);
    } else if _last.status.is_none() && current_price >= zone_7 {
        trade.status = Some(TradeStatus::PrepareZone7);
    } else if current_price > zone_5 && _last.status == Some(TradeStatus::PrepareZone7) {
        trade.status = Some(TradeStatus::PrepareZone7);
    } else if current_price <= zone_1 && _last.status == Some(TradeStatus::OutZone3) {
        trade.status = Some(TradeStatus::InZone1);
    } else if current_price >= zone_3 && _last.status == Some(TradeStatus::InZone1) {
        trade.status = Some(TradeStatus::OutZone3);
    } else if current_price >= zone_7 && _last.status == Some(TradeStatus::OutZone3) {
        trade.status = Some(TradeStatus::PrepareZone7);
    } else if current_price <= zone_5 && _last.status == Some(TradeStatus::PrepareZone7) {
        trade.status = Some(TradeStatus::InZone5);
    } else if current_price >= zone_7 && _last.status == Some(TradeStatus::InZone5) {
        trade.status = Some(TradeStatus::PrepareZone7Short);
    } else if current_price <= zone_5 && _last.status == Some(TradeStatus::PrepareZone7Short) {
        trade.status = Some(TradeStatus::ShortZone5);
    } else if current_price >= zone_7 && _last.status == Some(TradeStatus::ShortZone5) {
        trade.status = Some(TradeStatus::PrepareZone7);
    } else if current_price <= zone_1 && _last.status == Some(TradeStatus::ShortZone5) {
        trade.status = Some(TradeStatus::TargetShortZone1);
    } else if current_price < zone_2 && _last.status == Some(TradeStatus::TargetShortZone1) {
        trade.status = Some(TradeStatus::TargetShortZone1);
    } else if current_price >= zone_2 && _last.status == Some(TradeStatus::TargetShortZone1) {
        trade.status = None;
    } else if current_price <= zone_1 && _last.status == Some(TradeStatus::InZone5) {
        trade.status = Some(TradeStatus::InZone1);
    }
}

fn parse(value: &str) -> f64 {
    value.parse::<f64>().unwrap_or(0.0)
}

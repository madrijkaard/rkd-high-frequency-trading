use crate::dto::{Candlestick, Trade};

pub fn generate_trade(candlesticks: Vec<Candlestick>) -> Trade {
    
    let of = candlesticks.len();

    let max_high = candlesticks
        .iter()
        .filter_map(|c| c.high_price.parse::<f64>().ok())
        .fold(f64::MIN, f64::max);

    let min_low = candlesticks
        .iter()
        .filter_map(|c| c.low_price.parse::<f64>().ok())
        .fold(f64::MAX, f64::min);

    let current_price = candlesticks
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

    Trade {
        zone_max: format!("{:.8}", max_high),
        zone_7: format!("{:.8}", price_zone_7),
        zone_6: format!("{:.8}", price_zone_6),
        zone_5: format!("{:.8}", price_zone_5),
        zone_4: format!("{:.8}", price_zone_4),
        zone_3: format!("{:.8}", price_zone_3),
        zone_2: format!("{:.8}", price_zone_2),
        zone_1: format!("{:.8}", price_zone_1),
        zone_min: format!("{:.8}", min_low),
        current_price,
        of,
    }
}

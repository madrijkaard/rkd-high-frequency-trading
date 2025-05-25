use crate::blockchain::BLOCKCHAIN;
use crate::dto::{Bias, Candlestick, Trade};
use crate::status_trade::update_status;

pub fn generate_trade(candlesticks: Vec<Candlestick>, reference_candles: Vec<Candlestick>) -> Trade {
    
    let of = candlesticks.len();
    let reference_of = reference_candles.len();

    if of < 271 || reference_of < 271 {
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

    let cma_valor = calculate_moving_average(&reference_candles[71..]);
    let oma_valor = calculate_moving_average(&reference_candles[..200]);

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

    let trade = Trade {
        current_price,
        cma: format!("{:.8}", cma_valor),
        oma: format!("{:.8}", oma_valor),
        bias,
        status: None,
        zone_max: format!("{:.8}", max_high),
        zone_7: format!("{:.8}", log_zone_7.exp()),
        zone_6: format!("{:.8}", log_zone_6.exp()),
        zone_5: format!("{:.8}", log_zone_5.exp()),
        zone_4: format!("{:.8}", log_zone_4.exp()),
        zone_3: format!("{:.8}", log_zone_3.exp()),
        zone_2: format!("{:.8}", log_zone_2.exp()),
        zone_1: format!("{:.8}", log_zone_1.exp()),
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

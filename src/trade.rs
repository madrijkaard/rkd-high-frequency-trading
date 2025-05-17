use crate::dto::{Bias, Candlestick, Trade, TradeStatus};

pub fn generate_trade(candlesticks: Vec<Candlestick>) -> Trade {
    let of = candlesticks.len();

    let cma_valor = calculate_moving_average(&candlesticks[24..]);
    let oma_valor = calculate_moving_average(&candlesticks[..200]);

    let bias = if cma_valor > oma_valor {
        Bias::Bullish
    } else if cma_valor < oma_valor {
        Bias::Bearish
    } else {
        Bias::None
    };

    let analysis_slice = &candlesticks[24..];

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

    update_status(trade)
}

pub fn calculate_moving_average(candles: &[Candlestick]) -> f64 {
    let soma: f64 = candles
        .iter()
        .filter_map(|c| c.close_price.parse::<f64>().ok())
        .sum();

    soma / candles.len() as f64
}

pub fn update_status(mut trade: Trade) -> Trade {
    let current_price = parse(&trade.current_price);
    let zone_1 = parse(&trade.zone_1);
    let zone_3 = parse(&trade.zone_3);
    let zone_5 = parse(&trade.zone_5);
    let zone_7 = parse(&trade.zone_7);

    match trade.bias {
        Bias::Bullish => {
            if current_price >= zone_7 {
                trade.status = Some(TradeStatus::InZone7);
            } else if current_price <= zone_5 {
                trade.status = Some(TradeStatus::OutZone5);
            } else if current_price <= zone_1 {
                trade.status = Some(TradeStatus::PrepareZone1);
            }

            match trade.status {
                Some(TradeStatus::PrepareZone1) if current_price >= zone_3 => {
                    trade.status = Some(TradeStatus::InZone3);
                }
                Some(TradeStatus::InZone3) if current_price <= zone_1 => {
                    trade.status = Some(TradeStatus::PrepareZone1Long);
                }
                Some(TradeStatus::PrepareZone1Long) if current_price >= zone_3 => {
                    trade.status = Some(TradeStatus::LongZone3);
                }
                Some(TradeStatus::LongZone3) if current_price <= zone_1 => {
                    trade.status = Some(TradeStatus::PrepareZone1);
                }
                Some(TradeStatus::InZone3) | Some(TradeStatus::LongZone3)
                    if current_price >= zone_7 =>
                {
                    trade.status = Some(TradeStatus::TargetLongZone7);
                }
                _ => {}
            }
        }

        Bias::Bearish => {
            if current_price <= zone_1 {
                trade.status = Some(TradeStatus::InZone1);
            } else if current_price >= zone_3 {
                trade.status = Some(TradeStatus::OutZone3);
            } else if current_price >= zone_7 {
                trade.status = Some(TradeStatus::PrepareZone7);
            }

            match trade.status {
                Some(TradeStatus::PrepareZone7) if current_price <= zone_5 => {
                    trade.status = Some(TradeStatus::InZone5);
                }
                Some(TradeStatus::InZone5) if current_price >= zone_7 => {
                    trade.status = Some(TradeStatus::PrepareZone7Short);
                }
                Some(TradeStatus::PrepareZone7Short) if current_price <= zone_5 => {
                    trade.status = Some(TradeStatus::ShortZone5);
                }
                Some(TradeStatus::ShortZone5) if current_price >= zone_7 => {
                    trade.status = Some(TradeStatus::PrepareZone7);
                }
                Some(TradeStatus::InZone5) | Some(TradeStatus::ShortZone5)
                    if current_price <= zone_1 =>
                {
                    trade.status = Some(TradeStatus::TargetShortZone1);
                }
                _ => {}
            }
        }

        Bias::None => {
            
        }
    }

    trade
}

fn parse(value: &str) -> f64 {
    value.parse::<f64>().unwrap_or(0.0)
}

use crate::dto::Trade;

pub fn generate_trade_zones(
    max_high: f64,
    min_low: f64,
    current_price: String,
    of: usize,
) -> Trade {
    let log_min = min_low.ln();
    let log_max = max_high.ln();
    let log_middle = (log_min + log_max) / 2.0;
    let log_mid_min = (log_min + log_middle) / 2.0;
    let log_mid_max = (log_max + log_middle) / 2.0;
    let log_mid_min_inner = (log_mid_min + log_middle) / 2.0;
    let log_mid_max_inner = (log_mid_max + log_middle) / 2.0;
    let log_below_min = (log_min + log_mid_min) / 2.0;
    let log_above_max = (log_max + log_mid_max) / 2.0;

    let price_middle = log_middle.exp();
    let price_mid_min = log_mid_min.exp();
    let price_mid_max = log_mid_max.exp();
    let price_mid_min_inner = log_mid_min_inner.exp();
    let price_mid_max_inner = log_mid_max_inner.exp();
    let price_below_min = log_below_min.exp();
    let price_above_max = log_above_max.exp();

    Trade {
        zone_max: format!("{:.8}", max_high),
        zone_7: format!("{:.8}", price_above_max),
        zone_6: format!("{:.8}", price_mid_max),
        zone_5: format!("{:.8}", price_mid_max_inner),
        zone_4: format!("{:.8}", price_middle),
        zone_3: format!("{:.8}", price_mid_min_inner),
        zone_2: format!("{:.8}", price_mid_min),
        zone_1: format!("{:.8}", price_below_min),
        zone_min: format!("{:.8}", min_low),
        current_price,
        of,
    }
}

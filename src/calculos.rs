use crate::dto::MaxMinPrice;

pub fn calcular_max_min_price(
    max_high: f64,
    min_low: f64,
    current_price: f64,
    of: usize,
) -> MaxMinPrice {
    
    let log_range = (max_high.ln()) - (min_low.ln());
    let log_base = min_low.ln();

    let zone1 = (log_base + log_range / 4.0).exp();
    let zone2 = (log_base + log_range / 2.0).exp();
    let zone3 = (log_base + 3.0 * log_range / 4.0).exp();

    let mid_zone1_zone2 = (zone1 + zone2) / 2.0;
    let mid_zone2_zone3 = (zone2 + zone3) / 2.0;

    let intermediate_price_1 = (zone1 + mid_zone1_zone2) / 2.0;
    let intermediate_price_2 = (zone3 + mid_zone2_zone3) / 2.0;

    MaxMinPrice {
        max_high_price: format!("{:.2}", max_high),
        min_low_price: format!("{:.2}", min_low),
        current_price: format!("{:.2}", current_price),
        intermediate_price_1: format!("{:.2}", intermediate_price_1),
        intermediate_price_2: format!("{:.2}", intermediate_price_2),
        of,
    }
}

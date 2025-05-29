use crate::dto::{Bias, Trade, TradeStatus};
use crate::blockchain::{remove_blockchain, get_blockchain_for};

pub fn remove_if_out_of_zone(trade: &Trade) {
    use TradeStatus::*;

    if matches!(trade.bias, Bias::Bullish) && matches!(trade.status, Some(OutZone5))
        || matches!(trade.bias, Bias::Bearish) && matches!(trade.status, Some(OutZone3))
    {
        remove_blockchain(&trade.symbol);
        return;
    }

    if let Some(blocks) = get_blockchain_for(&trade.symbol) {
        if blocks.len() >= 2 {
            let last_status = blocks[blocks.len() - 1].trade.status.clone();
            let previous_status = blocks[blocks.len() - 2].trade.status.clone();

            match trade.bias {
                Bias::Bullish => {
                    if (last_status == Some(PrepareZone1) && previous_status == Some(LongZone3))
                        || (last_status == None && previous_status == Some(TargetZone7))
                    {
                        remove_blockchain(&trade.symbol);
                        return;
                    }
                }
                Bias::Bearish => {
                    if (last_status == Some(PrepareZone7) && previous_status == Some(ShortZone5))
                        || (last_status == None && previous_status == Some(TargetZone1))
                    {
                        remove_blockchain(&trade.symbol);
                        return;
                    }
                }
                _ => {}
            }
        }
    }

    if trade.status.is_none() {
        let price = parse(&trade.current_price);
        let z4 = parse(&trade.zone_4);
        let z5 = parse(&trade.zone_5);

        if price > z4 && price <= z5 {
            remove_blockchain(&trade.symbol);
        }
    }
}

fn parse(value: &str) -> f64 {
    value.parse::<f64>().unwrap_or(0.0)
}

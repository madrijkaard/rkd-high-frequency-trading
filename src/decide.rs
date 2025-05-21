use crate::blockchain::BLOCKCHAIN;
use crate::dto::{Bias, TradeStatus};
use crate::order::{execute_future_order, close_all_positions};
use crate::config::BinanceSettings;
use crate::leverage::set_leverage_with_value;

pub fn decide(binance_settings: &BinanceSettings) {
    
    let trade = {
        let chain = BLOCKCHAIN.lock().unwrap();
        match chain.get_last_trade() {
            Some(t) => t,
            None => {
                println!("No trades found for decision");
                return;
            }
        }
    };

    let bias = trade.bias.clone();
    let status = trade.status.clone();

    match (bias, status) {

        (_, None) => {
            let binance = binance_settings.clone();
            tokio::spawn(async move {
                if let Err(e) = set_leverage_with_value(&binance, 1).await {
                    eprintln!("Error setting leverage to 1 (status None): {}", e);
                }
                match close_all_positions(&binance).await {
                    Ok(closed) => println!("All positions closed (status None): {:?}", closed),
                    Err(e) => eprintln!("Error closing positions (status None): {}", e),
                }
            });
        }

        (Bias::Bullish, Some(TradeStatus::InZone7))
        | (Bias::Bullish, Some(TradeStatus::InZone3))
        | (Bias::Bullish, Some(TradeStatus::LongZone3)) => {
            let binance = binance_settings.clone();
            tokio::spawn(async move {
                match execute_future_order(&binance, "BUY").await {
                    Ok(order) => println!("BUY order executed: {:?}", order),
                    Err(e) => eprintln!("Error executing BUY order: {}", e),
                }
            });
        }

        (Bias::Bearish, Some(TradeStatus::InZone1))
        | (Bias::Bearish, Some(TradeStatus::InZone5))
        | (Bias::Bearish, Some(TradeStatus::ShortZone5)) => {
            let binance = binance_settings.clone();
            tokio::spawn(async move {
                match execute_future_order(&binance, "SELL").await {
                    Ok(order) => println!("SALE order executed: {:?}", order),
                    Err(e) => eprintln!("Error executing SALE order: {}", e),
                }
            });
        }

        (Bias::Bullish, Some(TradeStatus::OutZone5))
        | (Bias::Bullish, Some(TradeStatus::PrepareZone1))
        | (Bias::Bullish, Some(TradeStatus::TargetLongZone7))
        | (Bias::Bearish, Some(TradeStatus::OutZone3))
        | (Bias::Bearish, Some(TradeStatus::PrepareZone7))
        | (Bias::Bearish, Some(TradeStatus::TargetShortZone1)) => {
            let binance = binance_settings.clone();
            tokio::spawn(async move {
                if let Err(e) = set_leverage_with_value(&binance, 1).await {
                    eprintln!("Error setting leverage to 1: {}", e);
                }
                match close_all_positions(&binance).await {
                    Ok(closed) => println!("Closed positions (lev 1): {:?}", closed),
                    Err(e) => eprintln!("Error closing positions: {}", e),
                }
            });
        }

        (Bias::Bullish, Some(TradeStatus::PrepareZone1Long))
        | (Bias::Bearish, Some(TradeStatus::PrepareZone7Short)) => {
            let binance = binance_settings.clone();
            tokio::spawn(async move {
                if let Err(e) = set_leverage_with_value(&binance, 2).await {
                    eprintln!("Error setting leverage to 2: {}", e);
                }
                match close_all_positions(&binance).await {
                    Ok(closed) => println!("Closed positions (lev 2): {:?}", closed),
                    Err(e) => eprintln!("Error closing positions: {}", e),
                }
            });
        }

        _ => {
            println!("No action taken for status: {:?} with bias: {:?}", trade.status, trade.bias);
        }
    }
}

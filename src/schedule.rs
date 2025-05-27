use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::interval;
use once_cell::sync::Lazy;

use crate::trade::generate_trade;
use crate::config::Settings;
use crate::candlestick::get_candlesticks;
use crate::blockchain::add_trade_block;
use crate::decide::decide;
use crate::log::{log_current_zone, log_spied_cryptos};
use crate::spy::spy_cryptos;
use crate::dto::Trade;

static SCHEDULER: Lazy<Arc<Mutex<Scheduler>>> = Lazy::new(|| Arc::new(Mutex::new(Scheduler::new())));

pub struct Scheduler {
    active: bool,
    handle: Option<JoinHandle<()>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            active: false,
            handle: None,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn start(&mut self) {
        if self.active {
            return;
        }

        self.active = true;
        let settings = Settings::load();
        let binance_settings = settings.binance.clone();
        let spy_enabled = settings.spy;
        let cryptos = settings.cryptos.clone();

        self.handle = Some(tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(50));

            loop {
                interval.tick().await;

                let candlesticks = match get_candlesticks(
                    &binance_settings.base_url,
                    &binance_settings.symbol,
                    &binance_settings.interval,
                    binance_settings.limit,
                ).await {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Error getting candles: {}", e);
                        continue;
                    }
                };

                let ref_candlesticks = match get_candlesticks(
                    &binance_settings.base_url,
                    "BTCUSDT",
                    &binance_settings.interval,
                    binance_settings.limit,
                ).await {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Error getting BTCUSDT candles: {}", e);
                        continue;
                    }
                };

                let trade = generate_trade(
                    binance_settings.symbol.clone(),
                    candlesticks,
                    ref_candlesticks,
                );

                if spy_enabled {
                    let trades: Vec<Trade> = spy_cryptos(
                        &binance_settings.base_url,
                        &binance_settings.interval,
                        binance_settings.limit,
                        cryptos.clone(),
                    ).await;
                    log_spied_cryptos(&trades);
                } else {
                    log_current_zone(&trade);
                }

                let was_added = add_trade_block(trade.clone());

                if was_added && binance_settings.decide {
                    decide(&trade.symbol, &binance_settings);
                }
            }
        }));
    }

    pub fn stop(&mut self) {
        self.active = false;
        if let Some(handle) = self.handle.take() {
            handle.abort();
        }
    }
}

pub fn get_scheduler() -> Arc<Mutex<Scheduler>> {
    SCHEDULER.clone()
}

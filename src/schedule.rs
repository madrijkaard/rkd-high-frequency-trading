use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::interval;
use once_cell::sync::Lazy;

use crate::config::Settings;
use crate::dto::{Bias, Trade, TradeStatus};
use crate::spy::spy_cryptos;
use crate::blockchain::{
    add_trade_block,
    is_blockchain_limit_reached,
    get_current_blockchain_symbols,
    remove_blockchain,
};
use crate::decide::decide;
use crate::log::log_spied_cryptos;

use rand::seq::SliceRandom;
use rand::thread_rng;

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

        self.handle = Some(tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(50));

            loop {
                interval.tick().await;
                choose_crypto(&settings).await;
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

async fn choose_crypto(settings: &Settings) {
    let current_symbols = get_current_blockchain_symbols();

    let trades = spy_cryptos(
        &settings.binance.base_url,
        &settings.binance.interval,
        settings.binance.limit,
        settings.cryptos.clone(),
    )
    .await;

    log_spied_cryptos(&trades);

    let existing_trades: Vec<Trade> = trades
        .iter()
        .filter(|t| current_symbols.contains(&t.symbol))
        .cloned()
        .collect();

    for trade in &existing_trades {
        let was_added = add_trade_block(trade.clone());
        if was_added && settings.binance.decide {
            decide(&trade.symbol, &settings.binance);

            if matches!(trade.bias, Bias::Bullish) && matches!(trade.status, Some(TradeStatus::OutZone5))
                || matches!(trade.bias, Bias::Bearish) && matches!(trade.status, Some(TradeStatus::OutZone3))
            {
                remove_blockchain(&trade.symbol);
            }
        }
    }

    let mut rng = thread_rng();

    let mut candidates: Vec<Trade> = trades
        .into_iter()
        .filter(|t| !current_symbols.contains(&t.symbol))
        .filter(|t| match t.bias {
            Bias::Bullish => {
                let p = parse(&t.current_price);
                let z1 = parse(&t.zone_1);
                let z7 = parse(&t.zone_7);
                p <= z1 || (p > parse(&t.zone_6) && p <= z7)
            }
            Bias::Bearish => {
                let p = parse(&t.current_price);
                let z2 = parse(&t.zone_2);
                let z7 = parse(&t.zone_7);
                p <= z2 || (p > parse(&t.zone_6) && p <= z7)
            }
            _ => false,
        })
        .collect();

    candidates.shuffle(&mut rng);

    for trade in candidates {
        if is_blockchain_limit_reached() {
            break;
        }
        let was_added = add_trade_block(trade.clone());
        if was_added && settings.binance.decide {
            decide(&trade.symbol, &settings.binance);

            if matches!(trade.bias, Bias::Bullish) && matches!(trade.status, Some(TradeStatus::OutZone5))
                || matches!(trade.bias, Bias::Bearish) && matches!(trade.status, Some(TradeStatus::OutZone3))
            {
                remove_blockchain(&trade.symbol);
            }
        }
    }
}

fn parse(value: &str) -> f64 {
    value.parse::<f64>().unwrap_or(0.0)
}

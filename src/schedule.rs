use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::interval;
use once_cell::sync::Lazy;

use crate::trade::generate_trade;
use crate::config::Settings;
use crate::client::get_candlesticks;
use crate::blockchain::BLOCKCHAIN;
use crate::decide::decide;
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

        self.handle = Some(tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(50));

            loop {
                interval.tick().await;

                let candlesticks = match get_candlesticks(&binance_settings).await {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Error getting candles: {}", e);
                        continue;
                    }
                };

                let trade = generate_trade(candlesticks);

                log_current_zone(&trade);

                let was_added = {
                    let mut chain = BLOCKCHAIN.lock().unwrap();
                    chain.add_block(trade.clone())
                };

                if was_added {
                    decide(&binance_settings);
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

fn log_current_zone(trade: &Trade) {
    let current_price = parse(&trade.current_price);
    let z7 = parse(&trade.zone_7);
    let z6 = parse(&trade.zone_6);
    let z5 = parse(&trade.zone_5);
    let z4 = parse(&trade.zone_4);
    let z3 = parse(&trade.zone_3);
    let z2 = parse(&trade.zone_2);
    let z1 = parse(&trade.zone_1);

    let (zona_a, zona_b) = if current_price > z7 {
        ("MAX", "Z7")
    } else if current_price > z6 && current_price <= z7 {
        ("Z7", "Z6")
    } else if current_price > z5 && current_price <= z6 {
        ("Z6", "Z5")
    } else if current_price > z4 && current_price <= z5 {
        ("Z5", "Z4")
    } else if current_price > z3 && current_price <= z4 {
        ("Z4", "Z3")
    } else if current_price > z2 && current_price <= z3 {
        ("Z3", "Z2")
    } else if current_price > z1 && current_price <= z2 {
        ("Z2", "Z1")
    } else {
        ("Z1", "MIN")
    };

    println!("[{}] - {} is between {} and {}", 
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
    trade.current_price, 
    zona_a, 
    zona_b);
}

fn parse(value: &str) -> f64 {
    value.parse::<f64>().unwrap_or(0.0)
}


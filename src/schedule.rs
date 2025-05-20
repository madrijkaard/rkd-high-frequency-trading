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
                        eprintln!("Erro ao obter candles: {}", e);
                        continue;
                    }
                };

                let trade = generate_trade(candlesticks);

                let was_added = {
                    let mut chain = BLOCKCHAIN.lock().unwrap();
                    chain.add_block(trade.clone())
                };

                if was_added {
                    decide(&binance_settings);
                    println!(
                        "[{}] Novo bloco adicionado - Preco atual: {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                        trade.current_price
                    );
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

use crate::dto::Trade;
use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeBlock {
    pub index: u64,
    pub timestamp: u64,
    pub trade: Trade,
    pub previous_hash: String,
    pub hash: String,
}

impl TradeBlock {
    pub fn new(index: u64, trade: Trade, previous_hash: String) -> Self {
        let timestamp = current_timestamp();
        let hash = Self::calculate_hash(index, timestamp, &trade, &previous_hash);
        TradeBlock {
            index,
            timestamp,
            trade,
            previous_hash,
            hash,
        }
    }

    pub fn calculate_hash(index: u64, timestamp: u64, trade: &Trade, previous_hash: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_be_bytes());
        hasher.update(timestamp.to_be_bytes());
        hasher.update(serde_json::to_string(trade).unwrap());
        hasher.update(previous_hash.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub struct TradeBlockchain {
    chain: Vec<TradeBlock>,
}

impl TradeBlockchain {
    pub fn new() -> Self {
        Self { chain: vec![] }
    }

    pub fn add_block(&mut self, trade: Trade) -> bool {
        
        if let Some(last_trade) = self.get_last_trade() {
            if trade.status == last_trade.status {
                return false;
            }
        }

        let index = self.chain.len() as u64;
        let previous_hash = self.chain.last()
            .map(|b| b.hash.clone())
            .unwrap_or_else(|| "0".to_string());

        let new_block = TradeBlock::new(index, trade.clone(), previous_hash);
        self.chain.push(new_block);

        println!();
        println!();
        println!();
        
        println!(
            "[{}] - New block added - Status: {:?}, Price: {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            trade.status,
            trade.current_price
        );

        println!();
        println!();
        println!();

        true
    }

    pub fn get_last_trade(&self) -> Option<Trade> {
        self.chain.last().map(|block| block.trade.clone())
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.previous_hash != previous.hash {
                return false;
            }

            let recalculated_hash = TradeBlock::calculate_hash(
                current.index,
                current.timestamp,
                &current.trade,
                &current.previous_hash,
            );

            if current.hash != recalculated_hash {
                return false;
            }
        }
        true
    }

    pub fn all(&self) -> &[TradeBlock] {
        &self.chain
    }
}

pub static BLOCKCHAIN: Lazy<Mutex<TradeBlockchain>> = Lazy::new(|| {
    Mutex::new(TradeBlockchain::new())
});

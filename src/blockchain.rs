use crate::dto::Trade;
use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

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
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub struct TradeBlockchain {
    pub chain: Vec<TradeBlock>,
}

impl TradeBlockchain {
    pub fn new() -> Self {
        let genesis = Trade {
            zone_max: "0.0".into(),
            zone_7: "0.0".into(),
            zone_6: "0.0".into(),
            zone_5: "0.0".into(),
            zone_4: "0.0".into(),
            zone_3: "0.0".into(),
            zone_2: "0.0".into(),
            zone_1: "0.0".into(),
            zone_min: "0.0".into(),
            current_price: "0.0".into(),
            of: 0,
        };
        let genesis_block = TradeBlock::new(0, genesis, "0".into());
        Self {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, trade: Trade) {
        let last_block = self.chain.last().unwrap();
        let new_block = TradeBlock::new(last_block.index + 1, trade, last_block.hash.clone());
        self.chain.push(new_block);
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

    pub fn all(&self) -> &Vec<TradeBlock> {
        &self.chain
    }
}

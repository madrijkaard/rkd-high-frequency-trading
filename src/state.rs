use std::sync::Mutex;
use crate::blockchain::TradeBlockchain;

use once_cell::sync::Lazy;

pub static BLOCKCHAIN: Lazy<Mutex<TradeBlockchain>> = Lazy::new(|| {
    Mutex::new(TradeBlockchain::new())
});

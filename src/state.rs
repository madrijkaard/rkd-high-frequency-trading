use std::sync::Mutex;
use crate::blockchain::TradeBlockchain;

use once_cell::sync::Lazy;

// Estado global da blockchain (você pode injetar via AppData se preferir)
pub static BLOCKCHAIN: Lazy<Mutex<TradeBlockchain>> = Lazy::new(|| {
    Mutex::new(TradeBlockchain::new())
});


use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct TickerMain{
    pub ticker: Option<Ticker>
}

#[derive(Serialize, Deserialize)]
pub struct Ticker {
    pub high: String,
    pub low: String,
    pub vol: String,
    pub buy: String,
    pub sell: String,
    pub date: u64,
    pub last: String
}
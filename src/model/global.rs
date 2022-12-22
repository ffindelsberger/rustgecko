use crate::model::common::AllCurrencies;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Global {
    active_cryptocurrencies: i64,
    upcoming_icos: i64,
    ongoing_icos: i64,
    ended_icos: i64,
    markets: i64,
    total_market_cap: AllCurrencies,
    total_volume: AllCurrencies,
    market_cap_percentage: AllCurrencies,
    market_cap_change_percentage_24h_usd: f64,
    updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GlobalData {
    data: Global,
}

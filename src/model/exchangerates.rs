use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ExchangeRateItem {
    name: String,
    unit: String,
    value: f64,
    #[serde(rename = "type")]
    exchange_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ExchangeRates {
    rates: HashMap<String, ExchangeRateItem>,
}

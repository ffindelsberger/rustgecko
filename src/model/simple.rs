use serde::{Deserialize, Serialize};
use std::collections::HashMap;

///
/// The Symbol is provided in lowercase letters
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoinListing {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub platforms: Option<HashMap<String, Option<String>>>,
}

pub type ContractAddress = String;

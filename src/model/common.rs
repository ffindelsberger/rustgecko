use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Price = HashMap<String, f64>;
pub type Localization = HashMap<String, String>;
pub type AllCurrencies = HashMap<String, Option<f64>>;
pub type SupportedVsCurrencies = Vec<String>;
pub type Description = HashMap<String, Option<String>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinksItem {
    pub homepage: Vec<Option<String>>,
    pub blockchain_site: Vec<Option<String>>,
    pub official_forum_url: Vec<Option<String>>,
    pub chat_url: Vec<Option<String>>,
    pub announcement_url: Vec<Option<String>>,
    pub twitter_screen_name: serde_json::Value,
    pub facebook_username: serde_json::Value,
    pub bitcointalk_thread_identifier: serde_json::Value,
    pub telegram_channel_identifier: serde_json::Value,
    pub subreddit_url: serde_json::Value,
    pub repos_url: ReposUrl,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReposUrl {
    pub github: Vec<Option<String>>,
    pub bitbucket: Vec<Option<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageItem {
    pub thumb: Option<String>,
    pub small: Option<String>,
    pub large: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicCoinInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ath {
    ath: AllCurrencies,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AthChangePercentage {
    ath_change_percentage: AllCurrencies,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AthDate {
    ath_date: HashMap<String, String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Atl {
    atl: AllCurrencies,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AtlChangePercentage {
    atl_change_percentage: AllCurrencies,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AtlDate {
    atl_date: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullyDilutedValuation {
    fully_diluted_valuation: AllCurrencies,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct High24H {
    high_24h: AllCurrencies,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Low24H {
    low_24h: AllCurrencies,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceChange24HInCurrency {
    price_change_24h_in_currency: AllCurrencies,
}

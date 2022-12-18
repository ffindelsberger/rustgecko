use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Price = HashMap<String, f64>;
pub type Localization = HashMap<String, String>;
pub type AllCurrencies = HashMap<String, Option<f64>>;
pub type SupportedVsCurrencies = Vec<String>;
pub type Description = HashMap<String, Option<String>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinksItem {
    pub homepage: Vec<String>,
    pub blockchain_site: Vec<Option<String>>,
    pub official_forum_url: Vec<String>,
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
    pub github: Vec<String>,
    pub bitbucket: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageItem {
    pub thumb: Option<String>,
    pub small: Option<String>,
    pub large: Option<String>,
}

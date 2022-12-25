use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
    pub twitter_screen_name: Option<String>,
    pub facebook_username: Option<String>,
    pub bitcointalk_thread_identifier: Option<i64>,
    pub telegram_channel_identifier: Option<String>,
    pub subreddit_url: Option<String>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeveloperData {
    pub forks: Option<f64>,
    pub stars: Option<f64>,
    pub subscribers: Option<f64>,
    pub total_issues: Option<f64>,
    pub closed_issues: Option<f64>,
    pub pull_requests_merged: Option<f64>,
    pub pull_request_contributors: Option<f64>,
    pub code_additions_deletions_4_weeks: CodeAdditionsDeletions4Weeks,
    pub commit_count_4_weeks: Option<f64>,
    pub last_4_weeks_commit_activity_series: Option<Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommunityData {
    pub facebook_likes: Option<i64>,
    pub twitter_followers: Option<i64>,
    pub reddit_average_posts_48h: Option<f64>,
    pub reddit_average_comments_48h: Option<f64>,
    pub reddit_subscribers: Option<i64>,
    pub reddit_accounts_active_48h: serde_json::Value,
    pub telegram_channel_user_count: Option<Option<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeAdditionsDeletions4Weeks {
    pub additions: Option<f64>,
    pub deletions: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ping {
    pub gecko_says: String,
}

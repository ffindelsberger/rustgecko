use crate::model::common::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//TODO: test this struct
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinsMarketItem {
    #[serde(flatten)]
    pub basic_info: BasicCoinInfo,
    pub image: String,
    pub current_price: Option<f64>,
    pub market_cap: Option<f64>,
    pub market_cap_rank: serde_json::Value,
    pub fully_diluted_valuation: serde_json::Value,
    pub total_volume: Option<f64>,
    pub high_24h: Option<f64>,
    pub low_24h: Option<f64>,
    pub price_change_24h: Option<f64>,
    pub price_change_percentage_24h: Option<f64>,
    pub market_cap_change_24h: Option<f64>,
    pub market_cap_change_percentage_24h: Option<f64>,
    pub circulating_supply: Option<f64>,
    pub total_supply: Option<f64>,
    pub max_supply: Option<f64>,
    pub ath: Option<f64>,
    pub ath_change_percentage: Option<f64>,
    pub ath_date: Option<String>,
    pub atl: Option<f64>,
    pub atl_change_percentage: Option<f64>,
    pub atl_date: Option<String>,
    pub roi: Option<RoiItem>,
    pub last_updated: Option<String>,
    pub sparkline_in_7d: Option<Sparkline7days>,
    pub price_change_percentage_1h_in_currency: Option<f64>,
    pub price_change_percentage_24h_in_currency: Option<f64>,
    pub price_change_percentage_7d_in_currency: Option<f64>,
    pub price_change_percentage_14d_in_currency: Option<f64>,
    pub price_change_percentage_30d_in_currency: Option<f64>,
    pub price_change_percentage_200d_in_currency: Option<f64>,
    pub price_change_percentage_1y_in_currency: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinHistoryItem {
    #[serde(flatten)]
    pub basic_info: BasicCoinInfo,
    pub localization: Localization,
    pub image: ImageItem,
    pub market_data: BasicMarketData,
    pub community_data: CommunityData,
    pub developer_data: DeveloperData,
    pub public_interest_stats: PublicInterestStats,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicMarketData {
    pub current_price: AllCurrencies,
    pub market_cap: AllCurrencies,
    pub total_volume: AllCurrencies,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusUpdateItem {
    description: Option<String>,
    category: Option<String>,
    created_at: Option<String>,
    user: Option<String>,
    user_title: Option<String>,
    pin: bool,
    project: Project,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    #[serde(flatten)]
    basic_info: BasicCoinInfo,
    #[serde(rename = "type")]
    project_type: String,
    image: ImageItem,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicInterestStats {
    pub alexa_rank: Option<f64>,
    pub bing_matches: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sparkline7days {
    pub price: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoiItem {
    pub times: Option<f64>,
    pub currency: String,
    pub percentage: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ticker {
    pub base: String,
    pub target: String,
    pub market: Market,
    pub last: f64,
    pub volume: f64,
    pub converted_last: HashMap<String, f64>,
    pub converted_volume: HashMap<String, f64>,
    pub cost_to_move_up_usd: Option<f64>,
    pub cost_to_move_down_usd: Option<f64>,
    pub trust_score: Option<String>,
    pub bid_ask_spread_percentage: Option<f64>,
    pub timestamp: Option<String>,
    pub last_traded_at: Option<String>,
    pub last_fetch_at: Option<String>,
    pub is_anomaly: bool,
    pub is_stale: bool,
    pub trade_url: Option<String>,
    pub token_info_url: Option<String>,
    pub coin_id: String,
    pub target_coin_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Market {
    pub name: String,
    pub identifier: String,
    pub has_trading_incentive: bool,
    pub logo: Option<String>,
}

/// Moddels the MarketChart Data as Arrays of Tuples (unix_timestamp, value)
#[derive(Debug, Serialize, Deserialize)]
pub struct MarketChart {
    pub prices: Vec<(i64, Option<f64>)>,
    pub market_caps: Vec<(i64, Option<f64>)>,
    pub total_volumes: Vec<(i64, Option<f64>)>,
}

/// Tuple representing Candle Data (time, open, high, low, close)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Candle {
    pub time: i64,
    pub open: i64,
    pub high: i64,
    pub low: i64,
    pub close: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AssetPlatform {
    pub id: String,
    pub chain_identifier: Option<i64>,
    pub name: String,
    pub shortname: String,
}

/*
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contract {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub asset_platform_id: String,
    pub platforms: Option<HashMap<String, Option<String>>>,
    pub block_time_in_minutes: i64,
    pub hashing_algorithm: serde_json::Value,
    pub categories: Vec<String>,
    pub public_notice: serde_json::Value,
    pub additional_notices: Vec<serde_json::Value>,
    pub localization: Localization,
    pub description: Description,
    pub links: LinksItem,
    pub image: ImageItem,
    pub country_origin: String,
    pub genesis_date: serde_json::Value,
    pub contract_address: String,
    pub sentiment_votes_up_percentage: f64,
    pub sentiment_votes_down_percentage: f64,
    pub market_cap_rank: i64,
    pub coingecko_rank: i64,
    pub coingecko_score: f64,
    pub developer_score: i64,
    pub community_score: f64,
    pub liquidity_score: f64,
    pub public_interest_score: f64,
    pub market_data: MarketData,
    pub community_data: CommunityData,
    pub developer_data: DeveloperData,
    pub public_interest_stats: PublicInterestStats,
    pub status_updates: Vec<serde_json::Value>,
    pub last_updated: String,
    pub tickers: Vec<Ticker>,*/
//}

use crate::model::common::{AllCurrencies, Description, ImageItem, LinksItem, Localization};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicCoinInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ping {
    pub gecko_says: String,
}

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
    localization: Localization,
    image: ImageItem,
    market_data: BasicMarketData,
    community_data: CommunityData,
    developer_data: DeveloperData,
    public_interest_stats: PublicInterestStats,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicMarketData {
    current_price: AllCurrencies,
    market_cap: AllCurrencies,
    total_volume: AllCurrencies,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinsItem {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub asset_platform_id: Option<String>,
    pub platforms: Option<HashMap<String, Option<String>>>,
    pub block_time_in_minutes: f64,
    pub hashing_algorithm: Option<String>,
    pub categories: Vec<String>,
    pub public_notice: Option<String>,
    pub additional_notices: Vec<String>,
    pub localization: Option<Localization>,
    pub description: Description,
    pub links: LinksItem,
    pub image: ImageItem,
    pub country_origin: Option<String>,
    pub genesis_date: Option<String>,
    pub contract_address: Option<String>,
    pub sentiment_votes_up_percentage: Option<f32>,
    pub sentiment_votes_down_percentage: Option<f32>,
    pub market_cap_rank: Option<i32>,
    pub coingecko_rank: Option<f32>,
    pub coingecko_score: Option<f32>,
    pub developer_score: Option<f32>,
    pub community_score: Option<f32>,
    pub liquidity_score: Option<f32>,
    pub public_interest_score: Option<f32>,
    pub market_data: Option<MarketData>,
    pub community_data: Option<CommunityData>,
    pub developer_data: Option<DeveloperData>,
    pub public_interest_stats: PublicInterestStats,
    pub status_updates: Vec<StatusUpdateItem>,
    pub last_updated: Option<String>,
    pub tickers: Option<Vec<Ticker>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusUpdateItem {
    description: String,
    category: String,
    created_at: String,
    user: String,
    user_title: String,
    pin: bool,
    project: Project,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    #[serde(flatten)]
    basinfo: BasicCoinInfo,
    #[serde(rename = "type")]
    project_type: String,
    image: ImageItem,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MarketData {
    pub current_price: AllCurrencies,
    pub total_value_locked: serde_json::Value,
    pub mcap_to_tvl_ratio: serde_json::Value,
    pub fdv_to_tvl_ratio: serde_json::Value,
    ////TODO: Put RoiItem here ?
    pub roi: serde_json::Value,
    #[serde(flatten)]
    pub ath: Ath,
    #[serde(flatten)]
    pub ath_change_percentage: AthChangePercentage,
    #[serde(flatten)]
    pub ath_date: AthDate,
    #[serde(flatten)]
    pub atl: Atl,
    #[serde(flatten)]
    pub atl_change_percentage: AtlChangePercentage,
    #[serde(flatten)]
    pub atl_date: AtlDate,
    pub market_cap: AllCurrencies,
    #[serde(flatten)]
    pub market_cap_rank: serde_json::Value,
    #[serde(flatten)]
    pub fully_diluted_valuation: FullyDilutedValuation,
    pub total_volume: AllCurrencies,
    #[serde(flatten)]
    pub high_24h: High24H,
    #[serde(flatten)]
    pub low_24h: Low24H,
    pub price_change_24h: Option<f64>,
    pub price_change_percentage_24h: Option<f64>,
    pub price_change_percentage_7d: Option<f64>,
    pub price_change_percentage_14d: Option<f64>,
    pub price_change_percentage_30d: Option<f64>,
    pub price_change_percentage_60d: Option<f64>,
    pub price_change_percentage_200d: Option<f64>,
    pub price_change_percentage_1y: Option<f64>,
    pub market_cap_change_24h: Option<f64>,
    pub market_cap_change_percentage_24h: Option<f64>,
    #[serde(flatten)]
    pub price_change_24h_in_currency: Option<PriceChange24HInCurrency>,
    pub price_change_percentage_1h_in_currency: Option<AllCurrencies>,
    pub price_change_percentage_24h_in_currency: Option<AllCurrencies>,
    pub price_change_percentage_7d_in_currency: Option<AllCurrencies>,
    pub price_change_percentage_14d_in_currency: Option<AllCurrencies>,
    pub price_change_percentage_30d_in_currency: Option<AllCurrencies>,
    pub price_change_percentage_60d_in_currency: Option<AllCurrencies>,
    pub price_change_percentage_200d_in_currency: Option<AllCurrencies>,
    pub price_change_percentage_1y_in_currency: Option<AllCurrencies>,
    pub market_cap_change_24h_in_currency: Option<AllCurrencies>,
    pub market_cap_change_percentage_24h_in_currency: Option<AllCurrencies>,
    pub total_supply: serde_json::Value,
    pub max_supply: serde_json::Value,
    pub circulating_supply: serde_json::Value,
    pub sparkline_7d: Option<Sparkline7days>,
    pub last_updated: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicInterestStats {
    pub alexa_rank: Option<f64>,
    pub bing_matches: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommunityData {
    pub facebook_likes: Option<i64>,
    pub twitter_followers: Option<i64>,
    pub reddit_average_posts_48h: Option<f64>,
    pub reddit_average_comments_48h: Option<f64>,
    pub reddit_subscribers: Option<i64>,
    pub reddit_accounts_active_48h: Option<Option<f64>>,
    pub telegram_channel_user_count: Option<Option<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sparkline7days {
    pub price: Vec<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoiItem {
    pub times: f64,
    pub currency: String,
    pub percentage: f64,
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
pub struct CodeAdditionsDeletions4Weeks {
    pub additions: Option<f64>,
    pub deletions: Option<f64>,
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ExchangeRateItem {
    name: String,
    unit: String,
    value: f64,
    #[serde(rename = "type")]
    exchange_type: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ExchangeRates {
    rates: HashMap<String, ExchangeRateItem>,
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

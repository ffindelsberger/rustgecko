use crate::model::apimodels::{PublicInterestStats, Sparkline7days, StatusUpdateItem, Ticker};
use crate::model::common::{
    AllCurrencies, Ath, AthChangePercentage, AthDate, Atl, AtlChangePercentage, AtlDate,
    CommunityData, Description, DeveloperData, FullyDilutedValuation, High24H, ImageItem,
    LinksItem, Localization, Low24H, PriceChange24HInCurrency,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ping {
    pub gecko_says: String,
}

///
/// The Symbol is provided in lowercase letters
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoinListing {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub platforms: Option<HashMap<String, Option<String>>>,
}

struct CoinPriceSimple {}

pub type SupportedVsCurrencies = Vec<String>;

pub enum PriceChange {
    Hours1,
    Hours24,
    Days7,
    Days14,
    Days30,
    Days200,
    Years1,
}

impl PriceChange {
    pub fn get_string(&self) -> &str {
        match self {
            PriceChange::Hours1 => "1h",
            PriceChange::Hours24 => "24h",
            PriceChange::Days7 => "7d",
            PriceChange::Days30 => "14d",
            PriceChange::Days200 => "30d",
            PriceChange::Years1 => "1y",
            PriceChange::Days14 => "14d",
        }
    }
}

impl AsRef<str> for PriceChange {
    fn as_ref(&self) -> &str {
        self.get_string()
    }
}

pub enum MarketOrder {
    MarketCapDesc,
    MarketCapAsc,
    GeckoDesc,
    GeckoAsc,
    VolumeAsc,
    VolumeDesc,
    IdAsc,
    IdDesc,
}

impl MarketOrder {
    pub fn get_string(&self) -> &str {
        match self {
            MarketOrder::MarketCapDesc => "market_cap_desc",
            MarketOrder::MarketCapAsc => "market_cap_asc",
            MarketOrder::GeckoDesc => "gecko_desc",
            MarketOrder::GeckoAsc => "gecko_asc",
            MarketOrder::VolumeAsc => "volume_asc",
            MarketOrder::VolumeDesc => "volume_desc",
            MarketOrder::IdAsc => "id_asc",
            MarketOrder::IdDesc => "id_desc",
        }
    }
}

impl AsRef<str> for MarketOrder {
    fn as_ref(&self) -> &str {
        self.get_string()
    }
}

enum TrustOrders {
    TrustScoreDesc,
    TrustScoreAsc,
    VolumeDesc,
}

impl TrustOrders {
    fn get_string(&self) -> String {
        match self {
            TrustOrders::TrustScoreDesc => "trust_score_desc".into(),
            TrustOrders::TrustScoreAsc => "trust_score_asc".into(),
            TrustOrders::VolumeDesc => "volume_desc".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinsMarketItem {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub image: String,
    pub current_price: Option<f64>,
    pub market_cap: Option<f64>,
    pub market_cap_rank: Value,
    pub fully_diluted_valuation: Value,
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
    pub price_change_percentage_14d_in_currency: Option<f64>,
    pub price_change_percentage_1h_in_currency: Option<f64>,
    pub price_change_percentage_1y_in_currency: Option<f64>,
    pub price_change_percentage_200d_in_currency: Option<f64>,
    pub price_change_percentage_24h_in_currency: Option<f64>,
    pub price_change_percentage_30d_in_currency: Option<f64>,
    pub price_change_percentage_7d_in_currency: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sparkline7days {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoiItem {
    pub times: f64,
    pub currency: String,
    pub percentage: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Price {
    pub btc: Option<f64>,
    pub btc_market_cap: Option<f64>,
    #[serde(rename = "btc_24h_vol")]
    pub btc24_h_vol: Option<f64>,
    #[serde(rename = "btc_24h_change")]
    pub btc24_h_change: Option<f64>,
    pub eth: Option<f64>,
    pub eth_market_cap: Option<f64>,
    #[serde(rename = "eth_24h_vol")]
    pub eth24_h_vol: Option<f64>,
    #[serde(rename = "eth_24h_change")]
    pub eth24_h_change: Option<f64>,
    pub ltc: Option<f64>,
    pub ltc_market_cap: Option<f64>,
    #[serde(rename = "ltc_24h_vol")]
    pub ltc24_h_vol: Option<f64>,
    #[serde(rename = "ltc_24h_change")]
    pub ltc24_h_change: Option<f64>,
    pub bch: Option<f64>,
    pub bch_market_cap: Option<f64>,
    #[serde(rename = "bch_24h_vol")]
    pub bch24_h_vol: Option<f64>,
    #[serde(rename = "bch_24h_change")]
    pub bch24_h_change: Option<f64>,
    pub bnb: Option<f64>,
    pub bnb_market_cap: Option<f64>,
    #[serde(rename = "bnb_24h_vol")]
    pub bnb24_h_vol: Option<f64>,
    #[serde(rename = "bnb_24h_change")]
    pub bnb24_h_change: Option<f64>,
    pub eos: Option<f64>,
    pub eos_market_cap: Option<f64>,
    #[serde(rename = "eos_24h_vol")]
    pub eos24_h_vol: Option<f64>,
    #[serde(rename = "eos_24h_change")]
    pub eos24_h_change: Option<f64>,
    pub xrp: Option<f64>,
    pub xrp_market_cap: Option<f64>,
    #[serde(rename = "xrp_24h_vol")]
    pub xrp24_h_vol: Option<f64>,
    #[serde(rename = "xrp_24h_change")]
    pub xrp24_h_change: Option<f64>,
    pub xlm: Option<f64>,
    pub xlm_market_cap: Option<f64>,
    #[serde(rename = "xlm_24h_vol")]
    pub xlm24_h_vol: Option<f64>,
    #[serde(rename = "xlm_24h_change")]
    pub xlm24_h_change: Option<f64>,
    pub link: Option<f64>,
    pub link_market_cap: Option<f64>,
    #[serde(rename = "link_24h_vol")]
    pub link24_h_vol: Option<f64>,
    #[serde(rename = "link_24h_change")]
    pub link24_h_change: Option<f64>,
    pub dot: Option<f64>,
    pub dot_market_cap: Option<f64>,
    #[serde(rename = "dot_24h_vol")]
    pub dot24_h_vol: Option<f64>,
    #[serde(rename = "dot_24h_change")]
    pub dot24_h_change: Option<f64>,
    pub yfi: Option<f64>,
    pub yfi_market_cap: Option<f64>,
    #[serde(rename = "yfi_24h_vol")]
    pub yfi24_h_vol: Option<f64>,
    #[serde(rename = "yfi_24h_change")]
    pub yfi24_h_change: Option<f64>,
    pub usd: Option<f64>,
    pub usd_market_cap: Option<f64>,
    #[serde(rename = "usd_24h_vol")]
    pub usd24_h_vol: Option<f64>,
    #[serde(rename = "usd_24h_change")]
    pub usd24_h_change: Option<f64>,
    pub aed: Option<f64>,
    pub aed_market_cap: Option<f64>,
    #[serde(rename = "aed_24h_vol")]
    pub aed24_h_vol: Option<f64>,
    #[serde(rename = "aed_24h_change")]
    pub aed24_h_change: Option<f64>,
    pub ars: Option<f64>,
    pub ars_market_cap: Option<f64>,
    #[serde(rename = "ars_24h_vol")]
    pub ars24_h_vol: Option<f64>,
    #[serde(rename = "ars_24h_change")]
    pub ars24_h_change: Option<f64>,
    pub aud: Option<f64>,
    pub aud_market_cap: Option<f64>,
    #[serde(rename = "aud_24h_vol")]
    pub aud24_h_vol: Option<f64>,
    #[serde(rename = "aud_24h_change")]
    pub aud24_h_change: Option<f64>,
    pub bdt: Option<f64>,
    pub bdt_market_cap: Option<f64>,
    #[serde(rename = "bdt_24h_vol")]
    pub bdt24_h_vol: Option<f64>,
    #[serde(rename = "bdt_24h_change")]
    pub bdt24_h_change: Option<f64>,
    pub bhd: Option<f64>,
    pub bhd_market_cap: Option<f64>,
    #[serde(rename = "bhd_24h_vol")]
    pub bhd24_h_vol: Option<f64>,
    #[serde(rename = "bhd_24h_change")]
    pub bhd24_h_change: Option<f64>,
    pub bmd: Option<f64>,
    pub bmd_market_cap: Option<f64>,
    #[serde(rename = "bmd_24h_vol")]
    pub bmd24_h_vol: Option<f64>,
    #[serde(rename = "bmd_24h_change")]
    pub bmd24_h_change: Option<f64>,
    pub brl: Option<f64>,
    pub brl_market_cap: Option<f64>,
    #[serde(rename = "brl_24h_vol")]
    pub brl24_h_vol: Option<f64>,
    #[serde(rename = "brl_24h_change")]
    pub brl24_h_change: Option<f64>,
    pub cad: Option<f64>,
    pub cad_market_cap: Option<f64>,
    #[serde(rename = "cad_24h_vol")]
    pub cad24_h_vol: Option<f64>,
    #[serde(rename = "cad_24h_change")]
    pub cad24_h_change: Option<f64>,
    pub chf: Option<f64>,
    pub chf_market_cap: Option<f64>,
    #[serde(rename = "chf_24h_vol")]
    pub chf24_h_vol: Option<f64>,
    #[serde(rename = "chf_24h_change")]
    pub chf24_h_change: Option<f64>,
    pub clp: Option<f64>,
    pub clp_market_cap: Option<f64>,
    #[serde(rename = "clp_24h_vol")]
    pub clp24_h_vol: Option<f64>,
    #[serde(rename = "clp_24h_change")]
    pub clp24_h_change: Option<f64>,
    pub cny: Option<f64>,
    pub cny_market_cap: Option<f64>,
    #[serde(rename = "cny_24h_vol")]
    pub cny24_h_vol: Option<f64>,
    #[serde(rename = "cny_24h_change")]
    pub cny24_h_change: Option<f64>,
    pub czk: Option<f64>,
    pub czk_market_cap: Option<f64>,
    #[serde(rename = "czk_24h_vol")]
    pub czk24_h_vol: Option<f64>,
    #[serde(rename = "czk_24h_change")]
    pub czk24_h_change: Option<f64>,
    pub dkk: Option<f64>,
    pub dkk_market_cap: Option<f64>,
    #[serde(rename = "dkk_24h_vol")]
    pub dkk24_h_vol: Option<f64>,
    #[serde(rename = "dkk_24h_change")]
    pub dkk24_h_change: Option<f64>,
    pub eur: Option<f64>,
    pub eur_market_cap: Option<f64>,
    #[serde(rename = "eur_24h_vol")]
    pub eur24_h_vol: Option<f64>,
    #[serde(rename = "eur_24h_change")]
    pub eur24_h_change: Option<f64>,
    pub gbp: Option<f64>,
    pub gbp_market_cap: Option<f64>,
    #[serde(rename = "gbp_24h_vol")]
    pub gbp24_h_vol: Option<f64>,
    #[serde(rename = "gbp_24h_change")]
    pub gbp24_h_change: Option<f64>,
    pub hkd: Option<f64>,
    pub hkd_market_cap: Option<f64>,
    #[serde(rename = "hkd_24h_vol")]
    pub hkd24_h_vol: Option<f64>,
    #[serde(rename = "hkd_24h_change")]
    pub hkd24_h_change: Option<f64>,
    pub huf: Option<f64>,
    pub huf_market_cap: Option<f64>,
    #[serde(rename = "huf_24h_vol")]
    pub huf24_h_vol: Option<f64>,
    #[serde(rename = "huf_24h_change")]
    pub huf24_h_change: Option<f64>,
    pub idr: Option<f64>,
    pub idr_market_cap: Option<f64>,
    #[serde(rename = "idr_24h_vol")]
    pub idr24_h_vol: Option<f64>,
    #[serde(rename = "idr_24h_change")]
    pub idr24_h_change: Option<f64>,
    pub ils: Option<f64>,
    pub ils_market_cap: Option<f64>,
    #[serde(rename = "ils_24h_vol")]
    pub ils24_h_vol: Option<f64>,
    #[serde(rename = "ils_24h_change")]
    pub ils24_h_change: Option<f64>,
    pub inr: Option<f64>,
    pub inr_market_cap: Option<f64>,
    #[serde(rename = "inr_24h_vol")]
    pub inr24_h_vol: Option<f64>,
    #[serde(rename = "inr_24h_change")]
    pub inr24_h_change: Option<f64>,
    pub jpy: Option<f64>,
    pub jpy_market_cap: Option<f64>,
    #[serde(rename = "jpy_24h_vol")]
    pub jpy24_h_vol: Option<f64>,
    #[serde(rename = "jpy_24h_change")]
    pub jpy24_h_change: Option<f64>,
    pub krw: Option<f64>,
    pub krw_market_cap: Option<f64>,
    #[serde(rename = "krw_24h_vol")]
    pub krw24_h_vol: Option<f64>,
    #[serde(rename = "krw_24h_change")]
    pub krw24_h_change: Option<f64>,
    pub kwd: Option<f64>,
    pub kwd_market_cap: Option<f64>,
    #[serde(rename = "kwd_24h_vol")]
    pub kwd24_h_vol: Option<f64>,
    #[serde(rename = "kwd_24h_change")]
    pub kwd24_h_change: Option<f64>,
    pub lkr: Option<f64>,
    pub lkr_market_cap: Option<f64>,
    #[serde(rename = "lkr_24h_vol")]
    pub lkr24_h_vol: Option<f64>,
    #[serde(rename = "lkr_24h_change")]
    pub lkr24_h_change: Option<f64>,
    pub mmk: Option<f64>,
    pub mmk_market_cap: Option<f64>,
    #[serde(rename = "mmk_24h_vol")]
    pub mmk24_h_vol: Option<f64>,
    #[serde(rename = "mmk_24h_change")]
    pub mmk24_h_change: Option<f64>,
    pub mxn: Option<f64>,
    pub mxn_market_cap: Option<f64>,
    #[serde(rename = "mxn_24h_vol")]
    pub mxn24_h_vol: Option<f64>,
    #[serde(rename = "mxn_24h_change")]
    pub mxn24_h_change: Option<f64>,
    pub myr: Option<f64>,
    pub myr_market_cap: Option<f64>,
    #[serde(rename = "myr_24h_vol")]
    pub myr24_h_vol: Option<f64>,
    #[serde(rename = "myr_24h_change")]
    pub myr24_h_change: Option<f64>,
    pub ngn: Option<f64>,
    pub ngn_market_cap: Option<f64>,
    #[serde(rename = "ngn_24h_vol")]
    pub ngn24_h_vol: Option<f64>,
    #[serde(rename = "ngn_24h_change")]
    pub ngn24_h_change: Option<f64>,
    pub nok: Option<f64>,
    pub nok_market_cap: Option<f64>,
    #[serde(rename = "nok_24h_vol")]
    pub nok24_h_vol: Option<f64>,
    #[serde(rename = "nok_24h_change")]
    pub nok24_h_change: Option<f64>,
    pub nzd: Option<f64>,
    pub nzd_market_cap: Option<f64>,
    #[serde(rename = "nzd_24h_vol")]
    pub nzd24_h_vol: Option<f64>,
    #[serde(rename = "nzd_24h_change")]
    pub nzd24_h_change: Option<f64>,
    pub php: Option<f64>,
    pub php_market_cap: Option<f64>,
    #[serde(rename = "php_24h_vol")]
    pub php24_h_vol: Option<f64>,
    #[serde(rename = "php_24h_change")]
    pub php24_h_change: Option<f64>,
    pub pkr: Option<f64>,
    pub pkr_market_cap: Option<f64>,
    #[serde(rename = "pkr_24h_vol")]
    pub pkr24_h_vol: Option<f64>,
    #[serde(rename = "pkr_24h_change")]
    pub pkr24_h_change: Option<f64>,
    pub pln: Option<f64>,
    pub pln_market_cap: Option<f64>,
    #[serde(rename = "pln_24h_vol")]
    pub pln24_h_vol: Option<f64>,
    #[serde(rename = "pln_24h_change")]
    pub pln24_h_change: Option<f64>,
    pub rub: Option<f64>,
    pub rub_market_cap: Option<f64>,
    #[serde(rename = "rub_24h_vol")]
    pub rub24_h_vol: Option<f64>,
    #[serde(rename = "rub_24h_change")]
    pub rub24_h_change: Option<f64>,
    pub sar: Option<f64>,
    pub sar_market_cap: Option<f64>,
    #[serde(rename = "sar_24h_vol")]
    pub sar24_h_vol: Option<f64>,
    #[serde(rename = "sar_24h_change")]
    pub sar24_h_change: Option<f64>,
    pub sek: Option<f64>,
    pub sek_market_cap: Option<f64>,
    #[serde(rename = "sek_24h_vol")]
    pub sek24_h_vol: Option<f64>,
    #[serde(rename = "sek_24h_change")]
    pub sek24_h_change: Option<f64>,
    pub sgd: Option<f64>,
    pub sgd_market_cap: Option<f64>,
    #[serde(rename = "sgd_24h_vol")]
    pub sgd24_h_vol: Option<f64>,
    #[serde(rename = "sgd_24h_change")]
    pub sgd24_h_change: Option<f64>,
    pub thb: Option<f64>,
    pub thb_market_cap: Option<f64>,
    #[serde(rename = "thb_24h_vol")]
    pub thb24_h_vol: Option<f64>,
    #[serde(rename = "thb_24h_change")]
    pub thb24_h_change: Option<f64>,
    #[serde(rename = "try")]
    pub try_field: Option<f64>,
    pub try_market_cap: Option<f64>,
    #[serde(rename = "try_24h_vol")]
    pub try24_h_vol: Option<f64>,
    #[serde(rename = "try_24h_change")]
    pub try24_h_change: Option<f64>,
    pub twd: Option<f64>,
    pub twd_market_cap: Option<f64>,
    #[serde(rename = "twd_24h_vol")]
    pub twd24_h_vol: Option<f64>,
    #[serde(rename = "twd_24h_change")]
    pub twd24_h_change: Option<f64>,
    pub uah: Option<f64>,
    pub uah_market_cap: Option<f64>,
    #[serde(rename = "uah_24h_vol")]
    pub uah24_h_vol: Option<f64>,
    #[serde(rename = "uah_24h_change")]
    pub uah24_h_change: Option<f64>,
    pub vef: Option<f64>,
    pub vef_market_cap: Option<f64>,
    #[serde(rename = "vef_24h_vol")]
    pub vef24_h_vol: Option<f64>,
    #[serde(rename = "vef_24h_change")]
    pub vef24_h_change: Option<f64>,
    pub vnd: Option<f64>,
    pub vnd_market_cap: Option<f64>,
    #[serde(rename = "vnd_24h_vol")]
    pub vnd24_h_vol: Option<f64>,
    #[serde(rename = "vnd_24h_change")]
    pub vnd24_h_change: Option<f64>,
    pub zar: Option<f64>,
    pub zar_market_cap: Option<f64>,
    #[serde(rename = "zar_24h_vol")]
    pub zar24_h_vol: Option<f64>,
    #[serde(rename = "zar_24h_change")]
    pub zar24_h_change: Option<f64>,
    pub xdr: Option<f64>,
    pub xdr_market_cap: Option<f64>,
    #[serde(rename = "xdr_24h_vol")]
    pub xdr24_h_vol: Option<f64>,
    #[serde(rename = "xdr_24h_change")]
    pub xdr24_h_change: Option<f64>,
    pub xag: Option<f64>,
    pub xag_market_cap: Option<f64>,
    #[serde(rename = "xag_24h_vol")]
    pub xag24_h_vol: Option<f64>,
    #[serde(rename = "xag_24h_change")]
    pub xag24_h_change: Option<f64>,
    pub xau: Option<f64>,
    pub xau_market_cap: Option<f64>,
    #[serde(rename = "xau_24h_vol")]
    pub xau24_h_vol: Option<f64>,
    #[serde(rename = "xau_24h_change")]
    pub xau24_h_change: Option<f64>,
    pub bits: Option<f64>,
    pub bits_market_cap: Option<f64>,
    #[serde(rename = "bits_24h_vol")]
    pub bits24_h_vol: Option<f64>,
    #[serde(rename = "bits_24h_change")]
    pub bits24_h_change: Option<f64>,
    pub sats: Option<f64>,
    pub sats_market_cap: Option<f64>,
    #[serde(rename = "sats_24h_vol")]
    pub sats24_h_vol: Option<f64>,
    #[serde(rename = "sats_24h_change")]
    pub sats24_h_change: Option<f64>,
    pub last_updated_at: Option<u64>,
}

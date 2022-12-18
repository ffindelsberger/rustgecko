use std::fmt;
use std::fmt::Formatter;

pub enum TrustOrder {
    TrustScoreDesc,
    TrustScoreAsc,
    VolumeDesc,
}

impl fmt::Display for TrustOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TrustOrder::TrustScoreDesc => write!(f, "trust_score_desc"),
            TrustOrder::TrustScoreAsc => write!(f, "trust_score_asc"),
            TrustOrder::VolumeDesc => write!(f, "volume_desc"),
        }
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

impl fmt::Display for MarketOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MarketOrder::MarketCapDesc => write!(f, "market_cap_desc"),
            MarketOrder::MarketCapAsc => write!(f, "market_cap_asc"),
            MarketOrder::GeckoDesc => write!(f, "gecko_desc"),
            MarketOrder::GeckoAsc => write!(f, "gecko_asc"),
            MarketOrder::VolumeAsc => write!(f, "volume_asc"),
            MarketOrder::VolumeDesc => write!(f, "volume_desc"),
            MarketOrder::IdAsc => write!(f, "id_asc"),
            MarketOrder::IdDesc => write!(f, "id_desc"),
        }
    }
}

/*impl Into<String> for MarketOrder {
    fn into(self) -> String {
        match self {
            MarketOrder::MarketCapDesc => "market_cap_desc".into(),
            MarketOrder::MarketCapAsc => "market_cap_asc".into(),
            MarketOrder::GeckoDesc => "gecko_desc".into(),
            MarketOrder::GeckoAsc => "gecko_asc".into(),
            MarketOrder::VolumeAsc => "volume_asc".into(),
            MarketOrder::VolumeDesc => "volume_desc".into(),
            MarketOrder::IdAsc => "id_asc".into(),
            MarketOrder::IdDesc => "id_desc".into(),
        }
    }
}*/

/*impl From<String> for MarketOrder {
    fn from(marketorder: String) -> Self {
        match marketorder {
            "market_cap_desc" => Self::MarketCapDesc,
            "market_cap_asc" => Self::MarketCapAsc,
            "gecko_desc" => Self::GeckoDesc,
            "gecko_asc" => Self::GeckoAsc,
            "volume_asc" => Self::VolumeAsc,
            "volume_desc" => Self::VolumeDesc,
            "id_asc" => Self::IdAsc,
            "id_desc" => Self::IdDesc,
        }
    }
}
*/
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

pub enum CompaniesCoinId {
    Bitcoin,
    Ethereum,
}

impl fmt::Display for CompaniesCoinId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CompaniesCoinId::Bitcoin => write!(f, "bitcoin"),
            CompaniesCoinId::Ethereum => write!(f, "etherum"),
        }
    }
}

pub enum DerivativesIncludeTickers {
    All,
    Unexpired,
}

impl fmt::Display for DerivativesIncludeTickers {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DerivativesIncludeTickers::All => write!(f, "all"),
            DerivativesIncludeTickers::Unexpired => write!(f, "unexpired"),
        }
    }
}

pub enum PriceChangePercentage {
    OneHour,
    TwentyFourHours,
    SevenDays,
    FourteenDays,
    ThirtyDays,
    TwoHundredDays,
    OneYear,
}

impl fmt::Display for PriceChangePercentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub enum TickerOrder {
    TrustScoreAsc,
    TrustScoreDesc,
    VolumeDesc,
}

impl fmt::Display for TickerOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TickerOrder::TrustScoreAsc => write!(f, "trust_score_asc"),
            TickerOrder::TrustScoreDesc => write!(f, "trust_score_desc"),
            TickerOrder::VolumeDesc => write!(f, "volume_desc"),
        }
    }
}

pub enum OhlcDays {
    OneDay,
    SevenDays,
    FourteenDays,
    ThirtyDays,
    NinetyDays,
    OneHundredEightyDays,
    ThreeHundredSixtyFiveDays,
}

impl fmt::Display for OhlcDays {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

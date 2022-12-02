pub enum TrustOrder {
    TrustScoreDesc,
    TrustScoreAsc,
    VolumeDesc,
}

impl TrustOrder {
    pub fn get_string(&self) -> &str {
        match self {
            TrustOrder::TrustScoreDesc => "trust_score_desc",
            TrustOrder::TrustScoreAsc => "trust_score_asc",
            TrustOrder::VolumeDesc => "volume_desc",
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

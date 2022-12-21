use log::LevelFilter;
use rustgecko::client::GeckoClient;
use rustgecko::model::queryparams::{MarketOrder, PriceChange};

#[tokio::main]
async fn main() {
    let _ = env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .try_init();
    let client = GeckoClient::default();
    let _ = client
        .coins_markets(
            "usd",
            None,
            MarketOrder::MarketCapDesc,
            Some(&[PriceChange::Days7, PriceChange::Years1, PriceChange::Days30]),
            true,
            Some(1),
        )
        .await
        .unwrap();
}

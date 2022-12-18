//TODO : Remove
#![allow(dead_code)]
#![allow(unused_variables)]
extern crate core;

pub mod client;
pub mod model;

#[cfg(test)]
mod test {
    use crate::client::GeckoClient;
    use crate::model::queryparams::{MarketOrder, PriceChange};
    use serial_test::serial;
    use std::thread;
    use std::time::Duration;
    use time::macros::date;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test]
    #[serial]
    async fn coins_list() {
        init();
        let client = GeckoClient::default();
        let _ = client.coins_list().await.unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn coins_id() {
        init();
        let client = GeckoClient::default();
        let list = client.coins_list().await.unwrap();

        for i in 1..list.len() {
            thread::sleep(Duration::from_secs(5));
            let id = &list.get(i).unwrap().id;
            if let Err(error) = client.coins(id, true, true, true, true, true, true).await {
                println!("{}", error);
                break;
            }
        }
    }

    #[tokio::test]
    #[serial]
    async fn coins_market() {
        init();
        let client = GeckoClient::default();
        let bitcoin = client.coins_short("bitcoin").await.unwrap();
        let price_changes = [PriceChange::Days7, PriceChange::Years1, PriceChange::Days30];

        let mut response;
        let mut page = 0;
        loop {
            response = client
                .coins_markets(
                    "usd",
                    None,
                    MarketOrder::MarketCapDesc,
                    Some(&price_changes),
                    true,
                    Some(page),
                )
                .await
                .unwrap();

            if response.is_empty() {
                break;
            };
            page += 1;
            thread::sleep(Duration::from_secs(5));
        }
    }

    #[tokio::test]
    #[serial]
    async fn coins_history() {
        init();
        let client = GeckoClient::default();
        let res = client
            .coins_history("bitcoin", date!(2022 - 10 - 1), None)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn coins_market_chart() {
        init();
        let client = GeckoClient::default();
        let _ = client
            .coins_marketchart("bitcoin", "usd", "max", None)
            .await
            .unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn asset_platforms() {
        init();
        let client = GeckoClient::default();
        let _ = client.assetplatforms(None).await;
    }

    #[tokio::test]
    #[serial]
    async fn exchange_rates() {
        init();
        let client = GeckoClient::default();
        let _ = client.exchangerates().await.unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn global() {
        init();
        let client = GeckoClient::default();
        let _ = client.global().await.unwrap();
    }
}

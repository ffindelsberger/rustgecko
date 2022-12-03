use coingecko_watch::client::GeckoClient;
use time::macros::date;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let client = GeckoClient::default();

    let response = client.coins_list().await;

    if let Err(error) = response {
        if error.is_status() {
            info!("We rate limited : Code : {}", error.status().unwrap());
        }
    } else {
        let response = response.unwrap();

        let first_listing = response
            .iter()
            .find(|&listing| listing.symbol == "btc")
            .unwrap();

        info!("{}", first_listing.id);

        let _ = client.coins_history(&first_listing.id, date!(2022 - 1 - 1), None);

        let res = client
            .coins_marketchart(&first_listing.id, "eur", "5", None)
            .await
            .unwrap();

        info!("{:#?}", res);
    }
}

async fn _example_error_handling() {
    let client = GeckoClient::default();

    let response = client.coins_list().await;

    if let Err(error) = response {
        if error.is_status() {
            info!("We rate limited : Code : {}", error.status().unwrap());
        }
    } else {
        let _response = response.unwrap();
        //Do some stuff with the success response
    }
}

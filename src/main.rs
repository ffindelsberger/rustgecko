#[tokio::main]
async fn main() {
    let client = rustgecko::client::GeckoClient::default();

    let x = client.coins_list().await.unwrap().len();
    println!("{x}");
}

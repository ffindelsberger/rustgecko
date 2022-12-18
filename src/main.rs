use rustgecko::client::GeckoClient;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
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

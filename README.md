![coingecko](https://github.com/ffindelsberger/rustgecko/actions/workflows/rust.yml/badge.svg)

# CoinGecko API Client for Rust

Simple API Client for CoinGecko written in Rust

## Available endpoint

[Refer to CoinGecko official API](https://www.coingecko.com/api)

|            Endpoint             | Status  | Testing |             Function             |
|:-------------------------------:|:-------:|:-------:|:--------------------------------:|
|              /ping              | &check; |         |               ping               |
|          /simple/price          | &check; |         | simple_price_short, simple_price |
| /simple/supported_vs_currencies | &check; | &check; |   SimpleSupportedVSCurrencies    |
|           /coins/list           | &check; | &check; |            CoinsList             |
|          /coins/market          | &check; | &check; |           CoinsMarket            |
|           /coins/{id}           | &check; | &check; |             CoinsID              |
|       /coins/{id}/history       | &check; | &check; |          CoinsIDHistory          |
|    /coins/{id}/market_chart     | &check; | &check; |        CoinsIDMarketChart        |
|        /events/countries        |   WIP   |   WIP   |         EventsCountries          |
|          /events/types          |   WIP   |   WIP   |            EventsType            |
|         /exchange_rates         | &check; | &check; |           ExchangeRate           |
|             /global             | &check; | &check; |              Global              |

More api Endpoints than listed here will be supported in the Future. As soon as I start working on additional Endpoints
the Table will be updated.

## Shortcut Methods

Some Methods with a lot of boolean Flags have a shorter Version i.E "simple_price_short" for if you just want to
retrieve Some Data and leave the Rest of the Params as their Default.

## Usage

```rust
use rustgecko::client::GeckoClient;

fn main() {
    let client = GeckoClient::default();
}
```

In a Production Setting or when you have a Coingecko Subscription you might want to supply your own Client with
Credentials or with any other additional configuration.

```rust
use rustgecko::client::GeckoClient;

fn main() {
    use reqwest::header;
    let mut headers = header::HeaderMap::new();

    // Consider marking security-sensitive headers with `set_sensitive`.
    let mut auth_value = header::HeaderValue::from_static("secret");
    auth_value.set_sensitive(true);
    headers.insert("x-cg-pro-api-key", auth_value);

    // get a client builder
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let _ = GeckoClient::new_with_custom_client(client, "https://some.url");
}
```

## Error Handling

Every 4XX Response is turned into an error of type reqwest::Error and propagated up the call chain.  
For handling reqwest errors reference the Docs -> https://docs.rs/reqwest/0.7.2/reqwest/struct.Error.html

```rust
async fn main() {
    match client.coins_list().await {
        Ok(response) => {
            println!("We got a response :)");
        }
        Err(err) => {
            println!("Oh no we got a 4XX Response or some other kind of reqwest error :(");
        }
    }
}

```

## License

MIT

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.

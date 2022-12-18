# CoinGecko API Client for Rust

Simple API Client for CoinGecko written in Rust

## Available endpoint

[Refer to CoinGecko official API](https://www.coingecko.com/api)

|            Endpoint             | Status  | Testing |            Function            |
|:-------------------------------:|:-------:|:-------:|:------------------------------:|
|              /ping              | &check; | &check; |              Ping              |
|          /simple/price          | &check; | &check; | SimpleSinglePrice, SimplePrice |
| /simple/supported_vs_currencies | &check; | &check; |  SimpleSupportedVSCurrencies   |
|           /coins/list           | &check; | &check; |           CoinsList            |
|          /coins/market          | &check; | &check; |          CoinsMarket           |
|           /coins/{id}           | &check; | &check; |            CoinsID             |
|       /coins/{id}/history       | &check; | &check; |         CoinsIDHistory         |
|    /coins/{id}/market_chart     | &check; | &check; |       CoinsIDMarketChart       |
|        /events/countries        | &check; | &check; |        EventsCountries         |
|          /events/types          | &check; | &check; |           EventsType           |
|         /exchange_rates         | &check; | &check; |          ExchangeRate          |
|             /global             | &check; | &check; |             Global             |

## Usage

```rust
use rustgecko::client::GeckoClient;

fn main() {
    let client = CoinGecko::default();
}
```

## License

MIT

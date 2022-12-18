# CoinGecko API Client for Rust


Simple API Client for CoinGecko written in Rust 

## Available endpoint

[Refer to CoinGecko official API](https://www.coingecko.com/api)

|            Endpoint             | Status | Testing |            Function            |
| :-----------------------------: | :----: | :-----: | :----------------------------: |
|              /ping              |  [/]   |   [/]   |              Ping              |
|          /simple/price          |  [/]   |   [/]   | SimpleSinglePrice, SimplePrice |
| /simple/supported_vs_currencies |  [/]   |   [/]   |  SimpleSupportedVSCurrencies   |
|           /coins/list           |  [/]   |   [/]   |           CoinsList            |
|          /coins/market          |  [/]   |   [/]   |          CoinsMarket           |
|           /coins/{id}           |  [/]   |         |            CoinsID             |
|       /coins/{id}/history       |  [/]   |         |         CoinsIDHistory         |
|    /coins/{id}/market_chart     |  [/]   |         |       CoinsIDMarketChart       |
|        /events/countries        |  [/]   |         |        EventsCountries         |
|          /events/types          |  [/]   |         |           EventsType           |
|         /exchange_rates         |  [/]   |         |          ExchangeRate          |
|             /global             |  [/]   |         |             Global             |

## Usage

```rust
use rustgecko::client::GeckoClient;

fn main() {
    let client = CoinGecko::default();
}
```

## License

MIT

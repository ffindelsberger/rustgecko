use std::collections::HashMap;

use log::debug;
use reqwest::header;
pub use reqwest::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;
use time::format_description::FormatItem;
use time::macros::format_description;
use time::Date;

use crate::model::apimodels::*;
use crate::model::coins::CoinsItem;
use crate::model::common::{Ping, Price};
use crate::model::exchangerates::ExchangeRates;
use crate::model::global::GlobalData;
use crate::model::queryparams::*;
use crate::model::simple::{CoinListing, ContractAddress};

pub const COINGECKO_DATE_FORMAT: &[FormatItem<'_>] = format_description!("[day]-[month]-[year]");

pub struct GeckoClient {
    client: reqwest::Client,
    api_url: String,
}

impl Default for GeckoClient {
    /// Creates a new CoinGeckoClient client with the default Congecko Api Url
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgecko::client::GeckoClient;
    /// let client = GeckoClient::default();
    /// ```
    fn default() -> Self {
        GeckoClient::new("https://api.coingecko.com/api/v3")
    }
}

impl GeckoClient {
    /// Creates a new CoinGeckoClient client with a custom host url.
    /// The url should be supplied without a trailing slash.
    /// # Examples
    ///
    /// ```rust
    /// use rustgecko::client::GeckoClient;
    /// let client = GeckoClient::new("https://some.url");
    /// ```
    pub fn new(api_url: impl Into<String>) -> GeckoClient {
        let cl = {
            let mut headers = header::HeaderMap::new();
            headers.insert(
                "Accept-Encoding",
                header::HeaderValue::from_static("deflate, gzip"),
            );

            reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .expect("Error when building Coingecko Api Client")
        };

        let mut api_url = api_url.into();

        if api_url.ends_with("/") {
            api_url
                .pop()
                .expect("Tried to remove the trailing slash of the url but panicked");
        }

        GeckoClient {
            client: cl,
            api_url,
        }
    }

    /// Creates a new GeckoClient with the supplied httpclient,
    /// use this if you need to specify an Api Access Key in the Requests.
    /// Set the Key as a default header in the Client
    /// usefull also if you want to specify a custom retry or timeout behavior
    ///
    /// String or &str is the real question
    /// # Examples
    /// ```rust
    ///    use reqwest::header;
    ///    use rustgecko::client::GeckoClient;
    ///    let mut headers = header::HeaderMap::new();
    ///
    ///    // Consider marking security-sensitive headers with `set_sensitive`.
    ///    let mut auth_value = header::HeaderValue::from_static("secret");
    ///    auth_value.set_sensitive(true);
    ///    headers.insert("x-cg-pro-api-key", auth_value);
    ///
    ///    // get a client builder
    ///    let client = reqwest::Client::builder()
    ///        .default_headers(headers)
    ///        .build()
    ///        .unwrap();
    ///
    ///    let _ = GeckoClient::new_with_custom_client(client, "https://some.url");
    /// ```
    pub fn new_with_custom_client(
        client: reqwest::Client,
        api_url: impl Into<String>,
    ) -> GeckoClient {
        GeckoClient {
            client,
            api_url: api_url.into(),
        }
    }

    async fn send_gecko_request<T: Serialize + ?Sized, D: DeserializeOwned>(
        &self,
        endpoint: &str,
        query_params: Option<&T>,
    ) -> Result<D, reqwest::Error> {
        let url = format!("{}{}", self.api_url, endpoint);

        let mut req_builder = self.client.get(url);

        if let Some(params) = query_params {
            req_builder = req_builder.query(params);
        }

        let request = req_builder.build()?;
        debug!("Calling CoinGecko API with url: {}", request.url());
        let response = self.client.execute(request).await?;

        //Handle 4XX Status Codes
        if let Err(error) = response.error_for_status_ref() {
            debug!("{}", response.text().await?);
            return Err(error);
        };

        let content = response.text().await?;
        //info!("Response as text : {}", &content);
        let jd = &mut serde_json::Deserializer::from_str(&content);

        let result: Result<D, _> = serde_path_to_error::deserialize(jd);

        match result {
            Ok(res) => Ok(res),
            Err(err) => {
                panic!("{}", err.to_string());
            }
        }
        //response.json().await
    }

    /// Check API server status
    pub async fn ping(&self) -> Result<Ping, reqwest::Error> {
        self.send_gecko_request("/ping", None::<&[()]>).await
    }

    /// Calls the simple/supported_vs_currencies endpoint
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgecko::client::GeckoClient;
    /// let client = GeckoClient::new("https://some.url");
    /// client.simple_supportedvscurrencies();
    /// ```
    pub async fn simple_supportedvscurrencies(&self) -> Result<Vec<String>, reqwest::Error> {
        self.send_gecko_request("/simple/supported_vs_currencies", None::<&[()]>)
            .await
    }

    /// Shortcut method for [GeckoClient::simple_price]. Sets all boolean flags to true.
    pub async fn simple_price_short(
        &self,
        ids: &[&str],
        vs_currencies: &[&str],
    ) -> Result<HashMap<String, Price>, reqwest::Error> {
        self.simple_price(ids, vs_currencies, true, true, true, true, "max")
            .await
    }

    /// Calls the simple/price endpoint
    /// The supported Currencies can be retrieved via the simple/supported_vs_currencies endpoint
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgecko::client::GeckoClient;
    /// let client = GeckoClient::default();
    /// client.simple_price(&["1032"], &["usd","eur"], true, true, true ,true, "max");
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn simple_price(
        &self,
        ids: &[&str],
        vs_currencies: &[&str],
        include_market_cap: bool,
        include_24hr_vol: bool,
        include_24hr_change: bool,
        include_last_updated_at: bool,
        precision: &str,
    ) -> Result<HashMap<String, Price>, reqwest::Error> {
        self.send_gecko_request(
            "/simple/price",
            Some(&[
                ("ids", ids.join("%2C")),
                ("vs_currencies", vs_currencies.join("%2C")),
                ("include_market_cap", include_market_cap.to_string()),
                ("include_24hr_change", include_24hr_change.to_string()),
                ("include_24hr_vol", include_24hr_vol.to_string()),
                (
                    "include_last_updated_at",
                    include_last_updated_at.to_string(),
                ),
                ("precision", precision.to_string()),
            ]),
        )
        .await
    }
    /// Calls the simple/price/{id} endpoint
    /// The supported Currencies can be retrieved via the simple/supported_vs_currencies endpoint
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgecko::client::GeckoClient;
    /// let client = GeckoClient::default();
    /// client.simple_price(&["1032"], &["usd","eur"], true, true, true ,true, "max");
    /// ```
    #[allow(clippy::too_many_arguments)]
    async fn simple_price_id(
        &self,
        ids: &[&str],
        vs_currencies: &[&str],
        contract_addresses: &[&str],
        include_market_cap: bool,
        include_24hr_vol: bool,
        include_24hr_change: bool,
        include_last_updated_at: bool,
        precision: &str,
    ) -> Result<HashMap<String, Price>, reqwest::Error> {
        self.send_gecko_request(
            "/simple/price",
            Some(&[
                ("ids", ids.join("%2C")),
                ("contract_addresses", contract_addresses.join("%2C")),
                ("vs_currencies", vs_currencies.join("%2C")),
                ("include_market_cap", include_market_cap.to_string()),
                ("include_24hr_change", include_24hr_change.to_string()),
                ("include_24hr_vol", include_24hr_vol.to_string()),
                (
                    "include_last_updated_at",
                    include_last_updated_at.to_string(),
                ),
                ("precision", precision.to_string()),
            ]),
        )
        .await
    }

    pub async fn simple_token_price_short(
        &self,
        id: &str,
        vs_currencies: &[&str],
        contract_addresses: &[&str],
    ) -> Result<HashMap<ContractAddress, Price>, reqwest::Error> {
        self.simple_token_price(
            id,
            vs_currencies,
            contract_addresses,
            true,
            true,
            true,
            true,
            "full",
        )
        .await
    }

    /// Calls the simple/token_price/{id} endpoint
    /// Get current price of tokens (using contract addresses) for a given platform in any other currency that you need.
    /// # Examples
    ///
    /// ```rust
    /// use rustgecko::client::GeckoClient;
    /// let client = GeckoClient::default();
    /// let res = client
    ///             .simple_token_price(
    ///                 "ethereum",
    ///                 &["usd", "eur"],
    ///                 &[
    ///                     "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984",
    ///                     "0xf629cbd94d3791c9250152bd8dfbdf380e2a3b9c",
    ///                 ],
    ///                 false,
    ///                 false,
    ///                 false,
    ///                 false,
    ///                 "full",
    ///             );
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn simple_token_price(
        &self,
        id: &str,
        vs_currencies: &[&str],
        contract_addresses: &[&str],
        include_market_cap: bool,
        include_24hr_vol: bool,
        include_24hr_change: bool,
        include_last_updated_at: bool,
        precision: &str,
    ) -> Result<HashMap<ContractAddress, Price>, reqwest::Error> {
        let url = format!("/simple/token_price/{}", id);
        self.send_gecko_request(
            &url,
            Some(&[
                ("vs_currencies", vs_currencies.join(",")),
                ("include_market_cap", include_market_cap.to_string()),
                ("include_24hr_change", include_24hr_change.to_string()),
                ("include_24hr_vol", include_24hr_vol.to_string()),
                (
                    "include_last_updated_at",
                    include_last_updated_at.to_string(),
                ),
                ("precision", precision.to_string()),
                ("contract_addresses", contract_addresses.join(",")),
            ]),
        )
        .await
    }

    ///Use this to obtain all the coins' id in order to make API calls
    pub async fn coins_list(&self) -> Result<Vec<CoinListing>, reqwest::Error> {
        self.send_gecko_request("/coins/list", Some(&[("include_platform", "true")]))
            .await
    }

    /// Use this to obtain all the coins market data (price, market cap, volume)
    ///
    /// # Arguments
    /// * `vs_currency` - The target currency of market data (usd, eur, jpy, etc.)
    /// * `ids` - The ids of the coin, comma separated crytocurrency symbols (base). refers to /coins/list.
    /// * `ordering` - sort results by field. Default value if None is passed: market_cap_desc
    /// * `price_change_percentage ` - Include price change percentage for the given TimeFrames
    /// * `sparkline `- Include sparkline 7 days data (eg. true, false)
    /// * `page `- Page through results. If None default value 1 is used
    pub async fn coins_markets(
        &self,
        vs_currency: &str,
        ids: Option<&[&str]>,
        ordering: MarketOrder,
        price_change_percentage: Option<&[PriceChange]>,
        sparkline: bool,
        page: Option<i64>,
    ) -> Result<Vec<CoinsMarketItem>, reqwest::Error> {
        let mut params: Vec<(&str, String)> = Vec::new();

        params.push(("vs_currency", vs_currency.into()));
        params.push(("order", ordering.to_string()));
        params.push(("sparkline", sparkline.to_string()));

        if let Some(ids) = ids {
            params.push(("ids", ids.join("%2C")))
        }
        if let Some(price_change) = price_change_percentage {
            let tmp = price_change
                .iter()
                .map(|ele| ele.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.push(("price_change", tmp));
        }
        if let Some(page) = page {
            params.push(("page", page.to_string()));
        }
        self.send_gecko_request("/coins/markets", Some(&params))
            .await
    }

    /// Shortcut Method that calls [GeckoClient::coins] with all Flags set to true for convenience
    pub async fn coins_short(&self, id: &str) -> Result<CoinsItem, reqwest::Error> {
        self.coins(id, true, true, true, true, true, true).await
    }

    /// Get current data (name, price, market, ... including exchange tickers) for a coin.
    ///
    /// # Important
    /// Ticker object is limited to 100 items, to get more tickers, use /coins/{id}/tickers
    /// Ticker is_stale is true when ticker that has not been updated/unchanged from the exchange for a while.
    /// Ticker is_anomaly is true if ticker's price is outliered by our system.
    /// You are responsible for managing how you want to display these information (e.g. footnote, different background, change opacity, hide)
    #[allow(clippy::too_many_arguments)]
    pub async fn coins(
        &self,
        id: &str,
        localization: bool,
        tickers: bool,
        market_data: bool,
        community_data: bool,
        developer_data: bool,
        sparkline: bool,
    ) -> Result<CoinsItem, reqwest::Error> {
        let url = format!("/coins/{}", id);
        let params = [
            ("localization", localization),
            ("tickers", tickers),
            ("market_data", market_data),
            ("community_data", community_data),
            ("developer_data", developer_data),
            ("sparkline", sparkline),
        ];
        self.send_gecko_request(&url, Some(&params)).await
    }

    async fn coins_tickers(
        &self,
        id: &str,
        exchange_ids: Option<&[&str]>,
        include_exchange_logo: bool,
        page: i64,
        order: TrustOrder,
    ) -> Result<Vec<Ticker>, reqwest::Error> {
        let url = &format!("/coins/{}/tickers", id);

        let mut params: Vec<(&str, String)> = vec![
            ("include_exchange_logo", include_exchange_logo.to_string()),
            ("page", page.to_string()),
            ("order", order.to_string()),
        ];

        if let Some(ids) = exchange_ids {
            params.push(("exchange_ids", ids.join("%2C")))
        }

        self.send_gecko_request(url, Some(&params)).await
    }

    ///Get historical data (name, price, market, stats) at a given date for a coin
    ///
    ///# Arguments
    /// * `id` - pass the coin id (can be obtained from /coins) eg. bitcoin
    /// * `date` - The date of data snapshot, the function takes care of the proper Date Formatting to dd-mm-yyyy eg. 30-12-2017
    /// * `localization` - Set to false to exclude localized languages in response
    pub async fn coins_history(
        &self,
        id: &str,
        date: Date,
        localization: Option<&str>,
    ) -> Result<CoinHistoryItem, reqwest::Error> {
        let date = date.format(COINGECKO_DATE_FORMAT).unwrap();
        let mut params: Vec<(&str, &str)> = vec![("date", &date)];

        if let Some(languages) = localization {
            params.push(("localization", languages));
        }

        self.send_gecko_request(&format!("/coins/{}/history", id), Some(&params))
            .await
    }

    ///Get historical market data include price, market cap, and 24h volume (granularity auto)
    ///
    /// # Arguments
    /// * `id` - pass the coin id (can be obtained from /coins) eg. bitcoin
    /// * `vs_currencies` - The target currency of market data (usd, eur, jpy, etc.)
    /// * `days` - Data up to number of days ago (eg. 1,14,30,max)
    /// * `interval` - Data interval. Possible value: daily
    pub(crate) async fn coins_marketchart(
        &self,
        id: &str,
        vs_currencies: &str,
        days: &str,
        interval: Option<&str>,
    ) -> Result<MarketChart, reqwest::Error> {
        let url = format!("/coins/{}/market_chart", id);
        let mut params = vec![("vs_currency", vs_currencies), ("days", days)];

        if let Some(interval) = interval {
            params.push(("interval", interval));
        }

        self.send_gecko_request(&url, Some(&params)).await
    }

    //TODO: test this endpoint
    async fn coins_marketchart_range(
        &self,
        id: impl Into<String>,
        vs_currency: impl Into<String>,
        from: impl Into<String>,
        to: impl Into<String>,
    ) -> Result<MarketChart, reqwest::Error> {
        let url = format!("/coins/{}/market_chart/range", id.into());
        let params = vec![
            ("vs_currency", vs_currency.into()),
            ("from", from.into()),
            ("to", to.into()),
        ];
        self.send_gecko_request(&url, Some(&params)).await
    }

    ///
    /// # Arguments
    /// * `id` - pass the coin id (can be obtained from /coins/list) eg. bitcoin
    /// * `vs_currency` - The target currency of market data (usd, eur, jpy, etc.)
    /// * `days` - Data up to number of days ago (1/7/14/30/90/180/365/max)
    ///
    async fn coins_ohlc(
        &self,
        id: &str,
        vs_currency: &str,
        days: &str,
    ) -> Result<Vec<Candle>, reqwest::Error> {
        let url = format!("/coins/{}/ohlc", id);
        let params = vec![("vs_currency", vs_currency), ("days", days)];

        self.send_gecko_request(&url, Some(&params)).await
    }

    /*   pub fn contract(&self, id: &str, contract_address: &str) -> Result<_, reqwest::Error> {
        let url = format!("/coins/{}/contract/{}", id, contract_address);

        self.send_gecko_request(url, None)
    }*/

    pub fn contract_marketchart() {
        todo!();
    }

    pub fn contract_marketchart_range() {
        todo!();
    }

    ///List all asset platforms
    ///
    /// # Arguments
    /// * `filter` - apply relevant filters to results. Valid values: "nft" (asset_platform nft-support)
    pub(crate) async fn assetplatforms(
        &self,
        filter: Option<&str>,
    ) -> Result<Vec<AssetPlatform>, reqwest::Error> {
        let mut params: Vec<(&str, &str)> = Vec::new();

        if let Some(value) = filter {
            params.push(("filter", value));
        }

        self.send_gecko_request("/asset_platforms", Some(&params))
            .await
    }
    fn categories_list() {
        todo!();
    }

    fn categories() {
        todo!();
    }

    fn all_exchanges() {
        todo!();
    }

    fn exchanges_list() {
        todo!();
    }

    fn exchanges_tickers() {
        todo!();
    }

    fn exchanges() {
        todo!();
    }

    fn exchanges_volumechart() {
        todo!();
    }

    fn all_indexes() {
        todo!();
    }

    fn indexes() {
        todo!();
    }

    fn indexes_list() {
        todo!();
    }

    fn derivatives() {
        todo!();
    }

    fn all_derivatives_exchanges() {
        todo!();
    }

    fn derivatives_exchanges_list() {
        todo!();
    }

    fn nfts_list() {
        todo!();
    }

    fn nfts() {
        todo!();
    }

    fn nfts_contract() {
        todo!();
    }

    /// Get BTC-to-Currency exchange rates
    pub async fn exchangerates(&self) -> Result<ExchangeRates, reqwest::Error> {
        let url = "/exchange_rates";

        self.send_gecko_request(url, None::<&[()]>).await
    }

    fn search() {
        todo!();
    }

    fn search_trending() {
        todo!();
    }

    ///Get cryptocurrency global data
    pub async fn global(&self) -> Result<GlobalData, reqwest::Error> {
        let url = "/global";
        self.send_gecko_request(url, None::<&[()]>).await
    }

    fn global_defi() {
        todo!();
    }

    pub fn beta_companies_public_treasury() {
        todo!();
    }
}

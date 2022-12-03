use crate::model::queryparams::*;
use crate::model::responses::*;
use reqwest::header;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use time::format_description::FormatItem;
use time::macros::format_description;
use time::Date;
use tracing::{debug, info};

pub const COINGECKO_DATE_FORMAT: &[FormatItem<'_>] = format_description!("[day]-[month]-[year]");

pub struct GeckoClient {
    client: reqwest::Client,
    api_location: &'static str,
}

impl Default for GeckoClient {
    fn default() -> Self {
        GeckoClient::new("https://api.coingecko.com/api/v3")
    }
}

impl GeckoClient {
    /// Creates a new CoinGeckoClient client with a custom host url
    ///
    /// # Examples
    ///
    /// ```rust
    /// use geckoClient::geckoClient;
    /// let client = geckoClient::new("https://some.url");
    /// ```
    fn new(api_url: &'static str) -> GeckoClient {
        let cl = {
            let mut headers = header::HeaderMap::new();
            headers.insert(
                "Accept-Encoding",
                header::HeaderValue::from_static("deflate, gzip"),
            );

            reqwest::Client::builder()
                .gzip(true)
                .default_headers(headers)
                .build()
                .expect("Error when building Coinmarketcap Api Client")
        };

        if api_url.ends_with("/") {}

        return GeckoClient {
            client: cl,
            api_location: api_url,
        };
    }

    /// Creates a new GeckoClient with the supplied httpclient,
    /// use this if you need to specify an Api Access Key in the Requests.
    /// Set the Key as a default header in the Client
    /// usefull also if you want to specify a custom retry or timeout behavior
    ///
    /// String or &str is the real question
    /// # Examples
    ///
    ///
    pub fn new_with_custome_client(
        http_client: reqwest::Client,
        api_url: &'static str,
    ) -> GeckoClient {
        return GeckoClient {
            client: http_client,
            api_location: api_url,
        };
    }

    async fn send_gecko_request<T: Serialize + ?Sized, D: DeserializeOwned>(
        &self,
        endpoint: &str,
        query_params: Option<&T>,
    ) -> Result<D, reqwest::Error> {
        let url = format!("{}{}", self.api_location, endpoint);

        //TODO: handle reqwest::Error like no connectino or some shit somewhere, this here is just temporary
        let mut req_builder = self.client.get(url);

        if let Some(params) = query_params {
            req_builder = req_builder.query(params);
        }
        let request = req_builder.build().expect("error building request");

        debug!("Calling CoinGecko API with url: {}", request.url());
        let response = self.client.execute(request).await?;

        if let Err(error) = response.error_for_status_ref() {
            //TODO: Change to debug
            info!("{}", response.text().await?);
            return Err(error);
        };

        return response.json().await;
    }

    /// Calls the simple/supported_vs_currencies endpoint
    ///
    /// # Examples
    ///
    /// ```rust
    /// let client = geckoClient::new("https://some.url");
    /// client.simple_supportedvscurrencies();
    /// ```
    pub async fn simple_supportedvscurrencies(&self) -> Result<Vec<String>, reqwest::Error> {
        return self
            .send_gecko_request("/simple/supported_vs_currencies", None::<&[()]>)
            .await;
    }

    /// Calls the simple/price endpoint
    /// The supported Currencies can be retrieved via the simple/supported_vs_currencies endpoint
    ///
    /// # Examples
    ///
    /// ```rust
    /// let client = geckoClient::default();
    /// client.simple_price(&["1032"], &["usd","eur"]);
    /// ```
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
        return self
            .send_gecko_request(
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
            .await;
    }

    pub async fn simple_tokenprice(
        &self,
        id: &str,
        vs_currencies: &[&str],
        contract_addresses: &[&str],
        include_market_cap: bool,
        include_24hr_vol: bool,
        include_24hr_change: bool,
        include_last_updated_at: bool,
        precision: &str,
    ) -> Result<CoinListing, reqwest::Error> {
        //TODO: Test endpoint and set correct return type, this does not give a coinlisting
        return self
            .send_gecko_request(
                "/simple/price",
                Some(&[
                    ("id", id.to_string()),
                    ("vs_currencies", vs_currencies.join("%2C")),
                    ("include_market_cap", include_market_cap.to_string()),
                    ("include_24hr_change", include_24hr_change.to_string()),
                    ("include_24hr_vol", include_24hr_vol.to_string()),
                    (
                        "include_last_updated_at",
                        include_last_updated_at.to_string(),
                    ),
                    ("precision", precision.to_string()),
                    ("contract_addresses", contract_addresses.join("%2C")),
                ]),
            )
            .await;
    }

    pub async fn coins_list(&self) -> Result<Vec<CoinListing>, reqwest::Error> {
        return self
            .send_gecko_request("/coins/list", Some(&[("include_platform", "true")]))
            //.execute_gecko_request("/coins/list", None::<&[()]>)
            .await;
    }

    ///
    /// Market order uses the default if nonde is given -> MarketCap
    pub async fn coins_markets(
        &self,
        vs_currency: &str,
        ids: Option<&[&str]>,
        ordering: MarketOrder,
        price_change: Option<&[PriceChange]>,
    ) -> Result<Vec<CoinsMarketItem>, reqwest::Error> {
        let mut params: Vec<(&str, String)> = Vec::new();

        params.push(("vs_currency", vs_currency.into()));
        params.push(("order", ordering.get_string().into()));

        if let Some(ids) = ids {
            params.push(("ids", ids.join("%2C")))
        }

        if let Some(price_change) = price_change {
            let tmp = price_change.iter().map(|ele| ele.get_string()).collect();
            params.push(("price_change", tmp));
        }

        return self
            .send_gecko_request("/coins/markets", Some(&params))
            .await;
    }

    pub fn coins(
        _id: &str,
        _localization: bool,
        _tickers: bool,
        _market_data: bool,
        _community_data: bool,
        _developer_data: bool,
        _sparkline: bool,
    ) {
        todo!();
    }

    pub async fn coins_tickers(
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
            ("order", order.get_string().to_string()),
        ];

        if let Some(ids) = exchange_ids {
            params.push(("exchange_ids", ids.join("%2C")))
        }

        return self.send_gecko_request(url, Some(&params)).await;
    }

    ///dd-mm-yyyy
    pub async fn coins_history(
        &self,
        id: &str,
        date: Date,
        localization: Option<&str>,
    ) -> Result<Vec<Ticker>, reqwest::Error> {
        let date = date.format(COINGECKO_DATE_FORMAT).unwrap();
        let mut params: Vec<(&str, String)> = vec![("date", date)];

        if let Some(languages) = localization {
            params.push(("localization", languages.into()));
        }

        return self
            .send_gecko_request(&format!("/coins/{}/history", id), Some(&params))
            .await;
    }

    pub async fn coins_marketchart(
        &self,
        id: &str,
        vs_currencies: &str,
        days: &str,
        interval: Option<&str>,
    ) -> Result<MarketChart, reqwest::Error> {
        let url = format!("/coins/{}/market_chart", id);
        let mut params: Vec<(&str, String)> = vec![
            ("vs_currency", vs_currencies.to_string()),
            ("days", days.to_string()),
        ];

        if let Some(interval) = interval {
            params.push(("interval", interval.to_string()));
        }

        return self.send_gecko_request(&url, Some(&params)).await;
    }

    pub fn coins_marketchart_range() {
        todo!();
    }

    pub fn coins_ohlc() {
        todo!();
    }

    pub fn contract() {
        todo!();
    }

    pub fn contract_marketchart() {
        todo!();
    }

    pub fn contract_marketchart_range() {
        todo!();
    }

    pub fn assetplatforms() {
        todo!();
    }

    pub fn categories_list() {
        todo!();
    }

    pub fn categories() {
        todo!();
    }

    pub fn all_exchanges() {
        todo!();
    }

    pub fn exchanges_list() {
        todo!();
    }

    pub fn exchanges_tickers() {
        todo!();
    }

    pub fn exchanges() {
        todo!();
    }

    pub fn exchanges_volumechart() {
        todo!();
    }

    pub fn all_indexes() {
        todo!();
    }

    pub fn indexes() {
        todo!();
    }

    pub fn indexes_list() {
        todo!();
    }

    pub fn derivatives() {
        todo!();
    }

    pub fn all_derivatives_exchanges() {
        todo!();
    }

    pub fn derivatives_exchanges_list() {
        todo!();
    }

    pub fn nfts_list() {
        todo!();
    }

    pub fn nfts() {
        todo!();
    }

    pub fn nfts_contract() {
        todo!();
    }

    pub fn exchangerates() {
        todo!();
    }

    pub fn search() {
        todo!();
    }

    pub fn search_trending() {
        todo!();
    }

    pub fn global() {
        todo!();
    }

    pub fn global_defi() {
        todo!();
    }

    pub fn beta_companies_public_treasury() {
        todo!();
    }
}

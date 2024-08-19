use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE};
use reqwest::{Error, Client};
use serde_json::Value;
use log::{info, debug};


pub struct StockPulseApi {
    pub host: String,
    pub api_key: String,
}
impl StockPulseApi {
    fn construct_headers(&self) -> HeaderMap {
        info!("Constructing StockPulseApi headers.");
        debug!("api_key value: {}", &self.api_key);
        let mut headers = HeaderMap::new();

        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            HeaderName::from_static("x-rapidapi-host"),
            HeaderValue::from_str(&self.host).unwrap());
        headers.insert(
            HeaderName::from_static("x-rapidapi-key"),
            HeaderValue::from_str(&self.api_key).unwrap());
        headers
    }
    pub async fn request_multi_quote(&self, stocks: &String) -> Result<Value, Error> {
        info!("Calling StockPulseApi's 'multi_quote' endpoint.");
        let url = format!("https://{}/multi-quote/{stocks}", &self.host, );
        debug!("Url: {}", &url);
        let client = Client::new();
        let response: Value = client.get(url)
            .headers(self.construct_headers())
            .send()
            .await?
            .json()
            .await?;
        debug!("Response body: {}", response);
        Ok(response)
    }
}

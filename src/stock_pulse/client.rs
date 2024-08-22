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
    pub async fn request_multi_quote(&self, stocks: &String, protocol: &String) -> Result<Value, Error> {
        info!("Calling StockPulseApi's 'multi_quote' endpoint.");
        let url = format!("{protocol}://{}/multi-quote/{stocks}", &self.host);
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


#[cfg(test)]
mod stock_pulse_api_tests {
    use super::*;
    use reqwest::header::{HeaderValue, CONTENT_TYPE};
    use serde_json::json;

    #[test]
    fn test_construct_headers() {
        let api = StockPulseApi {
            host: "example.com".to_string(),
            api_key: "test_api_key".to_string(),
        };

        let headers = api.construct_headers();

        // Verify CONTENT_TYPE header
        assert_eq!(
            headers.get(CONTENT_TYPE),
            Some(&HeaderValue::from_static("application/json"))
        );

        // Verify x-rapidapi-host header
        assert_eq!(
            headers.get("x-rapidapi-host"),
            Some(&HeaderValue::from_static("example.com"))
        );

        // Verify x-rapidapi-key header
        assert_eq!(
            headers.get("x-rapidapi-key"),
            Some(&HeaderValue::from_static("test_api_key"))
        );
    }

    #[tokio::test]
    async fn test_request_multi_quote() {
        let mut server = mockito::Server::new_async().await;
        let host_with_port = server.host_with_port().replace("https://", "");

        let _mock = server.mock("GET", "/multi-quote/AAPL")
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "symbol": "AAPL",
                    "price": 150.0
                }).to_string()
            )
            .create_async().await;

        let api = StockPulseApi {
            host: host_with_port,
            api_key: "test_api_key".to_string(),
        };

        let stocks = "AAPL".to_string();
        let protocol = "http".to_string();
        let response = api.request_multi_quote(&stocks, &protocol).await.unwrap();

        // Check the response body
        assert_eq!(response["symbol"], "AAPL");
        assert_eq!(response["price"], 150.0);
    }
}
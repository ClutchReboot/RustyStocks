use serde::Deserialize;


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StockPulseResponse {
    pub display_name: String,
    pub symbol: String,
    pub regular_market_previous_close: Format,
    pub regular_market_change_percent: Format,
}

#[derive(Deserialize, Debug)]
pub struct Format {
    pub fmt: String,
}


#[cfg(test)]
mod stock_pulse_response_tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_stock_pulse_response_deserialization() {
        // Example JSON string that matches the expected structure
        let json_data = r#"
        {
            "displayName": "Example Stock",
            "symbol": "EXMPL",
            "regularMarketPreviousClose": {
                "fmt": "150.00"
            },
            "regularMarketChangePercent": {
                "fmt": "1.23"
            }
        }
        "#;

        // Deserialize the JSON string into StockPulseResponse
        let response: StockPulseResponse = serde_json::from_str(json_data).unwrap();

        // Assert that the fields were deserialized correctly
        // Assert keys were deserialized from camel to snake case
        assert_eq!(response.display_name, "Example Stock");
        assert_eq!(response.symbol, "EXMPL");
        assert_eq!(response.regular_market_previous_close.fmt, "150.00");
        assert_eq!(response.regular_market_change_percent.fmt, "1.23");
    }
}

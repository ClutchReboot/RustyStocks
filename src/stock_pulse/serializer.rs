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

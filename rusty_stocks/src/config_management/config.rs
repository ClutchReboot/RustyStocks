use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct RustyStocksConfig {
    pub host: String,
    pub api_key: String,
    pub stocks: String,
}
impl Default for RustyStocksConfig {
    fn default() -> Self { Self {
        host: String::from("").into(),
        api_key: String::from("").into(),
        stocks: String::from("").into(),
    } }
}

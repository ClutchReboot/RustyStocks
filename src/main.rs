use rusty_stocks::stock_pulse::{StockPulseApi, StockPulseResponse};
use rusty_stocks::config_management::RustyStocksConfig;
use log::{debug, info};
use clap::Parser;
use serde_json::Value;

#[derive(Parser)]
pub struct Cli {
    #[arg(long, help="Hostname RapidAPI provided. Example: 'fake-finance123.p.rapidapi.com'")]
    host: Option<String>,
    #[arg(short, long, help="Provided from StockPulse at RapidAPI's marketplace here: https://rapidapi.com/manwilbahaa/api/yahoo-finance127")]
    api_key: Option<String>,
    #[arg(short, long, help="Include the NASDAQ symbols with comma delimiter. Example: 'tsla,aapl,msft'")]
    stocks: Option<String>,
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app_name = "rusty_stocks";
    let mut config: RustyStocksConfig = confy::load(app_name, None).unwrap_or_default();
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    info!("Updating config.");
    update_config(&app_name, args, &mut config);

    info!("Calling Stock Pulse.");
    debug!("Config {:?}", config);
    let result: Value = StockPulseApi{host: config.host, api_key: config.api_key}
        .request_multi_quote(&config.stocks, &"https".to_string()).await.unwrap();

    info!("Formating Stock Pulse's response.");
    format_stock_data(&result);
    Ok(())
}

fn update_config(app_name: &str, args: Cli, config: &mut RustyStocksConfig) {
    if args.host.is_some() && args.api_key.is_some() {
        debug!("No 'host' or 'api_key' found in arguments.");
        config.host = args.host.clone().unwrap();
        config.api_key = args.api_key.clone().unwrap();
        confy::store(app_name, None, &config).unwrap();
    }

    if args.stocks.is_some() {
        debug!("Updating config 'stocks' value.");
        config.stocks = args.stocks.clone().unwrap();
        confy::store(app_name, None, &config).unwrap();
    }
}

fn format_stock_data(json_response: &Value) {
    if let Value::Object(map) = json_response {
        debug!("Iterating through Stock Pulse's response.");
        for (_, value) in map.iter() {
            let entry: StockPulseResponse = serde_json::from_value(value.clone()).unwrap();
            println!(
                "{} ({}):\t{}\t{}",
                entry.display_name,
                entry.symbol,
                entry.regular_market_previous_close.fmt,
                entry.regular_market_change_percent.fmt
            );
        }
    }
}

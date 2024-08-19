# Let's create a txt file with the README content.

# Rusty Stocks

Rusty Stocks is a command-line tool built in Rust that allows you to gather and view data on multiple stocks using the Stock Pulse API on RapidAPI. It stores your settings locally, so you don't have to repeatedly input your API keys or remember which stocks you frequently track.

## Features

- **Multiple Stock Tracking**: Query data for multiple stocks at once.
- **Local Configuration Storage**: Save your API keys and frequently checked stocks locally, so you don't need to re-enter them each time.
- **Command-Line Interface**: Simple and efficient interface for stock data retrieval.
- **Flexible Configuration**: Customize which stocks to track and update settings easily.

## Installation

1. **Clone the repository**:

    ```bash
    git clone git@github.com:ClutchReboot/RustyStocks.git
    cd RustyStocks/rusty_stocks
    ```

2. **Build the project**:

    ```bash
    cargo build --release
    ```

3. **Run the tool**:

    ```bash
    ./target/release/rusty_stocks
    ```

## Usage

This tool is used with [Stock Pulse](https://rapidapi.com/manwilbahaa/api/yahoo-finance127)'s endpoint on RapidAPI.
Make sure to sign up and get a free API key.

After running the tool for the first time, you will be prompted to enter your API key for Stock Pulse's API on RapidAPI. You can also specify the stocks you'd like to track.

Basic usage:

```bash
rusty_stocks --help

# All options stored in config file.
rusty_stocks --host rapidapi.com --api-key xxxxx --stocks tsla,aapl

# Can run just the command without options to reuse last settings.
rusty_stocks
```

## Acknowledgements
- Thanks to [RapidAPI](https://rapidapi.com/hub) for easy endpoints
- Thanks to [Stock Pulse](https://rapidapi.com/manwilbahaa/api/yahoo-finance127) specifically to their access to data.
pub mod config;

/// Shorten the import command so 'config' isn't needed.
///
/// Without:
/// ```rust
/// use rusty_stocks::config_management::config::RustyStocksConfig;
/// ```
/// With:
/// ```rust
/// use rusty_stocks::config_management::RustyStocksConfig;
/// ```
pub use config::RustyStocksConfig;

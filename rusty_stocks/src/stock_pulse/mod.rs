pub mod client;
pub mod serializer;

/// Shorten the import command so 'client' isn't needed.
///
/// Without:
/// ```rust
/// use rusty_stocks::stock_pulse::client::StockPulseApi;
/// ```
/// With:
/// ```rust
/// use rusty_stocks::stock_pulse::StockPulseApi;
/// ```
pub use client::StockPulseApi;
pub use serializer::StockPulseResponse;
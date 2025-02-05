pub mod core;
pub mod errors;
pub mod market_data;
pub mod trade_engine;


type Result<T> = std::result::Result<T, errors::Error>;


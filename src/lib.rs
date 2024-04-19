mod models;

pub use models::price_alert::PriceAlert;

#[derive(thiserror::Error, Debug)]
pub enum Error {}

#[derive(Debug, Clone, PartialEq)]
pub enum AlertSet {
    High(f64),
    Low(f64),
}

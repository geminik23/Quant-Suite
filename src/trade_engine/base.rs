use crate::core::Id;
use serde::{Serialize, Deserialize};


pub type TradeId = Id;
pub type OrderId = Id;

#[derive(Debug, Clone, Copy, Default)]
pub enum OrderSide{
    #[default]
    Buy,
    Sell
}

#[derive(Debug, Clone, Copy, Default)]
pub enum OrderType{
    #[default]
    Market,
    Limit,
    Stop,
    StopLimit,
}

#[derive(Debug, Clone, Copy)]
pub enum OrderStatus{
    Pending,
    Filled,
    Canceled,
    PartiallyFilled,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PriceType {
    Trigger,
    Entry,
    Stoploss,
    Target,
    MoveSlToEntry,
    Close,
    // MoveSlTo(f64),
}

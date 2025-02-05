use std::sync::Arc;

use crate::market_data::AlertId;

use super::base::{OrderType, TradeId};



pub trait PriceAlertHandler {
    fn register_price_alert(&self, symbol:&str, alert_id: &AlertId, price: &f64);

    fn cancel_alert(&self, alert_id: &AlertId);
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum TradeState{
    #[default]
    Initial,
    Pending,
    InProgress,
    Finished,
    Cancelled,
}

#[derive(Clone)]
pub struct Trade {
    pub id: TradeId,

    pub order_data:Option<OrderData>,

    pub state: TradeState,

    // change for default
    pub alert_handler: Arc<dyn PriceAlertHandler + Send + Sync>,
}



#[derive(Debug, Clone)]
pub enum Price{
    Absolute(f64),
    Pip(f64),
}


#[derive(Debug, Clone, Default)]
pub struct OrderData{
    pub order_type:OrderType,
    pub symbol:String,
    pub price:Option<Price>,
    pub stoploss:Option<Price>,
    pub targets: Option<Vec<Price>>,
}



#[derive(Debug, Clone)]
pub enum OrderCmd{
    NewOrder{
        order_data:OrderData,
    },

    CloseTrade,
    CancelTrade,

    //ChangeStoploss(Price),
    
}



impl Trade{
    pub fn new(trade_id:Option<TradeId>, alert_handler: Arc<dyn PriceAlertHandler + Send + Sync>)->Self{
        Self{
            id:match trade_id{
                Some(trade_id) => trade_id,
                None => TradeId::new(),
            },
            alert_handler,
            order_data: Default::default(),
            state: Default::default(),
        }

    }

    fn set_state(&mut self, state:TradeState){
        self.state = state;
    }

    pub fn on_alert(&mut self, alert_id:AlertId){
        todo!()
    }

    pub fn tick(&mut self, order_cmd:OrderCmd){
        match order_cmd{
            OrderCmd::NewOrder { order_data } => {
                if self.order_data.is_some(){
                    self.cancel_all_price_alert();
                }
                self.order_data = Some(order_data);
                
                // TODO: set alert for new order_data
                //
                // TODO: set the state (Market or Stop/Limit)
                self.set_state(TradeState::Pending);
            },
            OrderCmd::CloseTrade => {
                self.cancel_all_price_alert();
                
                self.set_state(TradeState::Finished);
            },
            OrderCmd::CancelTrade => {
                self.cancel_all_price_alert();

                self.set_state(TradeState::Cancelled);
            },
            //OrderCmd::ChangeStoploss(price) => {
            //    // TODO: Remove the price alert for current stoploss
            //
            //}


        }

    }

    pub fn cancel_all_price_alert(&mut self){
        // TODO: Cancel current all price alerts
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    struct DummyAlertHandler;

    impl PriceAlertHandler for DummyAlertHandler {
        fn register_price_alert(&self, symbol: &str, alert_id: &AlertId, price: &f64) {
            println!("Registering price alert for symbol: {}, alert_id: {}, price: {}", symbol, alert_id, price);
        }

        fn cancel_alert(&self, alert_id: &AlertId) {
            println!("Cancelling alert with id: {}", alert_id);
        }
    }

    // Test Trade struct
    #[test]
    fn test_initialize_trade() {
        let handler = Arc::new(DummyAlertHandler);

        let trade = Trade::new(None, handler.clone());
        assert_eq!(trade.state, TradeState::Initial);

    }
}

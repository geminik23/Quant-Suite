use super::base::{OrderType, TradeId, OrderId};

#[derive(Debug, Clone, Default)]
pub struct Trade {
    pub id: TradeId,

    pub order_data:Option<OrderData>,
}




#[derive(Debug, Clone, Default)]
pub enum Price{
    #[default]
    Price(f64),
}


#[derive(Debug, Clone, Default)]
pub struct OrderData{
    pub order_type:OrderType,
    pub symbol:String,
    pub price:Option<f64>,
    pub stoploss:Option<f64>,
    pub targets: Option<Vec<f64>>,
}



#[derive(Debug, Clone)]
pub enum OrderCmd{
    NewOrder{
        order_data:OrderData,
    },

    ChangeStoploss(Price),
    
}


impl Trade{
    pub fn new(trade_id:Option<TradeId>)->Self{
        Self{
            id:match trade_id{
                Some(trade_id) => trade_id,
                None => TradeId::new(),
            },
            ..Default::default()
        }

    }

    pub fn tick(&mut self, order_cmd:OrderCmd){
        match order_cmd{
            OrderCmd::NewOrder { order_data } => {
                if self.order_data.is_some(){
                    self.cancel_all_price_alert();
                }
                self.order_data = Some(order_data);
                
                // TODO: set alert for new order_data
            },
            OrderCmd::ChangeStoploss(price) => {
                // TODO: Remove the price alert for current stoploss
                
                
                
            }


        }

    }

    pub fn cancel_all_price_alert(&mut self){
        // TODO: Cancel current all price alerts
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    // Test Trade struct
    #[test]
    fn test_trade() {
        //let trade = Trade {
        //    id: TradeId::new(),
        //    metadata: HashMap::new(),
        //    symbol: "BTCUSD".to_string(),
        //    price_alerts: HashMap::new(),
        //    quantity: 0.0,
        //    price: 0.0,
        //};

        //assert_eq!(trade.symbol, "BTCUSD");
    }
}

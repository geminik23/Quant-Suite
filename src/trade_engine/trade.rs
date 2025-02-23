use std::{collections::HashMap, sync::Arc};

use crate::market_data::AlertId;

use super::base::{OrderSide, OrderType, TradeId};

pub trait PriceAlertHandler {
    fn register_price_alert(&self, symbol: &str, alert_id: AlertId, price: f64);

    fn cancel_alert(&self, alert_id: &AlertId);
}

#[derive(Debug, Clone)]
enum AlertType {
    StopTrigger,
    LimitTrigger,
    Target,
    Stoploss,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Copy)]
pub enum TradeState {
    #[default]
    Initial,
    Pending,
    InProgress,
    Finished,
    Cancelled,
}

#[derive(Clone)]
pub struct Trade {
    id: TradeId,

    order_data: Option<OrderData>,

    state: TradeState,

    // change for default
    alert_handler: Arc<dyn PriceAlertHandler + Send + Sync>,

    alerts: HashMap<AlertId, AlertType>,
}

#[derive(Debug, Clone)]
pub enum Price {
    Abs(f64),
    Pip(f64),
}

#[derive(Debug, Clone, Default)]
pub struct OrderData {
    pub order_type: OrderType,
    pub symbol: String,
    pub side: OrderSide,
    pub price: Option<Price>,
    pub stoploss: Option<Price>,
    pub targets: Option<Vec<Price>>,
}

impl OrderData {
    pub fn get_entry(&self) -> Option<f64> {
        // TODO: Replace with actual implementation later
        self.price.as_ref().map(|v| match v {
            Price::Abs(v) => *v,
            Price::Pip(_) => 0.0,
        })
    }

    pub fn get_stoploss(&self) -> Option<f64> {
        // TODO: Replace with actual implementation later
        self.stoploss.as_ref().map(|v| match v {
            Price::Abs(v) => *v,
            Price::Pip(_) => 0.0,
        })
    }

    pub fn get_targets(&self) -> Option<Vec<f64>> {
        // TODO: Replace with actual implementation later
        self.targets.as_ref().map(|v| {
            v.iter()
                .map(|v| match v {
                    Price::Abs(v) => *v,
                    Price::Pip(_) => 0.0,
                })
                .collect::<Vec<_>>()
        })
    }
}

#[derive(Debug, Clone)]
pub enum OrderCmd {
    NewOrder { order_data: OrderData },

    CloseTrade,
    CancelTrade,
    //ChangeStoploss(Price),
}

impl Trade {
    pub fn new(
        trade_id: Option<TradeId>,
        alert_handler: Arc<dyn PriceAlertHandler + Send + Sync>,
    ) -> Self {
        Self {
            id: match trade_id {
                Some(trade_id) => trade_id,
                None => TradeId::new(),
            },
            alert_handler,
            order_data: Default::default(),
            state: Default::default(),
            alerts: HashMap::new(),
        }
    }

    pub fn trade_id(&self) -> TradeId {
        self.id.clone()
    }

    fn set_state(&mut self, state: TradeState) {
        self.state = state;
    }

    pub fn state(&self) -> TradeState {
        self.state
    }

    pub fn on_alert(&mut self, alert_id: AlertId) {
        let Some(alert_type) = self.alerts.remove(&alert_id) else {
            return;
        };

        if matches!(alert_type, AlertType::Stoploss) {
            self.set_state(TradeState::Finished);
        }
    }

    pub fn tick(&mut self, order_cmd: OrderCmd) {
        match order_cmd {
            OrderCmd::NewOrder { order_data } => {
                if self.order_data.is_some() {
                    self.cancel_all_price_alert();
                }

                let mut reserved_alerts = Vec::new();

                // Check if market or pending
                match order_data.order_type {
                    OrderType::Market => {
                        // set the stoploss and target alerts
                        if let Some(stoploss) = order_data.get_stoploss() {
                            reserved_alerts.push((AlertType::Stoploss, AlertId::new(), stoploss));
                        }

                        if let Some(targets) = order_data.get_targets() {
                            targets.into_iter().for_each(|price| {
                                reserved_alerts.push((AlertType::Target, AlertId::new(), price));
                            });
                        }
                    }
                    otype => {
                        if let Some(entry_price) = order_data.get_entry() {
                            reserved_alerts.push((
                                match otype {
                                    OrderType::Limit => AlertType::LimitTrigger,
                                    OrderType::Stop => AlertType::StopTrigger,
                                    OrderType::StopLimit => AlertType::StopTrigger,
                                    _ => unreachable!("'Market' OrderType is not reachable"),
                                },
                                AlertId::new(),
                                entry_price,
                            ));
                        }
                    }
                }

                self.order_data = Some(order_data);

                // TODO: Execute the order if order type is Market

                let (symbol, is_buy) = {
                    let data = self.order_data.as_ref().unwrap();
                    (data.symbol.as_str(), matches!(data.side, OrderSide::Buy))
                };

                for (alert_type, alert_id, price) in reserved_alerts {
                    // TODO: AlertType and is_buy will be used later.
                    self.alerts.insert(alert_id.clone(), alert_type);
                    self.alert_handler
                        .register_price_alert(symbol, alert_id, price);
                }

                // Set the state
                self.set_state(TradeState::Pending);
            }
            OrderCmd::CloseTrade => {
                self.cancel_all_price_alert();

                self.set_state(TradeState::Finished);
            }
            OrderCmd::CancelTrade => {
                self.cancel_all_price_alert();

                self.set_state(TradeState::Cancelled);
            } //OrderCmd::ChangeStoploss(price) => {
              // TODO: Remove the price alert for current stoploss
              //
              //}
        }
    }

    pub fn cancel_all_price_alert(&mut self) {
        // TODO: Cancel current all price alerts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STOPLOSS_PRICE: f64 = 0.7991;

    pub struct DummyAlertHandler;

    impl PriceAlertHandler for DummyAlertHandler {
        fn register_price_alert(&self, symbol: &str, alert_id: AlertId, price: f64) {
            println!(
                "Registering price alert for symbol: {}, alert_id: {}, price: {}",
                symbol, alert_id, price
            );
        }

        fn cancel_alert(&self, alert_id: &AlertId) {
            println!("Cancelling alert with id: {}", alert_id);
        }
    }

    fn market_order(handler: Arc<DummyAlertHandler>) -> Trade {
        let mut trade = Trade::new(None, handler.clone());

        // New market order
        trade.tick(OrderCmd::NewOrder {
            order_data: OrderData {
                order_type: OrderType::Market,
                symbol: "EURUSD".into(),
                side: OrderSide::Buy,
                price: Some(Price::Abs(0.8011)),
                stoploss: Some(Price::Abs(STOPLOSS_PRICE)),
                targets: Some(vec![Price::Abs(0.8021)]),
            },
        });

        trade
    }

    // Test Trade struct
    #[test]
    fn test_initialize_trade() {
        let handler = Arc::new(DummyAlertHandler);

        let trade = Trade::new(None, handler.clone());

        // TradeID
        let trade_id = trade.trade_id();
        println!("Trade({}), state is {:?}", trade_id.0, trade.state());
    }

    #[test]
    fn test_market_order_hit_stoploss() {
        let handler = Arc::new(DummyAlertHandler);

        let mut trade = market_order(handler.clone());
        // TradeID
        let trade_id = trade.trade_id();
        println!("Trade({}), state is {:?}", trade_id.0, trade.state());
        assert_eq!(trade.state(), TradeState::Pending);

        let alert_id = trade
            .alerts
            .iter()
            .find(|(_, alert_type)| matches!(alert_type, AlertType::Stoploss))
            .map(|(id, _)| id.clone())
            .unwrap_or_else(|| panic!("Stoploss alert not found!"));

        // TODO: fixe this later
        //assert_eq!(trade.state(), TradeState::InProgress);

        // Hit Stoploss
        trade.on_alert(alert_id);

        // check state
        assert_eq!(trade.state(), TradeState::Finished);
    }

    #[test]
    fn test_market_order_hit_target() {}
}

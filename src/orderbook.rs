use std::collections::BTreeMap;
use crate::order::{Side, Order};

#[derive(Debug)]
pub struct OrderBook {

    // Price => Orders at that price
    pub bids: BTreeMap<i64, Vec<Order>>,
    pub asks: BTreeMap<i64, Vec<Order>>,
}

impl OrderBook {

    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        match &order.side {
            Side::Buy => {
                self.bids
                    .entry(order.price)
                    .or_insert_with(Vec::new)
                    .push(order);
            }

            Side::Sell => {
                self.asks
                    .entry(order.price)
                    .or_insert_with(Vec::new)
                    .push(order);
            }
        }
    }

}
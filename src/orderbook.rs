use std::collections::BTreeMap;
use crate::order::{Buy, Sell};

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

}